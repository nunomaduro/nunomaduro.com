use askama::Template;

pub struct MediaKitPage {
    id: String,
    title: String,
    description: String,
}

impl MediaKitPage {
    pub fn new() -> Self {
        Self {
            id: "media-kit".to_string(),
            title: "nunomaduro · Media kit".to_string(),
            description: "The media kit of nunomaduro: audience, channel performance, sponsorship packages, and the results of past campaigns with JetBrains and more."
                .to_string(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Template)]
#[template(path = "media_kit.html")]
pub struct MediaKitTemplate {
    page: MediaKitPage,
    url: String,
}

impl Default for MediaKitTemplate {
    fn default() -> Self {
        Self {
            page: MediaKitPage::new(),
            url: "/media-kit".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_each_section_of_the_media_kit_with_a_link_to_it() {
        let html = MediaKitTemplate::default().render().unwrap();

        for section in ["audience", "packages", "case-studies"] {
            assert!(
                html.contains(&format!("<a href=\"#{section}\"")),
                "{section}"
            );
            assert!(
                html.contains(&format!("<section id=\"{section}\"")),
                "{section}"
            );
        }
    }

    #[test]
    fn leaves_the_overview_of_the_about_page_out_of_the_media_kit() {
        let html = MediaKitTemplate::default().render().unwrap();

        assert!(!html.contains("id=\"overview\""));
        assert!(!html.contains("2.22B+"));
        assert!(!html.contains("153K+"));
    }

    #[test]
    fn renders_each_top_country_with_its_share_of_views() {
        let html = MediaKitTemplate::default().render().unwrap();

        for (country, share, width) in [
            ("United States", "12.31%", "100%"),
            ("Germany", "7.33%", "60%"),
            ("India", "5.97%", "48%"),
            ("Brazil", "5.40%", "44%"),
            ("United Kingdom", "4.94%", "40%"),
        ] {
            let start = html
                .find(&format!(">{country}<"))
                .unwrap_or_else(|| panic!("{country}"));
            let end = start + html[start..].find("</li>").unwrap();

            assert!(html[start..end].contains(share), "{share}");
            assert!(
                html[start..end].contains(&format!("width: {width}")),
                "{width}"
            );
        }
    }

    #[test]
    fn links_each_sponsorship_package_to_its_details_on_the_sponsorships_page() {
        let html = MediaKitTemplate::default().render().unwrap();

        for (anchor, price, package) in [
            (
                "recurring-logo-and-link",
                "$1,000",
                "Recurring logo &amp; link",
            ),
            ("ad-in-4-videos", "$8,000", "20-sec ad in 4 videos"),
            ("dedicated-video", "$16,000", "Dedicated video"),
            (
                "studio-live-build-sprint",
                "$24,000",
                "Studio live build sprint",
            ),
            (
                "on-site-live-build-sprint",
                "$36,000",
                "On-site live build sprint",
            ),
        ] {
            let card = format!("<a href=\"/sponsorships#{anchor}\" class=\"group card");
            let start = html.find(&card).unwrap_or_else(|| panic!("{anchor}"));
            let end = start + html[start..].find("</a>").unwrap();

            assert!(html[start..end].contains(price), "{price}");
            assert!(html[start..end].contains(package), "{package}");
        }
        assert!(html.contains("Most popular"));
        assert!(html.contains("Highest impact"));
        assert!(!html.contains("bg-zinc-900 p-6"));
    }

    #[test]
    fn renders_each_photo_from_the_public_directory_with_a_description() {
        let html = MediaKitTemplate::default().render().unwrap();
        let photos: Vec<&str> = html.split("<img src=\"/").skip(1).collect();

        assert_eq!(photos.len(), 11);
        for photo in photos {
            let tag = &photo[..photo.find('>').unwrap()];
            let path = &tag[..tag.find('"').unwrap()];

            assert!(
                std::path::Path::new("../public").join(path).exists(),
                "{path}"
            );
            assert!(
                tag.contains("alt=\"") && !tag.contains("alt=\"\""),
                "{path}"
            );
            assert!(tag.contains("loading=\"lazy\""), "{path}");
            assert!(tag.contains("rounded-2xl"), "{path}");
        }
    }

    #[test]
    fn mentions_no_former_partner_bento() {
        let html = MediaKitTemplate::default().render().unwrap();

        assert!(!html.to_lowercase().contains("bento"));
    }

    #[test]
    fn sends_each_sponsor_to_the_sponsorships_page_before_payment() {
        let html = MediaKitTemplate::default().render().unwrap();

        assert!(html.contains("href=\"/sponsorships\""));
        assert!(html.contains("href=\"mailto:enunomaduro@gmail.com\""));
        assert!(!html.contains("buy.stripe.com"));
    }

    #[test]
    fn marks_the_media_kit_as_the_current_page_in_the_navigation() {
        let html = MediaKitTemplate::default().render().unwrap();

        assert_eq!(
            html.matches("href=\"/media-kit\" aria-current=\"page\"")
                .count(),
            2
        );
    }

    #[test]
    fn writes_nunomaduro_as_one_word() {
        let html = MediaKitTemplate::default().render().unwrap();

        assert!(html.contains("<title>nunomaduro · Media kit</title>"));
        assert!(!html.to_lowercase().contains("nuno maduro"));
    }

    #[test]
    fn describes_the_media_kit_page_with_a_breadcrumb() {
        let html = MediaKitTemplate::default().render().unwrap();
        let graph = crate::templates::structured_data(&html);
        let page = crate::templates::node(&graph, "WebPage");

        assert!(
            html.contains("<link rel=\"canonical\" href=\"https://nunomaduro.com/media-kit\">")
        );
        assert_eq!(
            page["breadcrumb"]["itemListElement"][1]["name"],
            "Media kit"
        );
        assert_eq!(
            page["breadcrumb"]["itemListElement"][1]["item"],
            "https://nunomaduro.com/media-kit"
        );
    }
}
