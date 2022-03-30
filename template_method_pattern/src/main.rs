pub trait AddContent {
    fn add_content(&self) {
        let steps = ["Research", "Write", "Review", self.publish()];

        for (i, step) in steps.iter().enumerate() {
            println!("{}) {}", i + 1, step);
        }
    }
    fn publish(&self) -> &str;
}

struct WebBlog;
struct Magazine;

impl AddContent for WebBlog {
    fn publish(&self) -> &str {
        "Add new web page"
    }
}

impl AddContent for Magazine {
    fn publish(&self) -> &str {
        "Print new edition"
    }
}

fn main() {
    let web_blog = WebBlog {};
    web_blog.add_content();

    println!();

    let magazine = Magazine {};
    magazine.add_content();
}
