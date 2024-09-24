use crate::collector::FunctionSignature;
use strsim::jaro_winkler;

// const FUZZY_THRESHOLD: f64 = 0.70;

pub enum Token {
    Identifier(String),
    LeftParen,
    RightParen,
    Comma,
}

impl Token {
    pub fn tokenize(s: &str) -> Result<Vec<Self>, String> {
        let mut char_iter = s.chars();
        let mut tokens = Vec::new();
        while let Some(chr) = char_iter.next() {
            match chr {
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                ',' => tokens.push(Token::Comma),
                _ => {
                    let mut identifier = String::new();
                    identifier.push(chr);
                    while let Some(ch) = char_iter.next() {
                        match ch {
                            '(' | ')' | ',' => {
                                tokens.push(Token::Identifier(identifier));
                                break;
                            }
                            _ => identifier.push(ch),
                        }
                    }
                }
            }
        }
        Ok(tokens)
    }
}

impl FunctionSignature {
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        let mut return_type = String::new();
        let mut params = Vec::new();
        let is_variadic = false;
        let mut is_return = true;
        for token in tokens {
            match token {
                Token::Identifier(s) => {
                    if is_return {
                        return_type = s;
                        is_return = false;
                    } else {
                        params.push(s);
                    }
                }
                Token::LeftParen => {
                    is_return = false;
                }
                Token::RightParen => {
                    is_return = true;
                }
                Token::Comma => {}
            }
        }
        FunctionSignature {
            return_type,
            params,
            is_variadic,
        }
    }
}

pub fn normalize_query(query: &str) -> String {
    // Tokenize the query: 'Color (Vec4) -> ['Color', '(', 'Vec4', ')']
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    for c in query.chars() {
        if c.is_alphanumeric() {
            current_token.push(c);
        } else {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            tokens.push(c.to_string());
        }
    }

    // Normalize the query
    let mut normalized_query = String::new();
    for token in tokens {
        if token == " " {
            continue;
        }

        if token == "(" || token == ")" {
            normalized_query.push_str(&token);
        } else {
            normalized_query.push_str(&token.to_string());
        }
    }

    normalized_query
}

pub fn fuzzy_match(function: &FunctionSignature, query: &FunctionSignature) -> f64 {
    let mut func_sig = String::new();
    let mut query_sig = String::new();

    func_sig.push_str(&function.return_type);
    func_sig.push_str("(");
    for param in &function.params {
        func_sig.push_str(&param);
        func_sig.push_str(",");
    }
    func_sig.push_str(")");

    query_sig.push_str(&query.return_type);
    query_sig.push_str("(");
    for param in &query.params {
        query_sig.push_str(&param);
        query_sig.push_str(",");
    }
    query_sig.push_str(")");

    // if jaro_winkler(&func_sig, &query_sig) > FUZZY_THRESHOLD {
    //     return true;
    // }
    // false
    jaro_winkler(&func_sig, &query_sig)
}
