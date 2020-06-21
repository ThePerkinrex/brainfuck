use std::io::{stdin, stdout, Write};

use crate::ast::Code;

pub fn run(c: Vec<Code>) {
	let mut tape = [0_u8; 100];
	let mut pointer: usize = 0;
	inner_run(&c, &mut tape, &mut pointer)
}

fn inner_run(c: &Vec<Code>, tape: &mut [u8], pointer: &mut usize) {
	for statement in c {
		//println!("RUNNING {} {} {:?}", pointer, tape[*pointer], statement);
		match statement {
			Code::INCR_POINTER() => {
				*pointer += 1;
				if *pointer == tape.len() {
					*pointer = 0
				}
			}
			Code::DECR_POINTER() => {
				if *pointer == 0 {
					*pointer = tape.len()
				}
				*pointer -= 1;
			}

			Code::INCR_BYTE() => tape[*pointer] += 1,
			Code::DECR_BYTE() => tape[*pointer] -= 1,

			Code::READ() => {
				print!("READ: ");
				stdout().flush().expect("Error flushing stdout");
				let mut buf = String::new();
				stdin().read_line(&mut buf).expect("Error reading line");
				let ch = buf.bytes().next().expect("No char to read");
				tape[*pointer] = ch;
			}
			Code::WRITE() => print!("{}", tape[*pointer] as char),

			Code::LOOP(to_loop) => {
				while tape[*pointer] != 0 {
					inner_run(to_loop, tape, pointer)
				}
			}
		}
	}
}
