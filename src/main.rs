// use coogle_rs::ui;
use coogle_rs::collector::{parse_file, Function, FunctionSignature};
use coogle_rs::matcher::{fuzzy_match, Token};
use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Get file name from user
    println!("Enter the source file name:");
    let src_file = input().trim().to_string();

    // Check if the file exists
    if !fs::metadata(&src_file).is_ok() {
        println!("File not found");
        // Return IO error
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let query = get_query().unwrap();

    let mut funcs = get_funcs(src_file);
    coogle(&mut funcs, query);
    Ok(())
}

fn coogle(funcs: &mut Vec<Function>, query: FunctionSignature) {
    funcs.sort_by(|a, b| {
        let a_score = fuzzy_match(&a.signature, &query);
        let b_score = fuzzy_match(&b.signature, &query);
        b_score.partial_cmp(&a_score).unwrap()
    });

    for i in 0..20 {
        let func = &funcs[i];
        println!(
            "{:?}:{:4}:{:3} - {} :: {} -> {:?}",
            func.location.0,
            func.location.1,
            func.location.2,
            func.name,
            func.signature.return_type,
            func.signature.params,
        );
    }
}

fn get_funcs(file: String) -> Vec<Function> {
    match parse_file(file) {
        Ok(funcs) => funcs,
        Err(e) => {
            println!("Error: {}", e);
            Vec::new()
        }
    }
}

fn get_query() -> Option<FunctionSignature> {
    let stdin = io::stdin();
    println!("Query Format: [return_type] -> ([arg1_type], [arg2_type], ...)");
    println!("Enter the query: ");
    let query = stdin.lock().lines().next().unwrap().unwrap();
    println!("Query: {:?}", query);
    query.trim().to_string();
    let query = Token::tokenize(&query).unwrap();
    match FunctionSignature::from_tokens(query) {
        Ok(query) => Some(query),
        Err(_) => {
            println!("Invalid query");
            None
        }
    }
}

fn input() -> String {
    let mut inp_buf = String::new();
    io::stdin()
        .read_line(&mut inp_buf)
        .expect("Failed to read line");

    inp_buf
}
