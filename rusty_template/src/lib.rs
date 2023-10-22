mod lexer;

pub use macros::*;
use std::fmt::Display;

pub trait Template {
    fn render(&self) -> String;
}


/*
#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;
*/

/*
pub struct Template {}

impl Template {
    pub fn new() -> Template {
        Template {}
    }

    pub fn text(self, text: &str) -> Self {}
}
*/

const TAG: &str = "div";
const TEST: bool = true;

pub fn format(text: impl Display) -> String {
    format!("<div>Text: {text} inside a {TAG}</div>")
}

/*
pub fn hello_world(text: impl Display) -> String {
    templ!( Hello World "{}").to_string()
}

pub fn fn_template_one_line(text: impl Display) -> String {
    templ!(<div>Text: { text } inside a { if TEST { "a" } }</div>).to_string()
}

pub fn fn_template_for(strings: &[String]) -> String {
    templ!(
    <ul>
    { for s in strings {
        <li>{ s }</li>
    }}
    </ul>
    )
    .to_string()
}

pub fn fn_template_if(on: bool) -> String {
    templ!(
        <div>
        It is {
            if on {

            }
        }
        </div>
    )
    .to_string()
}

pub fn fn_template_for(strings: &[String]) -> String {
    templ!(
    { for s in strings {
        <li>{ s }</li>
    }}
    )
    .to_string()
}
*/

/*
pub fn fn_template(text: impl Display) -> String {
    templ!(
    <div>
    Text: { text } inside a { TAG }
    </div>
    ).to_string()
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_format() {
        assert_eq!(&format(5), "<div>Text: 5 inside a div</div>")
    }

    #[test]
    fn test_hello_world() {
        assert_eq!(&hello_world(5), "Hello World")
    }

    #[test]
    fn test_fn_one_line() {
        assert_eq!(&fn_template_one_line(5), "<div>Text: 5 inside a div</div>")
    }

    #[test]
    fn test_fn() {
        assert_eq!(&fn_template(5), "<div>Text: 5 inside a div</div>")
    }
    */
}
