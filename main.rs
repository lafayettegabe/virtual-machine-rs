mod machine;

use machine::Machine;

fn main() {
    let mut mv = Machine::new(128);

    // Set up memory with instructions...

    loop {
        mv.execute_next_instruction();
        if mv.pc >= mv.memory.len() {
            eprintln!("Erro: Fim inesperado do programa.");
            break;
        }
    }
}
