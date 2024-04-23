pub fn execute(memory: &mut [i32], operand: usize, ac: i32) {
    memory[operand] = ac;
}