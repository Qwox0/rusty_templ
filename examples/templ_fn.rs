use rusty_template::templ;
use std::fmt::Display;

fn template_fn(name: impl Display) -> String {
    templ!(This is a Test. Hello { name }!)
}

fn expected(name: impl Display) -> String {
    {
        let mut template = String::new();
        template.push_str("Hello ");
        template.push_str(name.to_string().as_str());
        template
    }
}

fn format(name: impl Display) -> String {
    format!("Hello {}", name)
}

fn main() {
    println!("{}", template_fn("World"));
}
