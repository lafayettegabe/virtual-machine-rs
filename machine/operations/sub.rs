pub fn execute(ac: &mut i32, memory: &[i32], operand: usize) {
    *ac -= memory[operand];
}
