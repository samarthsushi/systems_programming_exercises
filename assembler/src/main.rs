use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = r"C:\codes\systems_programming_exercises\assembler\data\test.asm";
    let source_lines: Vec<String> = read_lines(file_path).expect("Failed to read file");

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
    let result: Vec<String> = lines
        .filter_map(|line| line.ok()) 
        .map(|line| line.trim().to_string()) 
        .filter(|line| !line.is_empty()) 
        .collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assembler::{ValueKind, IntermediateCode};

    #[test]
    fn it_works() {
        let source_lines: Vec<String> = r#"
START 300
BEGIN: READ NUM
LOOP: MOVEM AREG NUM
PRINT NUM
MUL AREG NUM
COMP AREG HUNDRED
BC LT LOOP
STOP
NUM: DS 2
HUNDRED: DC 100
END
"#
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect();
        let mut assembler = assembler::Assembler::new();
        assembler.pass1(&source_lines);

        assert!(assembler.error_table.is_empty());

        assert_eq!(assembler.symbol_table.len(), 4);

        assert_eq!(assembler.symbol_table[0].name, "BEGIN");
        assert_eq!(assembler.symbol_table[0].address, 300);
        assert!(assembler.symbol_table[0].defined);
        assert!(!assembler.symbol_table[0].used);

        assert_eq!(assembler.symbol_table[1].name, "NUM");
        assert_eq!(assembler.symbol_table[1].address, 307);
        assert!(assembler.symbol_table[1].defined);
        assert!(assembler.symbol_table[1].used);

        assert_eq!(assembler.symbol_table[2].name, "LOOP");
        assert_eq!(assembler.symbol_table[2].address, 301);
        assert!(assembler.symbol_table[2].defined);
        assert!(assembler.symbol_table[2].used);

        assert_eq!(assembler.symbol_table[3].name, "HUNDRED");
        assert_eq!(assembler.symbol_table[3].address, 309);
        assert!(assembler.symbol_table[3].defined);
        assert!(assembler.symbol_table[3].used);

        assert_eq!(assembler.intermediate_code_table.len(), 8);

        let expected = vec![
            IntermediateCode { address: 300, opcode: 9, reg: None, kind: ValueKind::Symbol, value: 1 },
            IntermediateCode { address: 301, opcode: 5, reg: Some(0), kind: ValueKind::Symbol, value: 0 },
            IntermediateCode { address: 302, opcode: 10, reg: None, kind: ValueKind::Symbol, value: 0 },
            IntermediateCode { address: 303, opcode: 3, reg: Some(0), kind: ValueKind::Symbol, value: 0 },
            IntermediateCode { address: 304, opcode: 6, reg: Some(0), kind: ValueKind::Symbol, value: 3 },
            IntermediateCode { address: 305, opcode: 7, reg: Some(0), kind: ValueKind::Symbol, value: 301 },
            IntermediateCode { address: 306, opcode: 0, reg: None, kind: ValueKind::Constant, value: 0 },
            IntermediateCode { address: 309, opcode: 12, reg: None, kind: ValueKind::Constant, value: 100 },
        ];

        for (entry, expected_entry) in assembler.intermediate_code_table.iter().zip(expected.iter()) {
            assert_eq!(entry.address, expected_entry.address);
            assert_eq!(entry.opcode, expected_entry.opcode);
            assert_eq!(entry.reg, expected_entry.reg);
            assert_eq!(entry.kind, expected_entry.kind);
            assert_eq!(entry.value, expected_entry.value);
        }
    }
}
