use atty::Stream;
use evm_bunnyhop::bunny_hop;
use std::io::{self, Read};

fn main() {
    if atty::is(Stream::Stdin) {
        println!("EVM Bunny Hop is a small tool to convert PUSH2 jumps to PUSH1\n");
        println!("Usage: huffc path/to/contract.huff --bytecode | evm-bunnyhop");
    } else {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read from stdin");

        let bunny_hopped = if let Some(start_index) = input.find("3d393df3") {
            let runtime_code_index = start_index + 8;
            bunny_hop(&input.trim()[runtime_code_index..])
        } else {
            bunny_hop(input.trim())
        };
        println!("{}", bunny_hopped);
    }
}
