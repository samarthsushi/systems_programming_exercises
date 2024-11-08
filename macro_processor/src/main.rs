use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "";

    let source_lines = read_lines(file_path).expect("Failed to read file");

    let mut macro_processor = macro_processor::MacroProcessor::new();
    macro_processor.macro_process(&source_lines);
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