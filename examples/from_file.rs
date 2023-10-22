use rusty_template::{mytest, template, Template};

#[mytest]
#[derive(Template)]
//#[template(path = "./from_file.txt", other = 2, a, list{ 123 })]
//#[template(Hello { name }, path = "./from_file.txt", other = 2)]
#[template = "Hello { name }"]
//#[template()]
#[mytest]
struct MyTemplate;

fn main() {
    println!("{}", MyTemplate.render())
}

/*
 *
 *
item struct: ItemStruct {
    attrs: [],
    vis: Visibility::Inherited,
    struct_token: Struct,
    ident: Ident {
        ident: "MyTemplate",
        span: #0 bytes(137..147),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    fields: Fields::Unit,
    semi_token: Some(
        Semi,
    ),
}
 */
