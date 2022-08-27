use std::{env, fs::File, io::{BufReader, BufRead}};
use emulator::executor::CPUemulator;

mod emulator;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        panic!("necessary command arg: [file_path]")
    }

    let file = File::open(args.get(1).unwrap()).unwrap();
    let reader = BufReader::new(&file);
    let operations: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut instructions = emulator::parser::Parser::new(operations);
    let token = instructions.parse().unwrap();

    let compiler =emulator::compiler::Compiler::new();
    let dec = compiler.compile(token).unwrap();

    let rom = emulator::adapter::Rom::new(dec);
    let register = emulator::register::Register::new();
    let port = emulator::adapter::Port::new(0b0000, 0b0000);
    let mut emulator = CPUemulator::new(register, port, rom);
    emulator.execute().unwrap();
}
