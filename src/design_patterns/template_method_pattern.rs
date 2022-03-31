use crate::{design_pattern_example::DesignPatternExample, design_pattern_type::DesignPatternType};

trait AddContent {
    fn add_content(&self) {
        let steps = ["Research", "Write", "Review", self.publish()];

        for (i, step) in steps.iter().enumerate() {
            println!("{}) {}", i + 1, step);
        }
    }

    fn publish(&self) -> &str;
}

struct WebBlog;

impl AddContent for WebBlog {
    fn publish(&self) -> &str {
        "Add new web page"
    }
}

struct Magazine;

impl AddContent for Magazine {
    fn publish(&self) -> &str {
        "Print new edition"
    }
}

pub struct TemplateMethodPatternExample;

impl DesignPatternExample for TemplateMethodPatternExample {
    fn design_pattern_type<'a>(&self) -> crate::design_pattern_type::DesignPatternType {
        DesignPatternType::TemplateMethod("Article")
    }

    fn run(&self) {
        let web_blog = WebBlog {};
        web_blog.add_content();

        println!();

        let magazine = Magazine {};
        magazine.add_content();
    }
}
