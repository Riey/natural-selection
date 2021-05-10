use crate::constants::BaseType;

use memchr::memchr;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::slice::Iter;

pub fn run(code: &[Instruction], input: &[BaseType]) -> Result<Vec<BaseType>, ()> {
    let mut interpreter = Interpreter::new(code, input);

    if !interpreter.run() {
        Err(())
    } else {
        Ok(interpreter.into_output())
    }
}

#[derive(Copy, Clone, FromPrimitive)]
#[repr(u8)]
pub enum Instruction {
    /// `<`
    DecPtr,
    /// `>`
    IncPtr,
    /// `-`
    DecVal,
    /// `+`
    IncVal,
    /// `.`
    Write,
    /// `,`
    Read,
    /// `[`
    JumpLeft,
    /// `]`
    JumpRight,
    /// `@`
    Halt,
}

impl Distribution<Instruction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Instruction {
        FromPrimitive::from_u8(rng.gen_range(0..9)).unwrap_or_else(|| unreachable!())
    }
}

fn cast_instruction_as_bytes(instructions: &[Instruction]) -> &[u8] {
    unsafe { instructions.align_to().1 }
}

struct Interpreter<'a> {
    dead_count: usize,
    pc: usize,
    code: &'a [Instruction],
    tape: Tape,
    input: Iter<'a, BaseType>,
    output: Vec<BaseType>,
}

impl<'a> Interpreter<'a> {
    pub fn new(code: &'a [Instruction], input: &'a [BaseType]) -> Self {
        Self {
            dead_count: 100000,
            pc: 0,
            code,
            tape: Tape::new(8196),
            input: input.into_iter(),
            output: Vec::with_capacity(100),
        }
    }

    fn find_inst(&self, inst: Instruction) -> Option<usize> {
        let bytes = cast_instruction_as_bytes(&self.code[self.pc..]);
        memchr(inst as u8, bytes)
    }

    pub fn run_inst(&mut self, inst: Instruction) {
        match inst {
            Instruction::DecPtr => self.tape.dec_ptr(),
            Instruction::IncPtr => self.tape.inc_ptr(),
            Instruction::DecVal => self.tape.dec_val(),
            Instruction::IncVal => self.tape.inc_val(),
            Instruction::Write => self.output.push(self.tape.val()),
            Instruction::Read => self.tape.set_val(*self.input.next().unwrap_or(&0)),
            Instruction::JumpLeft => {
                if self.tape.val() == 0 {
                    self.pc = self
                        .find_inst(Instruction::JumpRight)
                        .unwrap_or(self.code.len());
                }
            }
            Instruction::JumpRight => {
                if self.tape.val() != 0 {
                    self.pc = self.find_inst(Instruction::JumpLeft).unwrap_or(0);
                }
            }
            Instruction::Halt => {
                self.pc = self.code.len();
            }
        }
    }
    pub fn run(&mut self) -> bool {
        while let Some(&inst) = self.code.get(self.pc) {
            match self.dead_count.checked_sub(1) {
                Some(dead_count) => self.dead_count = dead_count,
                None => return false,
            }
            self.pc += 1;
            self.run_inst(inst);
        }

        true
    }

    pub fn into_output(self) -> Vec<BaseType> {
        self.output
    }
}

struct Tape {
    ptr: usize,
    bytes: Vec<BaseType>,
}

impl Tape {
    pub fn new(size: usize) -> Self {
        Self {
            ptr: 0,
            bytes: vec![0; size],
        }
    }

    pub fn inc_ptr(&mut self) {
        self.ptr += 1;

        if self.ptr == self.bytes.len() {
            self.ptr = 0;
        }
    }

    pub fn dec_ptr(&mut self) {
        self.ptr = self.ptr.checked_sub(1).unwrap_or(self.bytes.len() - 1);
    }

    pub fn inc_val(&mut self) {
        self.bytes[self.ptr] = self.bytes[self.ptr].wrapping_add(1);
    }

    pub fn dec_val(&mut self) {
        self.bytes[self.ptr] = self.bytes[self.ptr].wrapping_sub(1);
    }

    pub fn set_val(&mut self, val: BaseType) {
        self.bytes[self.ptr] = val;
    }

    pub fn val(&self) -> BaseType {
        self.bytes[self.ptr]
    }
}

#[test]
fn run_test() {
    assert_eq!(run(&[Instruction::IncVal, Instruction::Write,], &[]), &[1]);
}
