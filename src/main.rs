use std::io;
use std::io::*;
use std::collections::*;
use expr_builder::evaluate_expr;
// use tabled::{builder::Builder};
use linked_hash_map::LinkedHashMap;
use expr_builder::Expr;

mod expr_builder;

static OPERATORS: [char; 7] = ['&', '|', '^', '!', '~', '(', ')'];

fn main() {
    let (inputs, bool_vars) = get_user_input();
    let mut expressions: Vec<Expr> = Vec::new();
    let permutations: Vec<Vec<bool>> = get_permutations(bool_vars.len());
    // let mut builder = Builder::default();

    for entry in inputs.iter() {
        expressions.push(expr_builder::build(entry, &bool_vars).expect("Build failed")); 
    }

    // let mut status = true;
    for perm in permutations.iter() {
        let mut runs: Vec<bool> = Vec::new(); 
        for entry in expressions.iter() {
            runs.push(evaluate_expr(entry, perm));
        }
        let res: bool = runs.into_iter().reduce(|acc, e| acc == e).unwrap();
        if !res {
            println!("Not all expressions are logically equivalent");
            std::process::exit(1);
        }
    }

    println!("Congrats! All expressions are logically equivalent");
}

// fn get_user_input() -> (String, String, HashSet<char>) {
fn get_user_input() -> (Vec<String>, LinkedHashMap<char, usize>) {
    let mut inputs: Vec<String> = Vec::new();
    println!("Boolean Formula Equivalence Checker; enter an empty string to evaluate");
    loop {
        print!("Please enter an expression: ");
        let input = read_input();
        if input.is_empty() {
            break;
        } else {
            inputs.push(input);
        }
    }

    let mut seen_chars: HashSet<char> = HashSet::new();
    // dbg!(&inputs);

    for entry in inputs.iter() {
        let mut temp_set: HashSet<char> = HashSet::new();

        for c in entry.chars() {
            if c == ' ' || OPERATORS.contains(&c) {
                continue;
            } else if c.is_ascii_alphabetic() {
                temp_set.insert(c);
            } else {
                panic!("Non-alphabetic variable found");
            }
        }

        if temp_set.len() > seen_chars.len() {
            seen_chars = temp_set;
        }
    }

    let bool_vars:LinkedHashMap<char, usize> = 
        seen_chars.iter()
                  .enumerate()
                  .map(|(i, c)| (*c, i))
                  .collect::<LinkedHashMap<char, usize>>();

    return (inputs, bool_vars)

}

fn read_input() -> String {
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