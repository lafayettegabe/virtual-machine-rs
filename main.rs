mod machine;

use machine::Machine;

fn main() {
    let mut memory = vec![
        200, // LA op (Load accumulator with value at memory location 0)
        201, // LA op (Load accumulator with value at memory location 1)
        202, // AA op (Add value at memory location 1 to accumulator)
        210, // PW op (Print word at memory location 0)
        211, // PW op (Print word at memory location 1)
    ];

    memory[0] = 10;
    memory[1] = 20;

    let mut vm = Machine::new(memory.len());
    vm.memory.copy_from_slice(&memory);

    loop {
        vm.execute_next_instruction();
        if vm.pc >= vm.memory.len() {
            break;
        }
    }
}