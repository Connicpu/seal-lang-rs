extern crate seal_lang;

fn main() {
    let input = include_str!("../../scripts/example.seal");
    let ast = seal_lang::parser::parse_Module(input).unwrap();
    println!("{:#?}", ast);
}
