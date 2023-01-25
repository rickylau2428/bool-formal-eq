use std::io;
use std::io::*;
use expr_builder::evaluate_expr;
use tabled::{builder::Builder, ModifyObject, object::Rows, Alignment, Style};
use linked_hash_map::LinkedHashMap;
use std::{thread, env};
use std::sync::{Arc, RwLock, Mutex};
use expr_builder::Expr;

mod expr_builder;

static OPERATORS: [char; 7] = ['&', '|', '^', '!', '~', '(', ')'];

fn main() {
    let disallow_thread = env::var("NO_THREAD").is_ok();
    let (inputs, bool_vars) = get_user_input();
    let expressions: Arc<RwLock<Vec<Expr>>> = Arc::new(RwLock::new(Vec::new()));
    let permutations: Vec<Vec<bool>> = get_permutations(bool_vars.len());
    let mut full_builder = Builder::default();
    let mut failure_builder = Builder::default();

    for entry in inputs.iter() {
        expressions.write().unwrap().push(expr_builder::build(entry, &bool_vars).expect("Build failed")); 
    }

    full_builder.set_columns(bool_vars.keys().map(|c| c.to_string()).chain(inputs.clone().into_iter()));
    failure_builder.set_columns(bool_vars.keys().map(|c| c.to_string()).chain(inputs.into_iter()));

    let master_flag;
    if disallow_thread {
        let mut flag = true;
        for perm in permutations.iter() {
            let mut record: Vec<usize> = perm.clone().iter().map(|b| b.clone().into()).collect();
            let mut runs: Vec<bool> = Vec::new(); 
            for entry in expressions.read().unwrap().iter() {
                runs.push(evaluate_expr(entry, perm));
            }

            record.append(&mut runs.clone().iter().map(|b| b.clone().into()).collect());
            let table_row = record.iter().map(|e| e.to_string());
            full_builder.add_record(table_row.clone());

            let res: bool = runs.into_iter().reduce(|acc, e| acc == e).unwrap();
            if !res {
                flag = false;
                failure_builder.add_record(table_row);
            }
        }
        master_flag = flag;
    } else {
        let flag: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
        let failures: Arc<Mutex<Vec<Vec<bool>>>> = Arc::new(Mutex::new(vec![]));
        let mut handles = vec![];
        for perm in permutations.into_iter() {
            let exprs = expressions.clone();
            let thread_flag = flag.clone();
            let failure_tb = failures.clone();
            let handle = thread::spawn(move || {
                let expr_vec = exprs.read().unwrap();
                let mut perm_res: Vec<bool> = Vec::with_capacity(expr_vec.len());
                for expr in expr_vec.iter() {
                    perm_res.push(evaluate_expr(expr, &perm));
                }

                let res = perm_res.iter().copied().reduce(|acc, e| acc == e).unwrap();
                let mut table_row = perm.clone();
                table_row.append(&mut perm_res);
                if !res {
                    *thread_flag.lock().unwrap() = false;
                    failure_tb.lock().unwrap().push(table_row.clone());
                }
                table_row
            });

            handles.push(handle);
        }
    
        for handle in handles.into_iter() {
            let record = handle.join().unwrap();
            let row = record.iter().map(|b| b.clone().into()).collect::<Vec<i32>>();
            full_builder.add_record(row.iter().map(|e| e.to_string()));
        }

        for fail in failures.lock().unwrap().iter() {
            let row = fail.iter().map(|b| b.clone().into()).collect::<Vec<i32>>();
            failure_builder.add_record(row.iter().map(|e| e.to_string()));
        }
        master_flag = *flag.lock().unwrap();
    }

    let table = full_builder.build()
        .with(Style::rounded())
        .with(Rows::new(1..).modify().with(Alignment::center()))
        .to_string();

    println!("{}", table);

    if master_flag {
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

fn get_user_input() -> (Vec<String>, LinkedHashMap<char, usize>) {
    let mut inputs: Vec<String> = Vec::new();

    // println!("Boolean Formula Equivalence Checker; testing thread speedup");
    // let expr1 = String::from("(a & (b | ~c)) ^ ~(~(e & f) | (g & h)) ^ d");
    // let expr2 = String::from("~a | (b ^ c) & (~(e & (g | h)) | (a & c)) ^ d");
    // inputs.push(expr1);
    // inputs.push(expr2);

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

    let mut seen_chars: LinkedHashMap<char, usize> = LinkedHashMap::new();
    for entry in inputs.iter() {
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