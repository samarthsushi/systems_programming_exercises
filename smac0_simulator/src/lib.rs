#[warn(unused_assignments)]

use std::fs;
use std::io::{self, Write};
use std::process;

pub struct SMAC0 {
    memory: [usize; 1000],
    registers: [usize; 4],
    condition_codes: [bool; 6],
    program_counter: usize,
    last_logical_addr: usize,
}

impl SMAC0 {
    pub fn new() -> Self{
        Self {
            memory: [0; 1000],
            registers: [0; 4],
            condition_codes: [false; 6],
            program_counter: 0,
            last_logical_addr: 0,
        }
    }

    pub fn process_input() -> Result<String, io::Error> {
        print!("? ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    pub fn parse_file(&mut self, contents: String) {
        let lines: Vec<&str> = contents.lines().collect();

        for line in lines {
            if line.starts_with("-1") {
                self.program_counter = line[3..=5].parse::<usize>().unwrap();
            } else {
                let addr = line[..=2].parse::<usize>().unwrap();
                self.last_logical_addr = addr;
                self.memory[addr] = line[4..].parse::<usize>().unwrap();
            }
        }
    }

    pub fn load_program(&mut self, filename: &str) {
        let contents = fs::read_to_string(filename)
            .expect("should have been able to read the file");
        self.parse_file(contents);
    }

    pub fn print_loaded_program(&self) {
        for i in self.program_counter..=self.last_logical_addr {
            println!("{}", &self.memory[i]);
        }
    }

    pub fn execute_line(&mut self) -> Result<&'static str, Box<dyn std::error::Error>> {
        let mem_str = self.memory[self.program_counter].to_string();
        let (mut opcode, mut register_op, mut mem_op) = (0, 0, 0);

        if mem_str.len() == 1 {
            return Ok("break");
        }

        if mem_str.len() == 6 {
            (opcode, register_op, mem_op) = ((&mem_str[..=1]).parse::<usize>()?, (&mem_str[2..=2]).parse::<usize>()?, (&mem_str[3..=5]).parse::<usize>()?);
        } else {
            (opcode, register_op, mem_op) = ((&mem_str[..=0]).parse::<usize>()?, (&mem_str[1..=1]).parse::<usize>()?, (&mem_str[2..=4]).parse::<usize>()?);
        }

        match opcode {
            0 => return Ok("break"),
            1 => self.registers[register_op] += self.memory[mem_op],
            2 => self.registers[register_op] -= self.memory[mem_op],
            3 => self.registers[register_op] *= self.memory[mem_op],
            8 => self.registers[register_op] /= self.memory[mem_op],
            4 => self.registers[register_op] = self.memory[mem_op],
            5 => self.memory[mem_op] = self.registers[register_op],
            6 => {
                self.condition_codes[0] = self.registers[register_op] <  self.memory[mem_op];
                self.condition_codes[1] = self.registers[register_op] <= self.memory[mem_op];
                self.condition_codes[2] = self.registers[register_op] == self.memory[mem_op];
                self.condition_codes[3] = self.registers[register_op] >  self.memory[mem_op];
                self.condition_codes[4] = self.registers[register_op] >= self.memory[mem_op];
                self.condition_codes[5] = true;
            },
            7 => {
                if self.condition_codes[register_op] || register_op == 5 {
                    self.program_counter = mem_op;
                    return Ok("continue");
                }
            },
            9 => {
                println!("taking input for mem block {mem_op}:");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input_int = input.trim().parse::<usize>()?;
                self.memory[mem_op] = input_int;
            },
            10 => println!("printing: {}", self.memory[mem_op]),
            _ => return Err("invalid opcode".into())
        }
        self.program_counter += 1;
        Ok("full cycle done")
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while self.program_counter < self.last_logical_addr {
            match self.execute_line()? {
                "full cycle done" | "continue" => {},
                "break" => break,
                _ => {},
            }
        }
        Ok(())
    }

    pub fn trace(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("program_counter: {}, last_logical_addr: {}", self.program_counter, self.last_logical_addr);
        while self.program_counter < self.last_logical_addr {
            println!("program_counter: {}, registers: {:?}, condition codes: {:?}", self.program_counter, self.registers, self.condition_codes);
            match self.execute_line()? {
                "full cycle done" | "continue" => {},
                "break" => break,
                _ => {},
            }
        }
        Ok(())
    }

    pub fn smac0_run(&mut self) {
        loop {
            let input = match Self::process_input() {
                Ok(input) => input.trim().to_string(),
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(1);
                }
            };

            let mut args = input.split_whitespace();

            match args.next() {
                Some("load") => {
                    if let Some(filename) = args.next() {
                        self.load_program(filename);
                    } else {
                        eprintln!("Filename not provided.");
                    }
                },
                Some("print") => {
                    self.print_loaded_program();
                },
                Some("run") => {
                    if let Err(e) = self.execute() {
                        println!("{e:?}");
                    }
                },
                Some("trace") => {
                    if let Err(e) = self.trace() {
                        println!("{e:?}");
                    }
                },
                Some("quit") => break,
                _ => continue,
            }
        }
    }

}