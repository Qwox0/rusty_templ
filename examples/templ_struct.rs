use rusty_template::{templ, Template};
use std::fmt::Display;

#[rusty_template::template]
struct TemplateStruct {
    name: String,
}

fn main() {
    let templ = TemplateStruct { name: "World".to_string() };
    println!("{:?}", templ.render());
}
