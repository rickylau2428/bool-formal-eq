use std::io;
use std::io::*;

use ast::ExprAst;

mod parser;
mod ast;

fn main() {
    print!("Please enter the first expression: ");
    let input1 = get_user_input();
    print!("Please enter the second expression: ");
    let input2 = get_user_input();
    let expr1 = parser::Expr::build_expr(input1).expect("Invalid string");
    dbg!(&expr1.rpn);
    let expr2 = parser::Expr::build_expr(input2).expect("Invalid string");

    if expr1.get_num_vars() != expr2.get_num_vars() {
        println!("The two expressions are not logically equivalent (Different # of variables)");
        std::process::exit(1)
    }

    let ast1 = ExprAst::build(&expr1);
    // dbg!(&ast1);
    let ast2 = ExprAst::build(&expr2);
    // dbg!(&ast2);

    let bool_permutations = get_permutations(expr1.get_num_vars());
    println!("{}", expr1.get_num_vars());
    for bool_perm in bool_permutations {
        let first = ast1.evaluate(&bool_perm);
        let second = ast2.evaluate(&bool_perm);
        println!("For perm {:?}, ast1 is {}, ast2 is {}", bool_perm, first, second);

        if first != second {
            println!("The two expressions are not logically equivalent");
            std::process::exit(1);
        }    
    }

    println!("Congrats! The two are logically equivalent");

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
    input.pop();

    input
}

fn get_permutations(n: usize) -> Vec<Vec<bool>> {
    if n == 0 {
        return vec![vec![]]
    }

    let mut perms = Vec::new();
    for perm in get_permutations(n-1) {
        let mut true_append = perm.clone();
        let mut false_append = perm;
        true_append.push(true);
        false_append.push(false);
        perms.push(true_append);
        perms.push(false_append); 
    }

    perms
}

//fn build(path: &String) -> Result<String, Box<dyn Error>> {
//    let contents = fs::read_to_string(path)?;
//
//    Ok(contents)
//
//}
