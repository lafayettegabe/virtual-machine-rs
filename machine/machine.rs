use std::io::{self, Write};

pub struct Machine {
    pub memory: Vec<i32>,       // Memória
    pub pc: usize,              // Registrador Program Counter
    pub ac: i32,                // Acumulador
}

impl Machine {
    pub fn new(memory_size: usize) -> Machine {
        Machine {
            memory: vec![0; memory_size],
            pc: 0,
            ac: 0,
        }
    }

    pub fn execute_next_instruction(&mut self) {
        let instruction = self.memory[self.pc];
        let opcode = instruction / 100;
        let operand = (instruction % 100) as usize;

        match opcode {
            0 => operations::load::execute(&mut self.ac, &self.memory, operand),    // LA: Load accumulator
            1 => operations::store::execute(&mut self.memory, operand, self.ac),    // SA: Store accumulator
            2 => operations::add::execute(&mut self.ac, &self.memory, operand),     // AA: Add to accumulator
            3 => operations::mul::execute(&mut self.ac, &self.memory, operand),     // MUL: Multiply accumulator
            4 => operations::div::execute(&mut self.ac, &self.memory, operand),     // DIV: Divide accumulator
            5 => operations::sub::execute(&mut self.ac, &self.memory, operand),     // SUB: Subtract from accumulator
            6 => operations::jmp::execute(&mut self.pc, operand),                   // JMP: Jump
            7 => operations::jeq::execute(&mut self.pc, operand, self.ac),          // JEQ: Jump if equal
            8 => operations::jgt::execute(&mut self.pc, operand, self.ac),          // JGT: Jump if greater than
            9 => operations::jlt::execute(&mut self.pc, operand, self.ac),          // JLT: Jump if less than
            10 => operations::print_word::execute(&self.memory, operand),           // PW: Print Word
            11 => operations::read_word::execute(&mut self.memory, operand),        // RW: Read Word
            12 => operations::stop::execute(),                                      // STOP: Stop machine
            _ => panic!("Erro: Opcode inválido."),
        }

        self.pc += 1;
    }
}

pub mod operations;
