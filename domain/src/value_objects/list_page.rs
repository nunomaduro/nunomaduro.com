use crate::value_objects::Page;

pub struct ListPage<TPage: Page> {
    id: String,
    title: String,
    description: String,
    elements: Vec<TPage>,
}

impl<TPage: Page> ListPage<TPage> {
    pub fn new(id: String, title: String, description: String, elements: Vec<TPage>) -> Self {
        Self {
            id,
            title,
            description,
            elements,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn elements(&self) -> &Vec<TPage> {
        &self.elements
    }
}

impl<TPage: Page> Iterator for ListPage<TPage> {
    type Item = TPage;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.pop()
    }
}

impl<TPage: Page> Page for ListPage<TPage> {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::PostPage;

    #[test]
    fn test_id() {
        let page = ListPage::<PostPage>::new(
            "posts".to_string(),
            "Nuno Maduro - Blog".to_string(),
            "Nuno Maduro is a speaker".to_string(),
            vec![],
        );

        assert_eq!(page.id(), "posts");
    }

    #[test]
    fn test_elements() {
        let page = ListPage::new(
            "posts".to_string(),
            "Nuno Maduro - Blog".to_string(),
            "Nuno Maduro is a speaker".to_string(),
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
