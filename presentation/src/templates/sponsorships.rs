use askama::Template;

pub struct SponsorshipsPage {
    id: String,
    title: String,
    description: String,
}

impl SponsorshipsPage {
    pub fn new() -> Self {
        Self {
            id: "sponsorships".to_string(),
            title: "nunomaduro · Sponsorships".to_string(),
            description: "Sponsor nunomaduro: the audience, five packages from a monthly logo to a two-day live build sprint, and the results of past campaigns."
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
#[template(path = "support.html")]
pub struct SponsorshipsTemplate {
    page: SponsorshipsPage,
    url: String,
}

impl Default for SponsorshipsTemplate {
    fn default() -> Self {
        Self {
            page: SponsorshipsPage::new(),
            url: "/sponsorships".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_each_sponsorship_package_with_its_stripe_link() {
        let html = SponsorshipsTemplate::default().render().unwrap();

        for (price, link) in [
            ("$1,000", "https://buy.stripe.com/28E28scOAcXMb8m9nw0Jq01"),
            ("$8,000", "https://buy.stripe.com/3cI6oI4i49LA3FUczI0Jq03"),
            ("$12,000", "https://buy.stripe.com/fZu6oI8ykaPE6S67fo0Jq0a"),
            ("$24,000", "https://buy.stripe.com/fZu00k01O7DsgsG57g0Jq08"),
            ("$36,000", "https://buy.stripe.com/7sYfZi6qc4rg7WaarA0Jq09"),
        ] {
            let card = format!("<a href=\"{link}\" class=\"group card ");
            let start = html.find(&card).unwrap_or_else(|| panic!("{link}"));
            let end = start + html[start..].find("</a>").unwrap();

            assert!(html[start..end].contains(price), "{price}");
        }
        assert!(html.contains("Most popular"));
        assert!(html.contains("Highest impact"));
    }

    #[test]
    fn gives_the_details_of_each_package_an_anchor() {
        let html = SponsorshipsTemplate::default().render().unwrap();

        for (anchor, package) in [
            (
                "recurring-logo-and-link",
                "Recurring monthly logo + top link",
            ),
            ("ad-in-4-videos", "20-second ad in 4 videos"),
            ("dedicated-video", "Dedicated video"),
            ("studio-live-build-sprint", "Studio live build sprint"),
            ("on-site-live-build-sprint", "On-site live build sprint"),
        ] {
            let start = html
                .find(&format!("<div id=\"{anchor}\""))
                .unwrap_or_else(|| panic!("{anchor}"));
            let end = start + html[start..].find("</dt>").unwrap();

            assert!(html[start..end].contains(package), "{package}");
        }
    }

    #[test]
    fn collapses_the_options_in_detail_and_the_terms() {
        let html = SponsorshipsTemplate::default().render().unwrap();

        for heading in [">The options in detail<", ">Terms<"] {
            let start = html.find(heading).unwrap_or_else(|| panic!("{heading}"));
            let details = html[..start].rfind("<details").unwrap();

            assert!(!html[details..start].contains("</details>"), "{heading}");
            assert!(html[details..start].contains("<summary"), "{heading}");
            assert!(!html[details..start].contains(" open"), "{heading}");
        }
    }

    #[test]
    fn renders_each_section_of_the_page_with_a_link_to_it() {
        let html = SponsorshipsTemplate::default().render().unwrap();

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
    fn puts_the_packages_with_their_terms_then_the_audience_then_the_case_studies() {
        let html = SponsorshipsTemplate::default().render().unwrap();
        let position = |section: &str| html.find(&format!("<section id=\"{section}\"")).unwrap();

        assert!(html.find("Most popular").unwrap() > position("packages"));
        assert!(html.find("Most popular").unwrap() < position("audience"));
        assert!(position("packages") < html.find(">Terms<").unwrap());
        assert!(html.find(">Terms<").unwrap() < html.find("For individuals:").unwrap());
        assert!(html.find("For individuals:").unwrap() < position("audience"));
        assert!(position("audience") < position("case-studies"));
    }

    #[test]
    fn links_to_the_packages_then_the_audience_then_the_case_studies_on_top_of_the_page() {
        let html = SponsorshipsTemplate::default().render().unwrap();
        let start = html.find("<nav aria-label=\"On this page\"").unwrap();
        let end = start + html[start..].find("</nav>").unwrap();
        let navigation = &html[start..end];
        let position = |section: &str| {
            navigation
                .find(&format!("<a href=\"#{section}\""))
                .unwrap_or_else(|| panic!("{section}"))
        };

        assert!(position("packages") < position("audience"));
        assert!(position("audience") < position("case-studies"));
        assert!(html[..start].contains("Pick a package, see who watches"));
    }

    #[test]
    fn asks_for_confirmation_once_and_leaves_out_the_removed_notes() {
        let html = SponsorshipsTemplate::default().render().unwrap();

        assert_eq!(
            html.matches("require confirmation before purchase").count(),
            1
        );
        assert!(!html.contains("Before you pay"));
        assert!(!html.contains("Exclusivity:"));
        assert!(!html.contains("Where the channel is heading"));
        assert!(!html.contains("Average view duration<"));
        assert!(!html.contains("Impressions per video"));
    }

    #[test]
    fn leaves_the_overview_of_the_about_page_out() {
        let html = SponsorshipsTemplate::default().render().unwrap();

        assert!(!html.contains("id=\"overview\""));
        assert!(!html.contains("2.22B+"));
        assert!(!html.contains("153K+"));
    }

    #[test]
    fn renders_each_top_country_with_its_share_of_views() {
        let html = SponsorshipsTemplate::default().render().unwrap();

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
    fn renders_the_top_10_videos_by_views_below_the_last_90_days() {
        let html = SponsorshipsTemplate::default().render().unwrap();
        let start = html.find("Top 10 most-watched videos").unwrap();
        let end = start + html[start..].find("</ol>").unwrap();
        let items: Vec<&str> = html[start..end].split("<li>").skip(1).collect();

        assert!(html.find("YouTube · last 90 days").unwrap() < start);
        assert!(start < html.find("Who watches").unwrap());
        assert_eq!(items.len(), 10);
        let mut previous_views = u32::MAX;
        for item in items {
            let video = &item[item.find("watch?v=").unwrap() + 8..];
            let video = &video[..video.find('"').unwrap()];
            let views = &item[..item.find(" views</span>").unwrap()];
            let views: u32 = views[views.rfind('>').unwrap() + 1..]
                .replace(',', "")
                .parse()
                .unwrap();

            assert!(item.contains(&format!("/top-video.{video}.jpg")), "{video}");
            assert!(views <= previous_views, "{video}");
            previous_views = views;
        }
    }

    #[test]
    fn lists_the_followers_of_each_main_platform_between_the_top_10_and_the_audience() {
        let html = SponsorshipsTemplate::default().render().unwrap();
        let start = html.find("Followers by platform").unwrap();
        let end = html.find("Who watches").unwrap();

        assert!(html.find("Top 10 most-watched videos").unwrap() < start);
        assert_eq!(html[start..end].matches("data-count>").count(), 7);
        for platform in [
            "X",
            "YouTube",
            "LinkedIn",
            "GitHub",
            "Instagram",
            "TikTok",
            "Twitch",
        ] {
            assert!(
                html[start..end].contains(&format!("</span>{platform}</a>")),
                "{platform}"
            );
        }
        assert!(!html[start..end].contains("Threads"));
        assert_eq!(
            include_str!("../../templates/support.html")
                .matches("href=\"https://twitter.com/enunomaduro\"")
                .count(),
            1
        );
    }

    #[test]
    fn renders_each_photo_from_the_public_directory_with_a_description() {
        let html = SponsorshipsTemplate::default().render().unwrap();
        let photos: Vec<&str> = html.split("<img src=\"/").skip(1).collect();

        assert_eq!(photos.len(), 21);
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
    fn renders_the_live_results_of_each_day_of_the_slack_sprint_in_a_table() {
        let html = SponsorshipsTemplate::default().render().unwrap();
        let start = html.find("id=\"slack-sprint\"").unwrap();
        let end = start + html[start..].find("</table>").unwrap();
        let table = &html[start + html[start..].find("<table").unwrap()..end];

        assert!(table.contains("watch?v=zq4vexfE2zs\" class=\"link\">Day 1<"));
        assert!(table.contains("watch?v=gPHN3wHdP8Q\" class=\"link\">Day 2<"));
        for (metric, first_day, second_day) in [
            ("YouTube views", "14,400", "9,582"),
            ("YouTube views while live", "8,455", "6,612"),
            ("YouTube peak concurrent", "384", "268"),
            ("YouTube average concurrent", "261", "204"),
            ("YouTube chat messages", "3.9K+", "~3K"),
            ("YouTube likes", "545", "368"),
            ("Click-through rate", "8.44%", "7.44%"),
        ] {
            let row_start = table
                .find(&format!(">{metric}</th>"))
                .unwrap_or_else(|| panic!("{metric}"));
            let row = &table[row_start..row_start + table[row_start..].find("</tr>").unwrap()];

            assert!(row.contains(&format!(">{first_day}</td>")), "{metric}");
            assert!(row.contains(&format!(">{second_day}</td>")), "{metric}");
        }
    }

    #[test]
    fn mentions_no_former_partner_bento() {
        let html = SponsorshipsTemplate::default().render().unwrap();

        assert!(!html.to_lowercase().contains("bento"));
    }

    #[test]
    fn links_to_no_media_kit_page() {
        let html = SponsorshipsTemplate::default().render().unwrap();

        assert!(!html.contains("href=\"/media-kit\""));
        assert!(!html.contains(">Media kit<"));
    }

    #[test]
    fn marks_the_sponsorships_as_the_current_page_in_the_navigation() {
        let html = SponsorshipsTemplate::default().render().unwrap();

        assert_eq!(
            html.matches("href=\"/sponsorships\" aria-current=\"page\"")
                .count(),
            2
        );
    }

    #[test]
    fn describes_the_sponsorships_page_with_a_breadcrumb() {
        let html = SponsorshipsTemplate::default().render().unwrap();
        let graph = crate::templates::structured_data(&html);
        let page = crate::templates::node(&graph, "WebPage");

        assert!(
            html.contains("<link rel=\"canonical\" href=\"https://nunomaduro.com/sponsorships\">")
        );
        assert_eq!(
            page["breadcrumb"]["itemListElement"][1]["name"],
            "Sponsorships"
        );
    }

    #[test]
    fn writes_nunomaduro_as_one_word() {
        let html = SponsorshipsTemplate::default().render().unwrap();

        assert!(html.contains("<title>nunomaduro · Sponsorships</title>"));
        assert!(!html.to_lowercase().contains("nuno maduro"));
    }
}
