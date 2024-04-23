pub fn execute(pc: &mut usize, operand: usize, ac: i32) {
    if ac == 0 {
        *pc = operand;
    }
}