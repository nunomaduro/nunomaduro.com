use askama::Template;

pub struct PartnersPage {
    id: String,
    title: String,
    description: String,
}

impl PartnersPage {
    pub fn new() -> Self {
        Self {
            id: "partners".to_string(),
            title: "nunomaduro · Partners".to_string(),
            description: "The companies that support my YouTube channel.".to_string(),
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

pub struct Partner {
    name: &'static str,
    description: &'static str,
    url: &'static str,
    logo: &'static str,
}

impl Partner {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn description(&self) -> &str {
        self.description
    }

    pub fn url(&self) -> &str {
        self.url
    }

    pub fn logo(&self) -> &str {
        self.logo
    }
}

#[derive(Clone, Copy)]
pub struct Month {
    year: u32,
    month: u32,
}

impl Month {
    fn current() -> Self {
        let seconds = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("the clock of the system is after 1970")
            .as_secs();
        let days = seconds / 86_400 + 719_468;
        let era = days / 146_097;
        let day_of_era = days % 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let shifted_month = (5 * day_of_year + 2) / 153;
        let month = if shifted_month < 10 {
            shifted_month + 3
        } else {
            shifted_month - 9
        };
        let year = year_of_era + era * 400 + u64::from(month <= 2);

        Self {
            year: u32::try_from(year).expect("the year fits in a u32"),
            month: u32::try_from(month).expect("the month fits in a u32"),
        }
    }

    fn months_since_year_zero(self) -> u32 {
        self.year * 12 + self.month - 1
    }
}

pub struct Sponsorship {
    partner: Partner,
    since: Month,
}

impl Sponsorship {
    pub fn partner(&self) -> &Partner {
        &self.partner
    }

    pub fn full_years_until(&self, today: &Month) -> u32 {
        today
            .months_since_year_zero()
            .saturating_sub(self.since.months_since_year_zero())
            / 12
    }
}

pub(crate) const CURRENT_SPONSORS: [Sponsorship; 7] = [
    Sponsorship {
        partner: Partner {
            name: "JetBrains",
            description: "Code with PhpStorm for world-class PHP development.",
            url: "https://jb.gg/nuno",
            logo: "/partners.jetbrains.png",
        },
        since: Month {
            year: 2024,
            month: 11,
        },
    },
    Sponsorship {
        partner: Partner {
            name: "CodeRabbit",
            description: "AI code reviews with instant feedback on pull requests.",
            url: "https://coderabbit.link/nunomaduro",
            logo: "/partners.coderabbit.png",
        },
        since: Month {
            year: 2025,
            month: 8,
        },
    },
    Sponsorship {
        partner: Partner {
            name: "Redberry",
            description: "Work with the best Laravel digital agency.",
            url: "https://redberry.international/?ref=nunomaduro",
            logo: "/partners.redberry.svg",
        },
        since: Month {
            year: 2025,
            month: 9,
        },
    },
    Sponsorship {
        partner: Partner {
            name: "SerpApi",
            description: "Instantly access real-time search data from Google, Amazon, and more.",
            url: "https://serpapi.com/?ref=nunomaduro",
            logo: "/partners.serpapi.svg",
        },
        since: Month {
            year: 2026,
            month: 2,
        },
    },
    Sponsorship {
        partner: Partner {
            name: "Typesense",
            description: "Cutting-edge, in-memory search engine for mere mortals.",
            url: "https://typesense.org/?ref=nunomaduro",
            logo: "/partners.typesense.png",
        },
        since: Month {
            year: 2026,
            month: 7,
        },
    },
    Sponsorship {
        partner: Partner {
            name: "NativePHP",
            description: "Build real native mobile apps with Laravel.",
            url: "https://nativephp.com/?ref=nunomaduro",
            logo: "/partners.nativephp.svg",
        },
        since: Month {
            year: 2026,
            month: 8,
        },
    },
    Sponsorship {
        partner: Partner {
            name: "Unstoppable",
            description: "The IDE for your unstoppable agent factory.",
            url: "https://app.unstoppable.ai/code",
            logo: "/partners.unstoppable.png",
        },
        since: Month {
            year: 2026,
            month: 10,
        },
    },
];

const PAST_PARTNERS: [Partner; 7] = [
    Partner {
        name: "Bento",
        description: "Campaigns, automations, and transactional email on one platform.",
        url: "https://bentonow.com",
        logo: "/partners.bento.png",
    },
    Partner {
        name: "Devin",
        description: "The AI coding agent and software engineer.",
        url: "https://devin.ai",
        logo: "/partners.devin.png",
    },
    Partner {
        name: "Laracasts",
        description: "The best way to learn Laravel and PHP.",
        url: "https://laracasts.com",
        logo: "/partners.laracasts.png",
    },
    Partner {
        name: "Laravel Cloud",
        description: "Deploy, run, and scale Laravel applications.",
        url: "https://laravel.com/cloud",
        logo: "/partners.laravel-cloud.png",
    },
    Partner {
        name: "Mailtrap",
        description: "Send, receive, and test email via API or SMTP.",
        url: "https://mailtrap.io",
        logo: "/partners.mailtrap.png",
    },
    Partner {
        name: "Tighten",
        description: "A software development firm specializing in Laravel, Livewire, Vue.js, and React.",
        url: "https://tighten.com",
        logo: "/partners.tighten.png",
    },
    Partner {
        name: "XGeeks",
        description: "Enterprise software with AI and cloud native development.",
        url: "https://xgeeks.com",
        logo: "/partners.xgeeks.svg",
    },
];

#[derive(Template)]
#[template(path = "partners.html")]
pub struct PartnersTemplate {
    page: PartnersPage,
    url: String,
    today: Month,
    current_sponsors: &'static [Sponsorship],
    past_partners: &'static [Partner],
}

impl PartnersTemplate {
    fn in_month(today: Month) -> Self {
        Self {
            page: PartnersPage::new(),
            url: "/partners".to_string(),
            today,
            current_sponsors: &CURRENT_SPONSORS,
            past_partners: &PAST_PARTNERS,
        }
    }
}

impl Default for PartnersTemplate {
    fn default() -> Self {
        Self::in_month(Month::current())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_each_current_sponsor_as_a_card_with_its_logo_name_description_and_link() {
        let html = PartnersTemplate::default().render().unwrap();
        let start = html.find("<section id=\"current-sponsors\"").unwrap();
        let end = html.find("<section id=\"part-of-the-journey\"").unwrap();
        let section = &html[start..end];

        assert_eq!(section.matches("<a href=").count(), 7);
        for sponsorship in CURRENT_SPONSORS {
            let partner = sponsorship.partner();
            let card_start = section
                .find(&format!("<a href=\"{}\"", partner.url()))
                .unwrap_or_else(|| panic!("{}", partner.name()));
            let card =
                &section[card_start..card_start + section[card_start..].find("</a>").unwrap()];

            assert!(
                card.contains(&format!("src=\"{}\"", partner.logo())),
                "{}",
                partner.name()
            );
            assert!(
                card.contains(&format!(">{}</h3>", partner.name())),
                "{}",
                partner.name()
            );
            assert!(card.contains(partner.description()), "{}", partner.name());
        }
    }

    #[test]
    fn renders_each_past_sponsor_in_the_part_of_the_journey() {
        let html = PartnersTemplate::default().render().unwrap();
        let start = html.find("<section id=\"part-of-the-journey\"").unwrap();
        let section = &html[start..start + html[start..].find("</section>").unwrap()];

        assert_eq!(section.matches("<a href=").count(), 7);
        for partner in PAST_PARTNERS {
            let card_start = section
                .find(&format!("<a href=\"{}\"", partner.url()))
                .unwrap_or_else(|| panic!("{}", partner.name()));
            let card =
                &section[card_start..card_start + section[card_start..].find("</a>").unwrap()];

            assert!(
                card.contains(&format!("src=\"{}\"", partner.logo())),
                "{}",
                partner.name()
            );
            assert!(
                card.contains(&format!(">{}</h3>", partner.name())),
                "{}",
                partner.name()
            );
        }
    }

    #[test]
    fn lists_the_current_sponsors_in_the_order_that_they_joined() {
        let html = PartnersTemplate::default().render().unwrap();
        let position = |name: &str| html.find(&format!(">{name}</h3>")).unwrap();

        for pair in [
            "JetBrains",
            "CodeRabbit",
            "Redberry",
            "SerpApi",
            "Typesense",
            "NativePHP",
            "Unstoppable",
        ]
        .windows(2)
        {
            assert!(position(pair[0]) < position(pair[1]), "{}", pair[1]);
        }
    }

    #[test]
    fn marks_each_sponsor_of_one_year_or_more_with_its_full_years() {
        let html = PartnersTemplate::in_month(Month {
            year: 2026,
            month: 11,
        })
        .render()
        .unwrap();
        let card = |name: &str| {
            let end = html.find(&format!(">{name}</h3>")).unwrap();
            let start = html[..end].rfind("<a href=").unwrap();
            &html[start..end]
        };

        assert!(card("JetBrains").contains(">Sponsor for 2 years<"));
        assert!(card("CodeRabbit").contains(">Sponsor for 1 year<"));
        assert!(card("Redberry").contains(">Sponsor for 1 year<"));
        for name in ["SerpApi", "Typesense", "NativePHP", "Unstoppable", "Devin"] {
            assert!(!card(name).contains(">Sponsor for "), "{name}");
            assert!(!card(name).contains("ring-amber-500"), "{name}");
        }
        assert!(card("JetBrains").contains("ring-amber-500"));
    }

    #[test]
    fn marks_a_sponsor_in_the_month_of_its_first_full_year() {
        let before = PartnersTemplate::in_month(Month {
            year: 2026,
            month: 7,
        })
        .render()
        .unwrap();
        let on = PartnersTemplate::in_month(Month {
            year: 2026,
            month: 8,
        })
        .render()
        .unwrap();

        assert_eq!(before.matches(">Sponsor for 1 year<").count(), 1);
        assert_eq!(on.matches(">Sponsor for 1 year<").count(), 2);
    }

    #[test]
    fn renders_each_logo_from_the_public_directory() {
        for partner in CURRENT_SPONSORS
            .iter()
            .map(Sponsorship::partner)
            .chain(PAST_PARTNERS.iter())
        {
            assert!(
                std::path::Path::new("../public")
                    .join(partner.logo().trim_start_matches('/'))
                    .exists(),
                "{}",
                partner.logo()
            );
        }
    }

    #[test]
    fn puts_the_current_sponsors_before_the_part_of_the_journey() {
        let html = PartnersTemplate::default().render().unwrap();

        assert!(
            html.find(">Current sponsors</h2>").unwrap()
                < html.find(">Part of the journey</h2>").unwrap()
        );
    }

    #[test]
    fn links_to_the_sponsorships_page() {
        let html = PartnersTemplate::default().render().unwrap();

        assert!(html.contains("href=\"/sponsorships\" class=\"link\""));
    }

    #[test]
    fn marks_the_partners_as_the_current_page_in_the_navigation() {
        let html = PartnersTemplate::default().render().unwrap();

        assert_eq!(
            html.matches("href=\"/partners\" aria-current=\"page\"")
                .count(),
            2
        );
    }

    #[test]
    fn writes_nunomaduro_as_one_word() {
        let html = PartnersTemplate::default().render().unwrap();

        assert!(html.contains("<title>nunomaduro · Partners</title>"));
        assert!(!html.to_lowercase().contains("nuno maduro"));
    }
}
