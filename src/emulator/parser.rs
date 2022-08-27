
use crate::emulator::instructions::Token;
use crate::emulator::register::RegisterOp;
use crate::lib::EmulatorErr;

#[derive(Debug)]
pub struct Parser {
    pub index: usize,
    pub opcodes: Vec<String>,
}

impl Parser {
    pub fn new(operations: Vec<String>) -> Parser {
        let mut opcodes = Vec::new();

        for operation in operations {
            let op: Vec<&str> = operation.split(" ").collect();
            for o in op {
                let st = o.to_string();
                opcodes.push(st);
            }
        }

        Parser { index: 0, opcodes }
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, EmulatorErr> {
        let mut results = Vec::new();

        loop {
            let opcode = self.opcodes.get(self.index);

            let opcode = match opcode {
                Some(op) => op,
                None => {
                    break
                }
            };

            match opcode.as_str() {
                "mov" => {
                    self.index += 1;
                    let lhs = self.opcodes.get(self.index).expect("mov instruction left side error");

                    self.index += 1;
                    let rhs = self.opcodes.get(self.index).expect("mov instruction right side error");

                    let token =
                        if lhs == "A" && rhs == "B" {
                            Token::MovBA
                        } else if lhs == "B" && rhs == "A" {
                            Token::MovAB
                        } else {
                            Token::Mov(
                                RegisterOp::from(lhs.to_string()),
                                self.binary_to_decimal(rhs).unwrap(),
                            )
                        };

                    results.push(token)

                },
                "add" => {
                    self.index += 1;
                    let lhs = self.opcodes.get(self.index).expect("add instruction left side error");

                    self.index += 1;
                    let rhs = self.opcodes.get(self.index).expect("add instruction right side error");

                    let token = Token::Add(
                        RegisterOp::from(lhs.to_string()),
                        self.binary_to_decimal(rhs).unwrap(),
                    );

                    results.push(token)

                },
                "jmp" => {
                    self.index += 1;
                    let immediate = self.opcodes.get(self.index).expect("jmp instruction error");

                    let token = Token::Jmp(self.binary_to_decimal(immediate).unwrap());
                    results.push(token)

                },
                "jnc" => {
                    self.index += 1;
                    let immediate = self.opcodes.get(self.index).expect("jnc instruction error");

                    let token = Token::Jnc(self.binary_to_decimal(immediate).unwrap());
                    results.push(token)

                },
                "in" => {
                    self.index += 1;
                    let immediate = self.opcodes.get(self.index).expect("in instruction error");

                    let token = Token::In(
                        RegisterOp::from(immediate.to_string())
                    );

                    results.push(token)

                },
                "out" => {
                    self.index += 1;
                    let immediate = self.opcodes.get(self.index).expect("out instruction error");

                    if immediate == "B" {
                        let token = Token::OutB;
                        results.push(token)
                    } else {
                        let token = Token::OutIm(self.binary_to_decimal(immediate).unwrap());
                        results.push(token)
                    }
                },
                _ => {
                    panic!("invalid instruction")
                },
            }

            self.index += 1;
        }
        Ok(results)
    }

    fn binary_to_decimal(&self, text: &String) -> Result<u8, EmulatorErr> {
        let s = text.as_str();
        let decimal = u8::from_str_radix(s, 2);     // radix: cardinal number

        decimal
            .map_err(|_| EmulatorErr::new("From binary to decimal failed"))
    }
}



#[cfg(test)]
mod parser_tests {
    use crate::emulator::parser::Parser;

    #[test]
    fn parse_test() {
        let instructions = vec![
            "mov A 0001".to_string(),
            "add A 0001".to_string(),
        ];

        let mut code = Parser::new(instructions);
        let result = code.parse().unwrap();

        assert_eq!(result.len(), 2);
    }
}