pub fn execute(ac: &mut i32, memory: &[i32], operand: usize) {
    if memory[operand] != 0 {
        *ac /= memory[operand];
    } else {
        eprintln!("Erro: Divisão por zero");
    }
}