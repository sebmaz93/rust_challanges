use std::fmt::Display;

fn info<T: Display>(text: &T) {
    println!("{}", text)
}

#[test]
fn print(){
    let st = "?";
    let stri = "?".to_string();
    info(&st);
    info(&stri)
}