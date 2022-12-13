use std::io;
use std::io::*;

mod parser;

fn main() {
//    let expr1 = get_user_input();
//    let expr2 = get_user_input();


//    let expr = build(&args[1]).unwrap_or_else(|err| {
//        eprintln!("Problem parsing arguments: {err}");
//        process::exit(1);
//    });
//    dbg!(expr);

}

fn get_user_input() -> String {
    let mut input = String::new();
    println!("Input an expression: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    println!("Input is {}", input);

    input
}

//fn build(path: &String) -> Result<String, Box<dyn Error>> {
//    let contents = fs::read_to_string(path)?;
//
//    Ok(contents)
//
//}
