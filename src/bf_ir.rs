use std::io::prelude::*;

use brainfuck::ir;
use brainfuck::opcode;

struct Interpreter {
    stack: Vec<u8>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self { stack: vec![0; 1] }
    }
}

impl Interpreter {
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let opcode_code = opcode::Code::from(data)?;
        let code = ir::Code::from(opcode_code.instrs)?;
        let code_len = code.instrs.len();
        let mut pc: usize = 0;
        let mut sp: usize = 0;
        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                ir::IR::SHL(x) => sp = if sp == 0 { 0 } else { sp - x as usize },
                ir::IR::SHR(x) => {
                    sp += x as usize;
                    if sp >= self.stack.len() {
                        let expand = sp - self.stack.len() + 1;
                        for _ in 0..expand {
                            self.stack.push(0);
                        }
                    }
                }
                ir::IR::ADD(x) => {
                    self.stack[sp] = self.stack[sp].overflowing_add(x).0;
                }
                ir::IR::SUB(x) => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(x).0;
                }
                ir::IR::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[sp]])?;
                }
                ir::IR::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                }
                ir::IR::JIZ(x) => {
                    if self.stack[sp] == 0x00 {
                        pc = x as usize;
                    }
                }
                ir::IR::JNZ(x) => {
                    if self.stack[sp] != 0x00 {
                        pc = x as usize;
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() >= 2);
    let mut f = std::fs::File::open(&args[1])?;
    let mut c: Vec<u8> = Vec::new();
    f.read_to_end(&mut c)?;
    let mut i = Interpreter::default();
    i.run(c)
}
