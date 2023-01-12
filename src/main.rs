use std::io;
use std::io::*;
use std::collections::*;

use ast::ExprAst;

mod parser;
mod ast;

fn main() {
    let (input1, input2, num_vars) = get_user_input();
    let mut bool_vars: HashMap<char, usize> = HashMap::new();

    let mut expr1 = parser::Expr::new();
    expr1.build_expr(input1, &mut bool_vars, |c, map| {
        if !c.is_ascii_alphabetic() {
            return Err("Invalid symbol in expression.");
        } else if !map.contains_key(&c) {
            map.insert(c.to_ascii_lowercase(), map.len());
        }
        Ok(())
    }).unwrap();

    let mut expr2 = parser::Expr::new();
    expr2.build_expr(input2, &mut bool_vars, |c, map| {
        if !map.contains_key(&c) {
            return Err("Expression 2 contains variables not in expression 1");
        }
        Ok(())
    }).unwrap();
    
    let ast1 = ExprAst::build(&expr1);
    // dbg!(&ast1);
    let ast2 = ExprAst::build(&expr2);
    // dbg!(&ast2);

    let bool_permutations = get_permutations(num_vars);
    // println!("{}", expr1.get_num_vars());
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

}

fn get_user_input() -> (String, String, usize) {
    let mut input1= String::new();
    let mut input2= String::new();

    print!("Please enter the first expression: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    input1.pop();

    print!("Please enter the second expression: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    input2.pop();

    let mut set1: HashSet<char> = HashSet::new();
    let mut set2: HashSet<char> = HashSet::new();

    input1.chars().for_each(|c| {
        if c.is_ascii_alphabetic() {
            set1.insert(c);
        } 
    });

    input2.chars().for_each(|c| {
        if c.is_ascii_alphabetic() {
            set2.insert(c);
        } 
    });

    // dbg!(&set1.len());
    // dbg!(&set2.len());

    if set1.len() > set2.len() {
        (input1, input2, set1.len())
    } else {
        (input2, input1, set2.len())
    }
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
