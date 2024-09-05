use brainfuck::opcode;

use std::{
    env::args,
    error::Error,
    fs::File,
    io::{stdin, stdout, Read, Write},
};

use opcode::{Code, Opcode};

struct Interpreter {
    stack: Vec<u8>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self { stack: vec![0; 1] }
    }
}

impl Interpreter {
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();

        let mut pc = 0;
        let mut sp = 0;

        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                Opcode::SHL => sp = if sp == 0 { 0 } else { sp - 1 },
                Opcode::SHR => {
                    sp += 1;
                    if sp == self.stack.len() {
                        self.stack.push(0)
                    }
                }
                Opcode::ADD => {
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;
                }
                Opcode::SUB => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;
                }
                Opcode::PUTCHAR => stdout().write_all(&[self.stack[sp]])?,
                Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0]
                }
                Opcode::LB => {
                    if 0x00 == self.stack[sp] {
                        pc = code.jtable[&pc]
                    }
                }
                Opcode::RB => {
                    if 0x00 != self.stack[sp] {
                        pc = code.jtable[&pc]
                    }
                }
            }
            pc += 1;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    assert!(args.len() >= 2);
    let mut f = File::open(&args[1])?;
    let mut c: Vec<u8> = Vec::new();
    f.read_to_end(&mut c)?;
    let mut i = Interpreter::default();
    i.run(c)
}
