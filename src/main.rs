//Note about compiling the transpiled version:
//The compiling command should include the flag -O, so `rustup -O output.rs`.
//Otherwise, the code will error upon an (under/over)flow.

pub mod bf_interpreter;
pub mod bf_transcompilers;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // println!("Brainfuck: ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.\n");
    // bf_interpreter::interpret("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string());
    // bf_interpreter::interpret(include_str!("hello_world.txt").to_string());
    // let translated = bf_interpreter::translate_rust("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string());
    let mut extensions: Vec<bf_transcompilers::Extension> = Vec::new();
    let test_extension = bf_transcompilers::Extension {
        character: '#',
        filename: "extensions/test.rs",
        function_callback: "print_hello()"
    };
    extensions.push(test_extension);
    let translated = bf_transcompilers::translate_rust(include_str!("hello_world.bf").to_string(), &extensions);
    // println!("{}", translated);

    let path = Path::new("output.rs");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(translated.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
