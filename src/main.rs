use std::{cell::RefCell, env, fs, intrinsics::prefetch_write_instruction, rc::Rc};

use eval::{eval_object, Scope};
use lexer::{lex, tokenize};
use parser::parse;

pub mod eval;
pub mod lexer;
pub mod parser;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = args.get(1).expect("Filepath needed as argument");

    // dbg!(filename);

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut token_list = lex(&tokenize(&contents));
    // dbg!(&token_list);
    token_list.reverse();
    let parsed_repr = parse(&mut token_list).unwrap();
    // dbg!(&parsed_repr);

    let result = eval_object(&parsed_repr, &mut Rc::new(RefCell::new(Scope::default()))).unwrap();

    dbg!(result);
}
