
pub struct Port {
    input: u8,
    output: u8,
}
impl Port {
    pub fn new(input: u8, output: u8) -> Self {
        Self { input, output }
    }

    pub fn input(&self) -> u8 {
        self.input
    }

    pub fn set_output(&mut self, immediate: u8) {
        self.output = immediate
    }

    pub fn output(&self) -> u8 {
        self.output
    }
}

pub struct Rom {
    pub memory_array: Vec<u8>,
}
impl Rom {
    pub fn new(memory_array: Vec<u8>) -> Self {
        Self { memory_array }
    }

    pub fn read(&self, pc: u8) -> u8 {
        self.memory_array[pc as usize]
    }

    pub fn size(&self) -> u8 {
        self.memory_array.len() as u8
    }
}