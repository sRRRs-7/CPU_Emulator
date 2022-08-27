use num_traits::FromPrimitive;
use crate::{
    emulator::{
        instructions::Opcodes,
        register::Register,
        adapter::{ Port, Rom }
    },
    lib::EmulatorErr
};

pub struct CPUemulator {
    pub register: Register,
    pub port: Port,
    pub rom: Rom,
}

impl CPUemulator {
    pub fn new(
        register: Register,
        port: Port,
        rom: Rom,
    ) -> Self {
        if rom.size() > 16 {
            panic!("Overflow rom size: maximum is 16 bytes")
        }
        Self { register, port, rom}
    }

   pub fn fetch(&self) -> u8 {
        let pc = self.register.pc();
        if self.rom.size() <= pc {
            return 0
        }

        self.rom.read(pc)
    }

    pub fn decode(&self, data: u8) -> Result<(Opcodes, u8), EmulatorErr> {
        let op = data >> 4;
        let im = data & 0x0f;

        if let Some(opcode) = FromPrimitive::from_u8(op) {
            match opcode {
                Opcodes::AddA
                | Opcodes::AddB
                | Opcodes::MovA
                | Opcodes::MovB
                | Opcodes::MovA2B
                | Opcodes::MovB2A
                | Opcodes::Jmp
                | Opcodes::Jnc
                | Opcodes::OutIm => Ok((opcode, im)),
                Opcodes::InA
                | Opcodes::InB
                | Opcodes::OutB => Ok((opcode, 0)),
            }
        } else {
            Err(EmulatorErr::new("No match opcode"))
        }
    }

    pub fn execute(&mut self) -> Result<(), EmulatorErr> {
        loop {
            let data = self.fetch();
            let (opcode, immediate) = self.decode(data).unwrap();

            println!("--------------------------------------------");
            println!("fetch: {}", format!("{:#b})", data));
            println!("opcode: {:?}", format!("{:?}", opcode));
            println!("immediate: {}", format!("{:#b})", immediate));

            match opcode {
                Opcodes::AddA => self.add_a(immediate),
                Opcodes::AddB => self.add_b(immediate),
                Opcodes::MovA => self.mov_a(immediate),
                Opcodes::MovB => self.mov_b(immediate),
                Opcodes::MovA2B => self.mov_a2b(),
                Opcodes::MovB2A => self.mov_b2a(),
                Opcodes::Jmp => self.jmp(immediate),
                Opcodes::Jnc => self.jnc(immediate),
                Opcodes::InA => self.in_a(),
                Opcodes::InB => self.in_b(),
                Opcodes::OutB => self.out_b(),
                Opcodes::OutIm => self.out_im(immediate),
            };

            if opcode != Opcodes::Jmp || opcode != Opcodes::Jnc {
                self.register.inc_pc();
            }

            if self.halt() {
                return Ok(());
            }
        }
    }

    fn halt(&self)-> bool {
        self.rom.size() <= self.register.pc() + 1
    }

    fn add_a(&mut self, immediate: u8) {
        let register_a = self.register.register_a();
        let value = register_a + immediate;
        if value > 0x0f {
            self.register.set_carry_flag(1);
        }
        self.register.set_register_a(value & 0x0f);
    }

    fn add_b(&mut self, immediate: u8) {
        let register_b = self.register.register_b();
        let value = register_b + immediate;
        if value > 0x0f {
            self.register.set_carry_flag(1);
        }
        self.register.set_register_b(value & 0x0f);
    }

    fn mov_a(&mut self, immediate: u8) {
        self.register.set_register_a(immediate);
        self.register.set_carry_flag(0)
    }

    fn mov_b(&mut self, immediate: u8) {
        self.register.set_register_b(immediate);
        self.register.set_carry_flag(0)
    }

    fn mov_a2b(&mut self) {
        let register_b = self.register.register_b();
        self.register.set_register_a(register_b);
        self.register.set_carry_flag(0)
    }

    fn mov_b2a(&mut self) {
        let register_a = self.register.register_a();
        self.register.set_register_b(register_a);
        self.register.set_carry_flag(0)
    }

    fn jmp(&mut self, immediate: u8) {
        self.register.set_pc(immediate);
        self.register.set_carry_flag(0);
    }

