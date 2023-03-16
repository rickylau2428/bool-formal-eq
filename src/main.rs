use std::io::*;
use tabled::{builder::Builder, ModifyObject, object::Rows, Alignment, Style};

mod parser;
mod ast;
mod bdd;

fn main() {
    let raw_inputs = get_user_input();
    let input = parser::create_session(raw_inputs.clone()).unwrap();
    let ast_session = ast::build_ast_session(&input);

    let mut truth_table = Builder::default();
    let mut cex_table = Builder::default();

    truth_table.set_columns(input.ast_order.keys().map(|c| c.to_string()).chain(raw_inputs.clone().into_iter()));
    cex_table.set_columns(input.ast_order.keys().map(|c| c.to_string()).chain(raw_inputs.clone().into_iter()));

    for (case, res) in ast_session.cases.iter().zip(ast_session.results.iter()) {
        let mut case = case.clone();
        case.append(&mut res.clone());
        let table_row: Vec<usize> = case.iter().map(|b| b.clone().into()).collect();
        let table_row: Vec<String> = table_row.iter().map(|e| e.to_string()).collect();
        truth_table.add_record(table_row);
    }

    for cex in ast_session.cex.iter() {
        let table_row: Vec<usize> = cex.iter().map(|b| b.clone().into()).collect();
        let table_row: Vec<String> = table_row.iter().map(|e| e.to_string()).collect(); 
        cex_table.add_record(table_row);
    }

    let table = truth_table.build()
        .with(Style::rounded())
        .with(Rows::new(1..).modify().with(Alignment::center()))
        .to_string();

    println!("{}", table);

    if ast_session.all_eq {
        println!("Congrats! All expressions are logically equivalent");
    } else {
        println!("Not all expressions are logically equivalent");
        println!("Failure cases are as follows: ");

        let failure_table = cex_table.build()
            .with(Style::rounded())
            .with(Rows::new(1..).modify().with(Alignment::center()))
            .to_string();
        
        println!("{}", failure_table);
    }
}

fn get_user_input() -> Vec<String> {
    let mut inputs: Vec<String> = Vec::with_capacity(5); // Arbitrary

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

    inputs
}

fn read_input() -> String {
    let mut input = String::new();
    
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    input.pop();

    input
}