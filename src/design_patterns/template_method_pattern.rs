use crate::design_pattern_example::DesignPatternExample;

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
    fn pattern_name<'a>(&self) -> &'a str {
        "Template Method"
    }

    fn example_name<'a>(&self) -> &'a str {
        "Article"
    }

    fn run(&self) {
        let web_blog = WebBlog {};
        web_blog.add_content();

        println!();

        let magazine = Magazine {};
        magazine.add_content();
    }
}
