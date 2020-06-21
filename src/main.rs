use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use recolored::*;

use structopt::StructOpt;

use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;

mod ast;
use ast::Code;
mod interpreter;

#[derive(StructOpt, Debug)]
#[structopt(name = "Rusty BF")]
struct Opt {
    // // A flag, true if used in the command line. Note doc comment will
    // // be used for the help message of the flag. The name of the
    // // argument will be, by default, based on the name of the field.
    // /// Activate debug mode
    // #[structopt(short, long)]
    // debug: bool,

    // // The number of occurrences of the `v/verbose` flag
    // /// Verbose mode (-v, -vv, -vvv, etc.)
    // #[structopt(short, long, parse(from_occurrences))]
    // verbose: u8,
    /// Input file
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: Option<PathBuf>,
}

#[derive(Parser)]
#[grammar = "bf.pest"]
struct BFParser;

fn main() {
    let o = Opt::from_args();
    //println!("{:#?}", o);
    //println!("Hello, world!");
    if let Some(f) = o.input {
        let mut file = File::open(f).expect("Error opening file");
        let mut s = String::new();
        file.read_to_string(&mut s).expect("Error reading file");
        //".[>+>+<<-]>>[<<+>>-]<<,>,>,"
        parse_and_run(s.as_str());
    } else {
        repl()
    }
}

fn repl() {
    loop {
        print!(" >>> ");
        stdout().flush().expect("Error flushing stdout");
        let mut s = String::new();
        if let Ok(_) = stdin().read_line(&mut s) {
            parse_and_run(s.as_str());
        } else {
            break;
        }
    }
}

fn parse_and_run(s: &str) {
    let parsing = std::time::Instant::now();
    match BFParser::parse(Rule::PROGRAM, s) {
        Ok(mut r) => {
            let p_e = parsing.elapsed();
            println!("Parsing elapsed {}μs", p_e.as_micros());
            let program = r.next().unwrap();
            // print(program.clone(), " ┣ ".into(), " ┗ ".into());

            // println!();

            let c = as_code(program.into_inner());
            //println!("{:?}", c);
            let running = std::time::Instant::now();
            interpreter::run(c);
            let r_e = running.elapsed();
            let t_e = parsing.elapsed();
            println!("\nRunning elapsed {}μs", r_e.as_micros());
            println!("\nTotal elapsed (with addeed AST conversion) {}μs", t_e.as_micros());
        }
        Err(e) => println!("{}", e),
    }
}

#[allow(dead_code)]
fn print(p: Pair<Rule>, s: String, e: String) {
    print!(
        "{}({})",
        format!("{:?}", p.as_rule()).blue(),
        p.as_str().green()
    );
    let inner = p.into_inner().collect::<Vec<_>>();
    if inner.len() > 1 {
        println!("");
        let mut i = 0;
        for pair in inner.clone() {
            println!("{} ┃ ", s[..s.len() - " ┣ ".len()].to_string());
            if i < inner.len() - 1 {
                print!("{}", s);
            } else {
                print!("{}", e);
                //line = " ";
            }
            print(pair, format!(" ┃ {}", s), format!(" ┃ {}", e));
            i += 1;
            if i < inner.len() {
                println!("");
            }
        }
    //print!("\n{}", s[..s.len()-" ┣ ".len()].to_string());
    } else if inner.len() > 0 {
        print!("{}", " > ".red());
        for pair in inner {
            print(pair, s.clone(), e);
            break;
        }
    }
}

fn as_code(inner: Pairs<Rule>) -> Vec<Code> {
    let mut res: Vec<Code> = Vec::new();
    for p in inner {
        match p.as_rule() {
            Rule::INCR_POINTER => res.push(Code::INCR_POINTER()),
            Rule::DECR_POINTER => res.push(Code::DECR_POINTER()),

            Rule::INCR_BYTE => res.push(Code::INCR_BYTE()),
            Rule::DECR_BYTE => res.push(Code::DECR_BYTE()),

            Rule::READ => res.push(Code::READ()),
            Rule::WRITE => res.push(Code::WRITE()),

            Rule::LOOP => res.push(Code::LOOP(as_code(p.into_inner()))),
            Rule::EOI => (),

            _ => panic!("Rule {:?} is not a valid instruction", p.as_rule()),
        }
    }
    res
}
