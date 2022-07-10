use pulldown_cmark::{html, Options, Parser};

use std::fs;

pub struct MarkdownFile {
    content: String,
}

impl MarkdownFile {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap();

        Self::new(content)
    }

    pub fn property(&self, key: &str) -> Option<String> {
        let mut sections = self.content.split("---");

        sections.next();

        let front_matter = sections.next().unwrap().split('\n');

        for line in front_matter {
            if line.is_empty() {
                continue;
            }

            let mut split = line.split(':');
            let potencial_key = split.next().unwrap();

            if potencial_key == key {
                return Some(split.next().unwrap().to_string().trim().to_string());
            }
        }

        None
    }

    pub fn html(&self) -> String {
        let options = Options::all();
        let mut content = self.content.split("---");
        content.next();
        content.next();
        let markdown = content.next().unwrap();
        let parser = Parser::new_ext(markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property() {
        let content = "---
title: Hello World
---";

        let markdown = MarkdownFile::new(content.to_string());

        assert_eq!(markdown.property("title"), Some("Hello World".to_string()));
    }

    #[test]
    fn test_html() {
        let content = "---
title: Hello World
---
<h1> Hello World </h1>";

        let markdown = MarkdownFile::new(content.to_string());

        assert_eq!(markdown.html(), "<h1> Hello World </h1>");
    }
}
