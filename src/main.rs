use std::collections::HashMap;
use std::env;
use std::fs;

enum Compare {
    EQ,
    NE,
    GT,
    LT,
    NONE,
}

struct Interpreter {
    reg: [i32; 16],
    mem: [i32; 256],
    line_num: usize,
    running: bool,
    cmp: Compare,
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
            line_num: 0,
            running: true,
            cmp: Compare::NONE,
            source: source_vec,
            jump_map: jump_hashmap,
        }
    }

    fn tick(&mut self) {
        let line = &self.source[self.line_num];
        if line.ends_with(':') {
            self.line_num += 1;
            return;
        }
        let mut line = line.split_whitespace();
        match line.next() {
            Some("LDR") => {
                let d = line
                    .next()
                    .unwrap()
                    .strip_prefix('R')
                    .expect("Invalid syntax")
                    .strip_suffix(',')
                    .expect("Invalid syntax")
                    .parse::<usize>()
                    .unwrap();
                let mem_ref = line.next().unwrap().parse::<usize>().unwrap();
                self.reg[d] = self.mem[mem_ref];
            }

            Some("STR") => {
                let d = line
                    .next()
                    .unwrap()
                    .strip_prefix('R')
                    .expect("Invalid syntax")
                    .strip_suffix(',')
                    .expect("Invalid syntax")
                    .parse::<usize>()
                    .unwrap();
                let mem_ref = line.next().unwrap().parse::<usize>().unwrap();
                self.mem[mem_ref] = self.reg[d];
            }

            Some("ADD") => {
                let d = line
                    .next()
                    .unwrap()
                    .strip_prefix('R')
                    .expect("Invalid syntax")
                    .strip_suffix(',')
                    .expect("Invalid syntax")
                    .parse::<usize>()
                    .unwrap();

                let n = line
                    .next()
                    .unwrap()
                    .strip_prefix('R')
                    .expect("Invalid syntax")
                    .strip_suffix(',')
                    .expect("Invalid syntax")
                    .parse::<usize>()
                    .unwrap();

                let operand2 = line.next().unwrap();

                if operand2.starts_with('#') {
                    self.reg[d] = self.reg[n]
                        + operand2
                            .strip_prefix('#')
                            .expect("hashtag")
                            .parse::<i32>()
                            .unwrap();
                } else if operand2.starts_with('R') {
                    self.reg[d] = self.reg[n]
                        + self.reg[operand2
                            .strip_prefix("R")
                            .expect("R")
                            .parse::<usize>()
                            .unwrap()];
                } else {
                    panic!("operand2 error");
                }
            }

            Some("SUB") => {
                let d = line
                    .next()
                    .unwrap()
                    .strip_prefix('R')
                    .expect("Invalid syntax")
                    .strip_suffix(',')
                    .expect("Invalid syntax")
                    .parse::<usize>()
                    .unwrap();

                let n = line
                    .next()
                    .unwrap()
                    .strip_prefix('R')
                    .expect("Invalid syntax")
                    .strip_suffix(',')
                    .expect("Invalid syntax")
                    .parse::<usize>()
                    .unwrap();

                let operand2 = line.next().unwrap();

                if operand2.starts_with('#') {
                    self.reg[d] = self.reg[n]
                        - operand2
                            .strip_prefix('#')
                            .expect("hashtag")
                            .parse::<i32>()
                            .unwrap();
                } else if operand2.starts_with('R') {
                    self.reg[d] = self.reg[n]
                        - self.reg[operand2
                            .strip_prefix("R")
                            .expect("R")
                            .parse::<usize>()
                            .unwrap()];
                } else {
                    panic!("operand2 error");
                }
            }

            Some("MOV") => (),
            Some("CMP") => (),

            Some("B") => {
                let label = line
                    .next()
                    .expect("Missing label")
                    .strip_suffix(':')
                    .unwrap()
                    .to_string();
                self.line_num = *self.jump_map.get(&label).unwrap();
            }

            Some("BEQ") => (),
            Some("BNE") => (),
            Some("BGT") => (),
            Some("BLT") => (),
            Some("AND") => (),
            Some("ORR") => (),
            Some("EOR") => (),
            Some("MVN") => (),
            Some("LSL") => (),
            Some("LSR") => (),
            Some("HALT") => {
                self.running = false;
            }
            Some(&_) => (),
            None => (),
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
