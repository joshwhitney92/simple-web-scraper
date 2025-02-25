pub struct HtmlPrettifier;

/*
 * Concrete implementation
 */
mod private_helpers {
    // use scraper::Html;
    use tidier::{Doc, FormatOptions};

    pub fn prettify(html: &str) -> String {
        let opts = FormatOptions::new()
            .tabs(false)
            .strip_comments(false)
            .wrap(120);

        let doc = Doc::new(html, false);
        match doc {
            Ok(inner) => inner.format(&opts).expect("Could not format string!"),
            _ => String::new(),
        }
    }
}

pub trait Prettify {
    fn prettify(html: &str) -> String;
}

impl Prettify for HtmlPrettifier {
    fn prettify(html: &str) -> String {
        private_helpers::prettify(html)
    }
}