    fn jnc(&mut self, immediate: u8) {
        if self.register.carry_flag() == 0 {
            self.register.set_pc(immediate);
        }
        self.register.set_carry_flag(0);
    }

    fn in_a(&mut self) {
        let input = self.port.input();
        self.register.set_register_a(input);
        self.register.set_carry_flag(0);
    }

    fn in_b(&mut self) {
        let input = self.port.input();
        self.register.set_register_b(input);
        self.register.set_carry_flag(0);
    }

    fn out_b(&mut self) {
        let register_b = self.register.register_b();
        self.port.set_output(register_b);
        self.register.set_carry_flag(0);
        println!("port (B) output: {}", self.port.output());
    }

    fn out_im(&mut self, immediate: u8) {
        self.port.set_output(immediate);
        self.register.set_carry_flag(0);
        println!("Output: {}", self.port.output());
    }
}


#[cfg(test)]
mod executor_tests{
    use crate::emulator::{
        executor::CPUemulator,
        adapter::Port,
        adapter::Rom,
        register::Register,
    };

    #[test]
    fn test_add_a() {
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let rom = Rom::new(vec!(0b00000001));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 1);
        assert_eq!(emu.register.register_b(), 0);
        assert_eq!(emu.register.pc(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_add_b() {
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let rom = Rom::new(vec!(0b01010001));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 0);
        assert_eq!(emu.register.register_b(), 1);
        assert_eq!(emu.register.pc(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_mov_a() {
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let rom = Rom::new(vec!(0b00110001));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 1);
        assert_eq!(emu.register.register_b(), 0);
        assert_eq!(emu.register.pc(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_mov_b() {
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let rom = Rom::new(vec!(0b01110001));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 0);
        assert_eq!(emu.register.register_b(), 1);
        assert_eq!(emu.register.pc(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_mov_ab() {
        let mut register = Register::new();
        register.set_register_b(2);
        let port = Port::new(0b0000, 0b0000);
        let rom = Rom::new(vec!(0b00010000));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 2);
        assert_eq!(emu.register.register_b(), 2);
        assert_eq!(emu.register.pc(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_mov_ba() {
        let mut register = Register::new();
        register.set_register_a(2);
        let port = Port::new(0b0000, 0b0000);
        let rom = Rom::new(vec!(0b01000000));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 2);
        assert_eq!(emu.register.register_b(), 2);
        assert_eq!(emu.register.pc(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_jmp() {
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let rom = Rom::new(vec!(0b11110010, 0b00110001, 0b01110010));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 0);
        assert_eq!(emu.register.register_b(), 2);
        assert_eq!(emu.register.pc(), 3);
    }

    #[test]
    fn test_jnc() {
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let rom = Rom::new(vec!(0b11110010, 0b00110001, 0b01110010));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 0);
        assert_eq!(emu.register.register_b(), 2);
        assert_eq!(emu.register.pc(), 3);
    }

    #[test]
    fn test_in_a() {
        let register = Register::new();
        let port = Port::new(0b0001, 0b0000);
        let rom = Rom::new(vec!(0b00100000));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 1);
        assert_eq!(emu.register.register_b(), 0);
        assert_eq!(emu.register.pc(), 1);
    }

    #[test]
    fn test_in_b() {
        let register = Register::new();
        let port = Port::new(0b0010, 0b0000);
        let rom = Rom::new(vec!(0b01100000));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 0);
        assert_eq!(emu.register.register_b(), 2);
        assert_eq!(emu.register.pc(), 1);
    }

    #[test]
    fn test_out_b() {
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let rom = Rom::new(vec!(0b10010000));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 0);
        assert_eq!(emu.register.register_b(), 0);
        assert_eq!(emu.register.pc(), 1);
    }

    #[test]
    fn test_out_im() {
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let rom = Rom::new(vec!(0b10110000));
        let mut emu = CPUemulator::new(register, port, rom);
        let result = emu.execute();
        assert!(result.is_ok());
        assert_eq!(emu.register.register_a(), 0);
        assert_eq!(emu.register.register_b(), 0);
        assert_eq!(emu.register.pc(), 1);
    }
}