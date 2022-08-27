
use std::default::Default;

#[derive(Clone, Default)]
pub struct Register {
    register_a: u8,
    register_b: u8,
    carry_flag: u8,
    pc: u8,
}

#[derive(Debug)]
pub enum RegisterOp {
    A,
    B,
}
impl From<String> for RegisterOp {
    fn from(a: String) -> Self {
        match a.as_str() {
            "A" => RegisterOp::A,
            "B" => RegisterOp::B,
            _ => panic!("Invalid RegisterOp error")
        }
    }
}

impl Register {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_pc(&mut self, value: u8) {
        self.pc = value
    }

    pub fn pc(&self) -> u8 {
        self.pc
    }

    pub fn inc_pc(&mut self) {
        self.pc += 1
    }

    pub fn set_carry_flag(&mut self, value: u8) {
        self.carry_flag = value
    }

    pub fn carry_flag(&self) -> u8 {
        self.carry_flag
    }

    pub fn set_register_a(&mut self, value: u8) {
        self.register_a = value
    }

    pub fn register_a(&self) -> u8 {
        self.register_a
    }

    pub fn set_register_b(&mut self, value: u8) {
        self.register_b = value
    }

    pub fn register_b(&self) -> u8 {
        self.register_b
    }
}


#[cfg(test)]
mod register_tests {
    use crate::emulator::register::Register;

    #[test]
    fn register_test() {
        let mut register = Register::new();

        register.set_pc(1);
        register.inc_pc();
        let pc = register.pc();
        assert_eq!(pc, 2);

        register.set_carry_flag(1);
        let carry_flag = register.carry_flag();
        assert_eq!(carry_flag, 1);

        register.set_register_a(1);
        let a = register.register_a();
        assert_eq!(a, 1);

        register.set_register_b(1);
        let b = register.register_b();
        assert_eq!(b, 1);
    }

}