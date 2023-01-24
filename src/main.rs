use std::io;
use std::io::*;
use expr_builder::evaluate_expr;
use tabled::{builder::Builder, ModifyObject, object::Rows, Alignment, Style};
use linked_hash_map::LinkedHashMap;
use std::thread;
use std::sync::{Arc, RwLock};
use expr_builder::Expr;

mod expr_builder;

static OPERATORS: [char; 7] = ['&', '|', '^', '!', '~', '(', ')'];

fn main() {
    let (inputs, bool_vars) = get_user_input();
    let mut expressions: Arc<RwLock<Vec<Expr>>> = Arc::new(RwLock::new(Vec::new()));
    let permutations: Vec<Vec<bool>> = get_permutations(bool_vars.len());
    let mut full_builder = Builder::default();
    let mut failure_builder = Builder::default();

    for entry in inputs.iter() {
        expressions.write().unwrap().push(expr_builder::build(entry, &bool_vars).expect("Build failed")); 
    }

    full_builder.set_columns(bool_vars.keys().map(|c| c.to_string()).chain(inputs.clone().into_iter()));
    failure_builder.set_columns(bool_vars.keys().map(|c| c.to_string()).chain(inputs.into_iter()));
    let mut flag = true;

    let mut handles = vec![];
    for perm in permutations.iter() {
        let mut record: Vec<usize> = perm.clone().iter().map(|b| b.clone().into()).collect();
        let mut runs: Vec<bool> = Vec::new(); 
        let handle = thread::spawn(|| {

        });
        for entry in expressions.iter() {
            // runs.push(evaluate_expr(entry.root, perm));
        }

        handles.push(handle);
        record.append(&mut runs.clone().iter().map(|b| b.clone().into()).collect());
        let table_row = record.iter().map(|e| e.to_string());
        full_builder.add_record(table_row.clone());

        let res: bool = runs.into_iter().reduce(|acc, e| acc == e).unwrap();
        if !res {
            flag = false;
            failure_builder.add_record(table_row);
        }
    }

    let mut results: Vec<Vec<bool>> = Vec::with_capacity(permutations.len());
    for perm in permutations.iter().enumerate() {
        let exprs = expressions.clone();
        let out_vec = results[perm.0].clone();
        let handle = thread::spawn(move || {
            let expr_vec = exprs.read().unwrap();
            for entry in expr_vec.iter() {
                out_vec.push(evaluate_expr(entry, &perm.1));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let table = full_builder.build()
        .with(Style::rounded())
        .with(Rows::new(1..).modify().with(Alignment::center()))
        .to_string();

    println!("{}", table);

    if flag {
        println!("Congrats! All expressions are logically equivalent");
    } else {
        println!("Not all expressions are logically equivalent");
        println!("Failure cases are as follows: ");
        let failure_table = failure_builder.build()
            .with(Style::rounded())
            .with(Rows::new(1..).modify().with(Alignment::center()))
            .to_string();
        
        println!("{}", failure_table);
    }
}

// fn get_user_input() -> (String, String, HashSet<char>) {
fn get_user_input() -> (Vec<String>, LinkedHashMap<char, usize>) {
    let mut inputs: Vec<String> = Vec::new();
    println!("Boolean Formula Equivalence Checker; enter an empty string to begin evaluation");
    loop {
        print!("Please enter an expression: ");
        let input = read_input();
        if input.is_empty() {
            break;
        } else {
            inputs.push(input);
        }
    }

    // let mut seen_chars: HashSet<char> = HashSet::new();
    let mut seen_chars: LinkedHashMap<char, usize> = LinkedHashMap::new();
    // dbg!(&inputs);

    for entry in inputs.iter() {
        // let mut temp_set: HashSet<char> = HashSet::new();
        let mut temp_map: LinkedHashMap<char, usize> = LinkedHashMap::new();

        for c in entry.chars() {
            if c == ' ' || OPERATORS.contains(&c) || temp_map.contains_key(&c) {
                continue;
            } else if c.is_ascii_alphabetic() {
                temp_map.insert(c, temp_map.len());
            } else {
                panic!("Non-alphabetic variable found");
            }
        }

        if temp_map.len() > seen_chars.len() {
            seen_chars = temp_map;
        }
    }

    return (inputs, seen_chars)

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
        perms.push(false_append); 
        perms.push(true_append);
    }

    perms
}