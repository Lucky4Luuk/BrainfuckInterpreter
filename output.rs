//Rust bf code. Generated by Lucky4Luuk's BF to Rust transpiler.
use std::io::Read;

fn main() {
    let mut memory: [u8; 65536] = [0; 65536];
    let mut data_ptr: u8 = 0;

let input: Option<u8> = std::io::stdin().bytes().next().and_then(|result| result.ok());
memory[data_ptr as usize] = match input {
                Some(x) => x,
                None => 0,
            };
    print!("{}", memory[data_ptr as usize] as char);
}