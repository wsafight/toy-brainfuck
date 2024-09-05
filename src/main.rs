mod opcode;
use std::{env::args, error::Error, fs::read};

use opcode::Code;

struct Interpreter {
    stack: Vec<u8>,
}

impl Interpreter {
    fn new() -> Self {
        Self { stack: vec![0; 1] }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();

        let mut pc = 0;
        let mut sp = 0;

        loop {
            pc += 1;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().skip(1).collect();
    let data = read(&args[0])?;
    let code = Code::from(data)?;
    println!("{:?}", code.instrs);
    Ok(())
}
