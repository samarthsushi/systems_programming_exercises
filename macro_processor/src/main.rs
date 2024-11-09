use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let input_file_path = r"C:\codes\systems_programming_exercises\macro_processor\data\input.asm";
    let output_file_path = r"C:\codes\systems_programming_exercises\macro_processor\data\output.asm";

    let source_lines = read_lines(input_file_path).expect("Failed to read file");

    let mut macro_processor = macro_processor::MacroProcessor::new();
    let expanded_asm = macro_processor.macro_process(&source_lines);

    append_vec_to_file(output_file_path, expanded_asm)?;
    Ok(())
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

fn append_vec_to_file(filename: &str, lines: Vec<String>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?; 
    for line in lines {
        writeln!(file, "{}", line)?; 
    }
    Ok(())
}