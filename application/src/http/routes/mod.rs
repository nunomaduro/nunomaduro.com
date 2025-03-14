pub mod api;
pub mod posts;

mod about;
mod health;
mod socials;
mod not_found;
mod talks;
mod sponsorships;

pub use about::About;
pub use health::Health;
pub use socials::Socials;
pub use not_found::NotFound;
pub use sponsorships::Sponsorships;
pub use talks::Talks;
