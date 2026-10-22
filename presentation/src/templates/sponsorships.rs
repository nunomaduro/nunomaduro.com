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
            title: "nunomaduro · sponsorships".to_string(),
            description: "sponsor nunomaduro: a monthly logo and link, ads in youtube videos, a dedicated video, or a two-day live build sprint with your developer tool."
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
            ("$16,000", "https://buy.stripe.com/5kQ7sM15Se1Qdgu0R00Jq00"),
            ("$24,000", "https://buy.stripe.com/fZu00k01O7DsgsG57g0Jq08"),
            ("$36,000", "https://buy.stripe.com/7sYfZi6qc4rg7WaarA0Jq09"),
        ] {
            let card = format!("<a href=\"{link}\" class=\"group");
            let start = html.find(&card).unwrap_or_else(|| panic!("{link}"));
            let end = start + html[start..].find("</a>").unwrap();

            assert!(html[start..end].contains(price), "{price}");
        }
        assert!(html.contains("most popular"));
        assert!(html.contains("highest impact"));
    }

    #[test]
    fn writes_nunomaduro_as_one_word() {
        let html = SponsorshipsTemplate::default().render().unwrap();

        assert!(html.contains("<title>nunomaduro · sponsorships</title>"));
        assert!(!html.to_lowercase().contains("nuno maduro"));
    }
}
