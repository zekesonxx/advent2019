use std::env;

#[derive(Debug, Clone)]
struct Intcode {
    /// List of instructions/memory
    list: Vec<usize>,
    /// Current position of the program
    /// (misusing the term "program counter")
    pc: usize
}

impl Intcode {
    pub fn new(program: Vec<usize>) -> Self {
        Intcode {
            list: program,
            pc: 0
        }
    }
    pub fn from_text(input: String) -> Self {
        let program = input.split(',').map(|s| s.parse().unwrap()).collect();
        Intcode {
            list: program,
            pc: 0
        }
    }
    pub fn state(&self) -> String {
        self.list.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",")
    }
    pub fn is_halted(&self) -> bool {
        self.list[self.pc*4] == 99
    }
    pub fn iterate(&mut self) -> bool {
        match self.list[self.pc] {
            99 => { return false; },
            1 => {
                let dest = self.list[self.pc+3];
                self.list[dest] = self.list[self.list[self.pc+1]] + self.list[self.list[self.pc+2]];
                self.pc += 4;
            },
            2 => {
                let dest = self.list[self.pc+3];
                self.list[dest] = self.list[self.list[self.pc+1]] * self.list[self.list[self.pc+2]];
                self.pc += 4;
            },
            _ => panic!("invalid opcode! {}", self.list[self.pc])
        }
        true
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("args: <program> [goal value]");
    } else if args.len() == 2 {
        let mut program = Intcode::from_text(args[1].to_string());
        println!("{}", program.state());
        while program.iterate() {
            println!("{}", program.state());
        }
    } else if args.len() == 3 {
        let goal = args[2].parse::<usize>().unwrap();
        let program = Intcode::from_text(args[1].to_string());
        let mut noun = 0;
        let mut verb = 0;
        let mut result = 0;
        loop {
            let mut program = program.clone();
            program.list[1] = noun;
            program.list[2] = verb;
            while program.iterate() {}
            result = program.list[0];
            if result != goal {
                noun+=1;
                if noun == 100 && verb == 99 {
                    println!("Failed to find anything");
                    return;
                }
                if noun == 100 { noun = 0; verb += 1; }
            } else {
                break;
            }
        }
        println!("{}", (noun*100)+verb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fuel_examples() {
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 966);
        assert_eq!(fuel(100756), 50346);
    }
}
