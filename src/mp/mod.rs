/* ------------------------------------------------------------
    Machine Pseudo-Code
    Project.Github: !(https://github.com/kerryeon/mp_rust)
---------------------------------------------------------------
    Author:
        Name: kerryeon
        Email: besqer996@gnu.ac.kr
        Github: !(https://github.com/kerryeon)
    Generated:
        Date: 2019-01-03
------------------------------------------------------------ */

mod config;
mod token;
mod tokenizer;
mod parser;

pub fn compile(source: &str) {
    let mut root = parser::new_ast();
    for token in tokenizer::tokenize(source) {
        root.attach(token);
    }
    root.tree();
}
