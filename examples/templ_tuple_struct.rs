use rusty_template::{templ, Template};
use std::fmt::Display;

#[rusty_template::template]
struct TemplateTupleStruct(&'static str);

fn main() {
    let templ = TemplateTupleStruct("World");
    let TemplateTupleStruct { 0: _0} = templ;
    println!("{:?}", templ.render());
}
