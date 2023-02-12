use async_trait::async_trait;
use domain::contracts::NewsletterRepository;
use reqwest::header::ACCEPT;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use std::default::Default;

pub struct MailcoachNewsletterRepository {
    client: Client,
}

impl Default for MailcoachNewsletterRepository {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NewsletterRepository for MailcoachNewsletterRepository {
    async fn add(&self, email: &str) -> () {
        let uri = format!(
            "https://nunomaduro.mailcoach.app/api/email-lists/{}/subscribers",
            std::env::var("MAILCOACH_MAIL_LIST_ID").unwrap()
        );

        self.client
            .post(uri)
            .body(format!("{{\"email\":\"{email}\"}}"))
            .timeout(std::time::Duration::from_secs(2))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .header(
                AUTHORIZATION,
                format!("Bearer {}", std::env::var("MAILCOACH_API_TOKEN").unwrap()),
            )
            .send()
            .await
            .unwrap();
    }
}
