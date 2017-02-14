extern crate seal_lang;
extern crate serde_json;

fn main() {
    let input = include_str!("../../scripts/example.seal");
    let lexer = seal_lang::lexer::Lexer::new(input);

    let mut ast = seal_lang::parser::parse_Module(lexer).unwrap();
    seal_lang::ast::constant_folding::fold_module(&mut ast);

    println!("{}", serde_json::to_string_pretty(&ast).unwrap());
}
