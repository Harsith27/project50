use coogle_rs::collector::FunctionSignature;
use coogle_rs::{
    collector::parse_file,
    matcher::{fuzzy_match, Token},
};
// use std::fs::{self, File};
use std::io::{self, BufRead};

fn main() {
    // Get file name from user
    println!("Enter the source file name:");
    let stdin = io::stdin();
    let src_file = stdin.lock().lines().next().unwrap().unwrap();
    let src_file = src_file.trim().to_string();

    let mut funcs = match parse_file(src_file) {
        Some(funcs) => funcs,
        None => return,
    };

    println!("Query Format: [return_type] -> ([arg1_type], [arg2_type], ...)");
    println!("Enter the query: ");
    let query = stdin.lock().lines().next().unwrap().unwrap();
    println!("Query: {:?}", query);
    let query = query.trim().to_string();
    let query = FunctionSignature::from_tokens(Token::tokenize(&query).unwrap());
    println!("Query Function: {:?}", query);

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

    //     if !fuzzy_match(&func, &query) {
    //         continue;
    //     }
    //     println!(
    //         "{:?}:{:4}:{:3} - {} :: {} -> {:?}",
    //         func.location.0,
    //         func.location.1,
    //         func.location.2,
    //         func.name,
    //         func.signature.return_type,
    //         func.signature.params,
    //     );
    // }
}
