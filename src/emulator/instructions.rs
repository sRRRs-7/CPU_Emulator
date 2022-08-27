
use crate::emulator::register::RegisterOp;

use num_derive::FromPrimitive;


#[derive(Debug, PartialEq, FromPrimitive)]
pub enum Opcodes {
    AddA = 0b0000,
    AddB = 0b0101,
    MovA = 0b0011,
    MovB = 0b0111,
    MovA2B = 0b0001,
    MovB2A = 0b0100,
    Jmp = 0b1111,
    Jnc = 0b1110,
    InA = 0b0010,
    InB = 0b0110,
    OutB = 0b1001,
    OutIm = 0b1011,
}

#[derive(Debug)]
pub enum Token {
    Add(RegisterOp, u8),
    Mov(RegisterOp, u8),
    MovAB,
    MovBA,
    Jmp(u8),
    Jnc(u8),
    In(RegisterOp),
    OutB,
    OutIm(u8),
}

