use crate::lib::EmulatorErr;
use crate::emulator::instructions::Token;
use crate::emulator::register::RegisterOp;
use std::default::Default;

#[derive(Debug, Default)]
pub struct Compiler;

impl Compiler {
    pub fn new() -> Compiler {
        Compiler::default()
    }

    pub fn compile(&self, tokens: Vec<Token>) -> Result<Vec<u8>, EmulatorErr> {
        if tokens.is_empty() {
            return Err(EmulatorErr::new("instruction is empty"));
        };

        let mut bin_codes = Vec::new();

        for token in tokens {
            let code = match token {
                Token::Add(RegisterOp::A, im) => { self.gen_binary(0b0000, im) },
                Token::Add(RegisterOp::B, im) => { self.gen_binary(0b0101, im) },
                Token::Mov(RegisterOp::A, im) => { self.gen_binary(0b0011, im) },
                Token::Mov(RegisterOp::B, im) => { self.gen_binary(0b0111, im) },
                Token::MovAB => { self.gen_binary_with_zero(0b0001) },
                Token::MovBA => { self.gen_binary_with_zero(0b0100) },
                Token::Jmp(im) => { self.gen_binary(0b1111, im) },
                Token::Jnc(im) => { self.gen_binary(0b1110, im) },
                Token::In(RegisterOp::A) => { self.gen_binary_with_zero(0b0010) },
                Token::In(RegisterOp::B) => { self.gen_binary_with_zero(0b0110) },
                Token::OutB => { self.gen_binary_with_zero(0b1001) },
                Token::OutIm(im) => { self.gen_binary(0b1011, im) },
            };
            bin_codes.push(code);
        };

        Ok(bin_codes)
    }

    fn gen_binary(&self, op: u8, im: u8) -> u8 {
        let shift_op = op << 4;
        let shift_data = im & 0x0f;
        shift_op | shift_data
    }

    #[allow(clippy::erasing_op)]
    fn gen_binary_with_zero(&self, op: u8) -> u8 {
        let shift_op = op << 4;
        let shift_data = 0b0000 & 0x0f;
        shift_op | shift_data
    }
}


#[cfg(test)]
mod compiler_tests {
    use crate::emulator::compiler::Compiler;
    use crate::emulator::instructions::Token;
    use crate::emulator::register::RegisterOp;

    #[test]
    fn compile_test_add_a() {
        let compiler = Compiler::new();
        let token = vec!(Token::Add(RegisterOp::A, 1));
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b00000001));
    }

    #[test]
    fn compile_test_add_b() {
        let compiler = Compiler::new();
        let token = vec!(Token::Add(RegisterOp::B, 1));
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b01010001));
    }

    #[test]
    fn compile_test_mov_a() {
        let compiler = Compiler::new();
        let token = vec!(Token::Mov(RegisterOp::A, 1));
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b00110001));
    }

    #[test]
    fn compile_test_mov_b() {
        let compiler = Compiler::new();
        let token = vec!(Token::Mov(RegisterOp::B, 1));
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b01110001));
    }

    #[test]
    fn compile_test_mov_ab() {
        let compiler = Compiler::new();
        let token = vec!(Token::MovAB);
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b00010000));
    }

    #[test]
    fn compile_test_mov_ba() {
        let compiler = Compiler::new();
        let token = vec!(Token::MovBA);
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b01000000));
    }

    #[test]
    fn compile_test_jmp() {
        let compiler = Compiler::new();
        let token = vec!(Token::Jmp(1));
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b11110001));
    }

    #[test]
    fn compile_test_jnc() {
        let compiler = Compiler::new();
        let token = vec!(Token::Jnc(1));
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b11100001));
    }

    #[test]
    fn compile_test_in_a() {
        let compiler = Compiler::new();
        let token = vec!(Token::In(RegisterOp::A));
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b00100000));
    }

    #[test]
    fn compile_test_in_b() {
        let compiler = Compiler::new();
        let token = vec!(Token::In(RegisterOp::B));
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b01100000));
    }

    #[test]
    fn compile_test_out_b() {
        let compiler = Compiler::new();
        let token = vec!(Token::OutB);
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b10010000));
    }

    #[test]
    fn compile_test_out_im() {
        let compiler = Compiler::new();
        let token = vec!(Token::OutIm(1));
        let code = compiler.compile(token).unwrap();
        assert_eq!(code, vec!(0b10110001));
    }
}