use colored::*;
use std::env::args;
use std::fs::File;
use std::io::{self, Read, Write};

mod lox_err;
use lox_err::LoxErr;

mod token;
use token::{Token, TokenKind};

mod scanner;
use scanner::Scanner;

mod expression;
use expression::Expression;

mod parser;
use parser::Parser;

fn run(statement: &str) -> Result<bool, Vec<LoxErr>> {
    let mut scanner = Scanner::new(statement.to_string());

    match scanner.scan() {
        Err(errs) => Err(errs),
        Ok(tokens) => {
            println!("{:?}", tokens);
            let mut parser = Parser::new(tokens.to_vec());
            match parser.parse() {
                Ok(expression) => println!("Parsed: {}", expression),
                Err(err) => eprintln!("{}", format!("{}", err).red()),
            }
            Ok(true)
        }
    }
}

fn run_file(fname: &String) {
    let file = File::open(fname);

    match file {
        Ok(mut file) => {
            let mut program = String::new();
            file.read_to_string(&mut program).unwrap();
            let source = program.trim_end();

            let mut scanner = Scanner::new(String::from(source));
            match scanner.scan() {
                Err(errs) => {
                    for err in errs {
                        eprintln!("{}", format!("{}", err).red())
                    }
                }
                _ => println!("{:?}", scanner),
            }
        }
        Err(e) => eprintln!("File read error: {}", e),
    }
}

fn run_interpreter() {
    loop {
        print!("{} ", ">>".green().bold());
        io::stdout().flush().unwrap();

        let mut statement = String::new();

        match io::stdin().read_line(&mut statement) {
            Ok(_) => {
                let statement = statement.trim_end();

                if statement == "exit" {
                    println!("\n{}", "bye!!".green());
                    return;
                } else {
                    match run(statement) {
                        Ok(_) => println!("{}", statement),
                        Err(errs) => {
                            for err in errs {
                                eprintln!("{}", err);
                            }
                            break;
                        }
                    }
                }
            }
            Err(e) => println!("read error: {}", e),
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let expr = Expression::NumberLiteral(100.00);
    let sexpr = Expression::StringLiteral(String::from("Testing lol"));
    println!("Expression: {}", expr);
    println!("Expression: {}", sexpr);

    let unary_expr = Expression::Unary {
        operator: Token::new(TokenKind::Bang, String::from("!"), 20),
        right: Box::new(Expression::Unary {
            operator: Token::new(TokenKind::Bang, String::from("!"), 20),
            right: Box::new(sexpr),
        }),
    };

    println!("Expression: {}", unary_expr);

    let binary_expr = Expression::Binary {
        left: Box::new(unary_expr),
        operator: Token::new(TokenKind::Plus, String::from("+"), 20),
        right: Box::new(Expression::Unary {
            operator: Token::new(TokenKind::Bang, String::from("!"), 20),
            right: Box::new(expr),
        }),
    };

    println!("Expression: {}", binary_expr);

    if args.len() > 2 {
        println!("Usage: lox [file]");
    } else if args.len() == 2 {
        println!("running file...");
        run_file(&args[1]);
    } else {
        run_interpreter();
    }
}
