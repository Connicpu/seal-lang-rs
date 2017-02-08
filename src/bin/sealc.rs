extern crate seal_lang;

fn main() {
    let input = r#"
    fn main() {
        let err
        
        let me = foo("me")
        err = a.fly()
        
        if err {
            print("I'm sad")
        } else if dont_care_about_flying() {
            print("meh")
        } else {
            print("I can fly!")
        }
    }
    "#;
    let ast = seal_lang::parser::parse_Module(input).unwrap();
    println!("{:#?}", ast);
}
