use rusty_template::templ;
use std::fmt::Display;

fn template_fn(name: impl Display) -> String {
    templ! {
        This is a   Test.
        Hello { name }!
    }
}

fn main() {
    println!("{}", template_fn("World"));

    println!("{}", template_fn(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_fn() {
        let text = template_fn("World");
        assert_eq!(text, "This is a   Test. Hello World!");
    }
}
