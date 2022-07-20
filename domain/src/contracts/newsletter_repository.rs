use async_trait::async_trait;

#[async_trait]
pub trait NewsletterRepository {
    async fn add(&self, email: &str) -> ();
}
