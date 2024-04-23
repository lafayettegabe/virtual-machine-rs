use std::io::{self, Write};

pub fn execute(memory: &mut [i32], operand: usize) {
    print!("Digite um valor: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    memory[operand] = input.trim().parse().unwrap();
}