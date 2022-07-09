use crate::value_objects::Page;

pub struct ListPage<TPage: Page> {
    id: String,
    elements: Vec<TPage>,
}

impl<TPage: Page> ListPage<TPage> {
    pub fn new(id: String, elements: Vec<TPage>) -> Self {
        Self { id, elements }
    }

    pub fn elements(&self) -> &Vec<TPage> {
        &self.elements
    }
}

impl<TPage: Page> Page for ListPage<TPage> {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::PostPage;

    #[test]
    fn test_id() {
        let page = ListPage::<PostPage>::new("posts".to_string(), vec![]);

        assert_eq!(page.id(), "posts");
    }

    #[test]
    fn test_elements() {
        let page = ListPage::new(
            "posts".to_string(),
            vec![PostPage::new(
                "1".to_string(),
                "how_to_install_php82_rc_on_mac".to_string(),
                "Tutorial".to_string(),
                "How to install PHP 8.2 RC on Mac".to_string(),
                "First, you need to run".to_string(),
                "July 20, 2022".to_string(),
                "4 min read".to_string(),
                "<p> Hi everyone, enjoy this new post about PHP. </p>".to_string(),
            )],
        );
        let elements = page.elements();

        assert_eq!(elements.len(), 1);
    }
}
