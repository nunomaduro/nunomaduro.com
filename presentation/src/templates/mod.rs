mod about;
mod feed;
mod not_found;
mod post;
mod posts;
mod sitemap;
mod sponsorships;
mod talks;

pub use about::AboutTemplate;
pub use feed::FeedTemplate;
pub use not_found::NotFoundTemplate;
pub use post::PostTemplate;
pub use posts::PostsTemplate;
pub use sitemap::SitemapTemplate;
pub use sponsorships::SponsorshipsTemplate;
pub use talks::TalksTemplate;

#[cfg(test)]
pub(crate) fn structured_data(html: &str) -> serde_json::Value {
    let start = html.find("<script type=\"application/ld+json\">").unwrap() + 35;
    let end = start + html[start..].find("</script>").unwrap();

    serde_json::from_str(&html[start..end]).unwrap()
}

#[cfg(test)]
pub(crate) fn node<'a>(graph: &'a serde_json::Value, kind: &str) -> &'a serde_json::Value {
    graph["@graph"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["@type"] == kind)
        .unwrap_or_else(|| panic!("{kind}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use askama::Template;

    fn meta(html: &str, attribute: &str, name: &str) -> String {
        let start = html.find(&format!("<meta {attribute}=\"{name}\" content=\"")).unwrap_or_else(|| panic!("{name}"));
        let content = &html[start..];
        let content = &content[content.find("content=\"").unwrap() + 9..];

        content[..content.find('"').unwrap()].to_string()
    }

    #[test]
    fn gives_each_page_its_own_title_description_and_social_card() {
        let pages = [
            AboutTemplate::new(Vec::new()).render().unwrap(),
            PostsTemplate::new(Vec::new()).render().unwrap(),
            TalksTemplate::default().render().unwrap(),
            SponsorshipsTemplate::default().render().unwrap(),
        ];

        let mut titles: Vec<String> = pages.iter().map(|html| meta(html, "property", "og:title")).collect();
        let mut descriptions: Vec<String> = pages.iter().map(|html| meta(html, "name", "description")).collect();
        let mut cards: Vec<String> = pages.iter().map(|html| meta(html, "property", "og:image")).collect();

        for description in &descriptions {
            assert!((50..=160).contains(&description.chars().count()), "{description}");
        }
        for card in &cards {
            let path = card.replace("https://nunomaduro.com/", "../public/");
            assert!(std::path::Path::new(&path).exists(), "{path}");
        }

        for list in [&mut titles, &mut descriptions, &mut cards] {
            list.sort();
            list.dedup();
            assert_eq!(list.len(), 4, "{list:?}");
        }
    }
}
