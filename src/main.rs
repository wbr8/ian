use std::collections::HashMap;
use std::env;
use std::fs;

struct Interpreter {
    reg: [i32; 16],
    mem: [i32; 256],
    line: usize,
    running: bool,
    source: Vec<String>,
    jump_map: HashMap<String, usize>,
}

impl Interpreter {
    fn new(source_string: String) -> Self {
        let mut source_vec: Vec<String> = vec![];
        for line in source_string.lines() {
            source_vec.push(line.to_string());
        }

        let jump_hashmap: HashMap<String, usize> = source_vec
            .iter()
            .enumerate()
            .filter(|(_, line)| line.ends_with(':'))
            .map(|(index, line)| (line.strip_suffix(':').unwrap().to_string(), index + 1))
            .collect();

        Self {
            reg: [0; 16],
            mem: [0; 256],
            line: 0,
            running: true,
            source: source_vec,
            jump_map: jump_hashmap,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("File {file_path}");

    let source = fs::read_to_string(file_path).expect("File read error");

    println!("With contents:\n{source}");

    let int = Interpreter::new(source);
    println!("{is:?}", is = int.source);
    println!("{jm:?}", jm = int.jump_map);
}
