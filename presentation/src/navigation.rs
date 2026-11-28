pub struct NavigationLink {
    pub path: &'static str,
    pub label: &'static str,
    page_id: &'static str,
    child_page_id: Option<&'static str>,
}

impl NavigationLink {
    #[must_use]
    pub fn is_current(&self, page_id: &str) -> bool {
        self.page_id == page_id
    }

    #[must_use]
    pub fn is_highlighted(&self, page_id: &str) -> bool {
        self.is_current(page_id) || self.child_page_id == Some(page_id)
    }
}

pub const NAVIGATION_LINKS: [NavigationLink; 4] = [
    NavigationLink {
        path: "/",
        label: "About",
        page_id: "about",
        child_page_id: None,
    },
    NavigationLink {
        path: "/posts",
        label: "Posts",
        page_id: "posts",
        child_page_id: Some("post"),
    },
    NavigationLink {
        path: "/talks",
        label: "Talks",
        page_id: "talks",
        child_page_id: None,
    },
    NavigationLink {
        path: "/sponsorships",
        label: "Sponsorships",
        page_id: "sponsorships",
        child_page_id: None,
    },
];

#[cfg(test)]
mod tests {
    use super::NAVIGATION_LINKS;

    #[test]
    fn marks_only_the_link_of_the_page_as_current() {
        let current: Vec<&str> = NAVIGATION_LINKS
            .iter()
            .filter(|link| link.is_current("posts"))
            .map(|link| link.label)
            .collect();

        assert_eq!(current, ["Posts"]);
    }

    #[test]
    fn highlights_the_posts_link_on_a_post_without_marking_it_as_current() {
        let posts = &NAVIGATION_LINKS[1];

        assert!(posts.is_highlighted("post"));
        assert!(!posts.is_current("post"));
    }

    #[test]
    fn highlights_no_link_on_a_page_outside_the_navigation() {
        assert!(
            NAVIGATION_LINKS
                .iter()
                .all(|link| !link.is_highlighted("not-found"))
        );
    }
}
