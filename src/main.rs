use std::io;
use std::io::*;

mod parser;
mod ast;

fn main() {
    let input1 = get_user_input();
    let input2 = get_user_input();
    let expr1 = parser::Expr::build_expr(input1);
    let expr2 = parser::Expr::build_expr(input2);
//    let expr = build(&args[1]).unwrap_or_else(|err| {
//        eprintln!("Problem parsing arguments: {err}");
//        process::exit(1);
//    });
//    dbg!(expr);

}

fn get_user_input() -> String {
    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input
}

//fn build(path: &String) -> Result<String, Box<dyn Error>> {
//    let contents = fs::read_to_string(path)?;
//
//    Ok(contents)
//
//}
