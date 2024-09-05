use std::{collections::HashMap, error::Error};

#[derive(Debug, PartialEq)]
pub enum Opcode {
    SHR = 0x3E,
    SHL = 0x3C,
    ADD = 0x2B,
    SUB = 0x2D,
    PUTCHAR = 0x2E,
    GETCHAR = 0x2C,
    LB = 0x5B,
    RB = 0x5D,
}

impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2C => Opcode::GETCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            _ => unreachable!(),
        }
    }
}

pub struct Code {
    pub instrs: Vec<Opcode>,
    pub jtable: HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let dict = vec![
            Opcode::SHR as u8,
            Opcode::SHL as u8,
            Opcode::ADD as u8,
            Opcode::SUB as u8,
            Opcode::PUTCHAR as u8,
            Opcode::GETCHAR as u8,
            Opcode::LB as u8,
            Opcode::RB as u8,
        ];
        let instrs: Vec<Opcode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| Opcode::from(*x))
            .collect();
        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: HashMap<usize, usize> = HashMap::new();

        for (index, val) in instrs.iter().enumerate() {
            if Opcode::LB == *val {
                jstack.push(index);
            }
            if Opcode::RB == *val {
                let lb_code_index = jstack.pop().ok_or("error")?;
                jtable.insert(lb_code_index, index);
                jtable.insert(index, lb_code_index);
            }
        }

        Ok(Code {
            instrs,
            jtable,
        })
    }
}
