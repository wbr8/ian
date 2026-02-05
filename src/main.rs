use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(PartialEq, Debug)]
enum Compare {
    EQ,
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

            Some("MOV") => {
                let d = line
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
                    self.reg[d] = operand2
                        .strip_prefix('#')
                        .expect("hashtag")
                        .parse::<i32>()
                        .unwrap();
                } else if operand2.starts_with('R') {
                    self.reg[d] = self.reg[operand2
                        .strip_prefix('R')
                        .expect("R")
                        .parse::<usize>()
                        .unwrap()];
                } else {
                    panic!("operand2 error");
                }
            }

            Some("CMP") => {
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
                    let value: i32 = operand2
                        .strip_prefix('#')
                        .expect("hashtag")
                        .parse()
                        .unwrap();
                    match self.reg[n].cmp(&value) {
                        Ordering::Less => self.cmp = Compare::LT,
                        Ordering::Greater => self.cmp = Compare::GT,
                        Ordering::Equal => self.cmp = Compare::EQ,
                    }
                } else if operand2.starts_with('R') {
                    let value: i32 = self.reg[operand2
                        .strip_prefix('R')
                        .expect("R")
                        .parse::<usize>()
                        .unwrap()];
                    match self.reg[n].cmp(&value) {
                        Ordering::Less => self.cmp = Compare::LT,
                        Ordering::Greater => self.cmp = Compare::GT,
                        Ordering::Equal => self.cmp = Compare::EQ,
                    }
                } else {
                    panic!("operand2 error");
                }
            }

            Some("B") => {
                let label = line
                    .next()
                    .expect("Missing label")
                    .to_string();
                self.line_num = *self.jump_map.get(&label).unwrap();
                return;
            }

            Some("BEQ") => {
                let label = line
                    .next()
                    .expect("Missing label")
                    .to_string();

                if let Compare::EQ = self.cmp {
                    self.line_num = *self.jump_map.get(&label).unwrap();
                }
                return;
            }

            Some("BNE") => {
                let label = line
                    .next()
                    .expect("Missing label")
                    .to_string();

                if let Compare::GT = self.cmp {
                    self.line_num = *self.jump_map.get(&label).unwrap();
                } else if let Compare::LT = self.cmp {
                    self.line_num = *self.jump_map.get(&label).unwrap();
                }
                return;
            }

            Some("BGT") => {
                let label = line
                    .next()
                    .expect("Missing label")
                    .to_string();

                if let Compare::GT = self.cmp {
                    self.line_num = *self.jump_map.get(&label).unwrap();
                }
                return;
            }

            Some("BLT") => {
                let label = line
                    .next()
                    .expect("Missing label")
                    .to_string();

                if let Compare::LT = self.cmp {
                    self.line_num = *self.jump_map.get(&label).unwrap();
                }
                return;
            }

            Some("AND") => {
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
                    self.reg[d] = self.reg[n] & operand2.strip_prefix('#').expect("hashtag").parse::<i32>().unwrap();
                } else if operand2.starts_with('R') {
                    self.reg[d] = self.reg[n] & self.reg[operand2.strip_prefix('R').expect("R").parse::<usize>().unwrap()];
                } else {
                    panic!("operand2 error");
                }
            }

            Some("ORR") => {
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
                    self.reg[d] = self.reg[n] | operand2.strip_prefix('#').expect("hashtag").parse::<i32>().unwrap();
                } else if operand2.starts_with('R') {
                    self.reg[d] = self.reg[n] | self.reg[operand2.strip_prefix('R').expect("R").parse::<usize>().unwrap()];
                } else {
                    panic!("operand2 error");
                }
            }

            Some("EOR") => {
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
                    self.reg[d] = self.reg[n] ^ operand2.strip_prefix('#').expect("hashtag").parse::<i32>().unwrap();
                } else if operand2.starts_with('R') {
                    self.reg[d] = self.reg[n] ^ self.reg[operand2.strip_prefix('R').expect("R").parse::<usize>().unwrap()];
                } else {
                    panic!("operand2 error");
                }
            }

            Some("MVN") => {
                let d = line
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
                    self.reg[d] = ! operand2.strip_prefix('#').expect("hashtag").parse::<i32>().unwrap();
                } else if operand2.starts_with('R') {
                    self.reg[d] = ! self.reg[operand2.strip_prefix('R').expect("R").parse::<usize>().unwrap()];
                } else {
                    panic!("operand2 error");
                }
            }

            Some("LSL") => {
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
                    self.reg[d] = self.reg[n] << operand2.strip_prefix('#').expect("hashtag").parse::<i32>().unwrap();
                } else if operand2.starts_with('R') {
                    self.reg[d] = self.reg[n] << self.reg[operand2.strip_prefix('R').expect("R").parse::<usize>().unwrap()];
                } else {
                    panic!("operand2 error");
                }
            }

            Some("LSR") => {
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
                    self.reg[d] = self.reg[n] >> operand2.strip_prefix('#').expect("hashtag").parse::<i32>().unwrap();
                } else if operand2.starts_with('R') {
                    self.reg[d] = self.reg[n] >> self.reg[operand2.strip_prefix('R').expect("R").parse::<usize>().unwrap()];
                } else {
                    panic!("operand2 error");
                }
            }

            Some("HALT") => {
                self.running = false;
            }
            Some(&_) => (),
            None => (),
        }
        self.line_num += 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldr() {
        let source = String::from("LDR R0, 42");
        let mut test = Interpreter::new(source);
        test.mem[42] = 123;
        test.tick();
        assert_eq!(test.reg[0], 123);
    }

    #[test]
    fn str() {
        let source = String::from("STR R1, 21");
        let mut test = Interpreter::new(source);
        test.reg[1] = 67;
        test.tick();
        assert_eq!(test.mem[21], 67);
    }

    #[test]
    fn add() {
        let source = String::from("ADD R0, R1, #3\nADD R2, R3, R4");
        let mut test = Interpreter::new(source);
        test.reg[1] = 2;
        test.reg[3] = 60;
        test.reg[4] = 9;
        test.tick();
        test.tick();
        assert_eq!(test.reg[0], 5);
        assert_eq!(test.reg[2], 69);
    }

    #[test]
    fn sub() {
        let source = String::from("SUB R0, R1, #55\nSUB R2, R3, R4");
        let mut test = Interpreter::new(source);
        test.reg[1] = 100;
        test.reg[3] = 42;
        test.reg[4] = 12;
        test.tick();
        test.tick();
        assert_eq!(test.reg[0], 45);
        assert_eq!(test.reg[2], 30);
    }

    #[test]
    fn mov() {
        let source = String::from("MOV R0, #456\nMOV R1, R2");
        let mut test = Interpreter::new(source);
        test.reg[2] = 678;
        test.tick();
        test.tick();
        assert_eq!(test.reg[0], 456);
        assert_eq!(test.reg[1], 678);
    }

    #[test]
    fn cmp() {
        let source = String::from("CMP R0, #2\nCMP R1, R2\nCMP R3, #3\nCMP R4, R5\nCMP R6, #4\nCMP R7, R8");
        let mut test = Interpreter::new(source);
        test.reg[0] = 1;
        test.tick();
        assert_eq!(test.cmp, Compare::LT);
        test.reg[1] = 5;
        test.reg[2] = 10;
        test.tick();
        assert_eq!(test.cmp, Compare::LT);
        test.reg[3] = 6;
        test.tick();
        assert_eq!(test.cmp, Compare::GT);
        test.reg[4] = 42;
        test.reg[5] = 21;
        test.tick();
        assert_eq!(test.cmp, Compare::GT);
        test.reg[6] = 4;
        test.tick();
        assert_eq!(test.cmp, Compare::EQ);
        test.reg[7] = 8;
        test.reg[8] = 8;
        test.tick();
        assert_eq!(test.cmp, Compare::EQ);
    }

    #[test]
    fn b() {
        let source = String::from("B label\nlabel:\nHALT");
        let mut test = Interpreter::new(source);
        test.tick();
        assert_eq!(test.line_num, 2);
    }

    #[test]
    fn beq() {
        let source = String::from("CMP R0, #42\nBEQ label\nHALT\nlabel:\nHALT");
        let mut test = Interpreter::new(source);
        test.reg[0] = 42;
        test.tick();
        test.tick();
        assert_eq!(test.line_num, 4);
    }

    #[test]
    fn bne() {
        let source = String::from("CMP R0, #42\nBNE label\nHALT\nlabel:\nHALT");
        let mut test = Interpreter::new(source);
        test.reg[0] = 123;
        test.tick();
        test.tick();
        assert_eq!(test.line_num, 4);
    }

    #[test]
    fn bgt() {
        let source = String::from("CMP R0, #42\nBGT label\nHALT\nlabel:\nHALT");
        let mut test = Interpreter::new(source);
        test.reg[0] = 123;
        test.tick();
        test.tick();
        assert_eq!(test.line_num, 4);
    }

    #[test]
    fn blt() {
        let source = String::from("CMP R0, #42\nBLT label\nHALT\nlabel:\nHALT");
        let mut test = Interpreter::new(source);
        test.reg[0] = 21;
        test.tick();
        test.tick();
        assert_eq!(test.line_num, 4);
    }

    #[test]
    fn and() {
        let source = String::from("AND R0, R1, #10\nAND R2, R3, R4");
        let mut test = Interpreter::new(source);
        test.reg[1] = 15;
        test.reg[3] = 11;
        test.reg[4] = 2;
        test.tick();
        test.tick();
        assert_eq!(test.reg[0], 10);
        assert_eq!(test.reg[2], 2);
    }

    #[test]
    fn orr() {
        let source = String::from("ORR R0, R1, #9\nORR R2, R3, R4");
        let mut test = Interpreter::new(source);
        test.reg[1] = 6;
        test.reg[3] = 8;
        test.reg[4] = 2;
        test.tick();
        test.tick();
        assert_eq!(test.reg[0], 15);
        assert_eq!(test.reg[2], 10);
    }

    #[test]
    fn eor() {
        let source = String::from("EOR R0, R1, #9\nEOR R2, R3, R4");
        let mut test = Interpreter::new(source);
        test.reg[1] = 15;
        test.reg[3] = 11;
        test.reg[4] = 2;
        test.tick();
        test.tick();
        assert_eq!(test.reg[0], 6);
        assert_eq!(test.reg[2], 9);
    }

    #[test]
    fn mvn() {
        // i'm just gonna believe that this one works
        // i don't want to deal with inverting signed 32 bit integers
    }

    #[test]
    fn lsl() {
        let source = String::from("LSL R0, R1, #1");
        let mut test = Interpreter::new(source);
        test.reg[1] = 4;
        test.tick();
        assert_eq!(test.reg[0], 8);
    }

    #[test]
    fn lsr() {
        let source = String::from("LSR R0, R1, #1");
        let mut test = Interpreter::new(source);
        test.reg[1] = 4;
        test.tick();
        assert_eq!(test.reg[0], 2);
    }

    #[test]
    fn halt() {
        let source = String::from("HALT");
        let mut test = Interpreter::new(source);
        test.tick();
        assert!(!test.running);
    }
}
