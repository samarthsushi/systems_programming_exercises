use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = r"C:\codes\systems_programming_exercises\assembler\test.asm";

    let source_lines = read_lines(file_path).expect("Failed to read file");

    let mut assembler = assembler::Assembler::new();
    assembler.pass1(&source_lines);

    assembler.print_intermediate_code();
    assembler.print_symbol_table();
    assembler.print_error_table();
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut result = Vec::new();
    for line in lines {
        result.push(line?);
    }
    Ok(result)
}
