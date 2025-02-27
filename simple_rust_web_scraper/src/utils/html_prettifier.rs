pub struct HtmlPrettifier;

/*
 * Concrete implementation
 */
mod private_helpers {
    // use scraper::Html;
    use tidier::{Doc, FormatOptions, Indent};

    pub fn prettify(html: &str) -> String {
        /*
         * let opts = FormatOptions::new()
         *     .tabs(false)
         *     .strip_comments(false)
         *     .custom_tags(tidier::CustomTags::Blocklevel)
         *     .wrap(120);
         */

        let opts = FormatOptions {
            wrap: 80,
            strip_comments: true,
            indent: Indent {
                tabs: true,
                ..Indent::DEFAULT
            },
            ..FormatOptions::DEFAULT
        };

        let doc = Doc::new(html, false);
        match doc {
            Ok(inner) => inner.format(&opts).expect("Could not format string!"),
            Err(e) => {
                println!("Could not format file - Error {e}");
                String::from(html)
            },
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
