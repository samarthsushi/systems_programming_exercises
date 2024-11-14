// assembly instructions for previously built smac0:

//     00 STOP:
//         stop or halt execution
//     01 ADD
//     02 SUB
//     03 MUL
//     04 MOVER:
//         move memory operand contents to register operand
//     05 MOVEM:
//         move register operand contents to memory
//     06 COMP:
//         compare register and memory operands to set condition code appropriately
//     07 BC:
//         branch to second operand depending on condition code as specified by first operand 
//     08 DIV
//     09 READ:
//         read into memory operand
//     10 PRINT:
//         print contents of memory operand
    // 11 DS:
    //     reserve n memory locations under some name
    
    // 12 DC:
    //     store some value in a memory location under some name

const OPCODETABLE: [OpcodeStr; 13] = [
    OpcodeStr { name: "STOP", code: 0 },
    OpcodeStr { name: "ADD", code: 1 },
    OpcodeStr { name: "SUB", code: 2 },
    OpcodeStr { name: "MUL", code: 3 },
    OpcodeStr { name: "MOVER", code: 4 },
    OpcodeStr { name: "MOVEM", code: 5 },
    OpcodeStr { name: "COMP", code: 6 },
    OpcodeStr { name: "BC", code: 7 },
    OpcodeStr { name: "DIV", code: 8 },
    OpcodeStr { name: "READ", code: 9 },
    OpcodeStr { name: "PRINT", code: 10 },
    OpcodeStr { name: "DS", code: 11 },
    OpcodeStr { name: "DC", code: 12 },
]; 

const REGISTERTABLE: [RegisterStr; 4] = [
    RegisterStr { name: "AREG", code: 0 },
    RegisterStr { name: "BREG", code: 1 },
    RegisterStr { name: "CREG", code: 2 },
    RegisterStr { name: "DREG", code: 3 },
];

const CONDITIONTABLE: [ConditionCodeStr; 6] = [
    ConditionCodeStr { name: "LT", code: 0 },
    ConditionCodeStr { name: "LE", code: 1 },
    ConditionCodeStr { name: "EQ", code: 2 },
    ConditionCodeStr { name: "GT", code: 3 },
    ConditionCodeStr { name: "GE", code: 4},
    ConditionCodeStr { name: "ANY", code: 5},
];

pub struct Symbol {
    pub name: String,
    pub address: usize,
    pub defined: bool,
    pub used: bool,
}

struct OpcodeStr {
    name: &'static str,
    code: usize,
}
impl From<&OpcodeStr> for Opcode {
    fn from(op_str: &OpcodeStr) -> Self {
        Opcode {
            name: op_str.name.to_string(),
            code: op_str.code,
        }
    }
}

struct RegisterStr {
    name: &'static str,
    code: usize,
}
impl From<&RegisterStr> for Register {
    fn from(reg_str: &RegisterStr) -> Self {
        Register {
            name: reg_str.name.to_string(),
            code: reg_str.code,
        }
    }
}

struct ConditionCodeStr {
    name: &'static str,
    code: usize,
}
impl From<&ConditionCodeStr> for ConditionCode {
    fn from(cond_str: &ConditionCodeStr) -> Self {
        ConditionCode {
            name: cond_str.name.to_string(),
            code: cond_str.code,
        }
    }
}

#[derive(Clone)]
struct Opcode {
    name: String,
    code: usize,
}

struct Register {
    name: String,
    code: usize,
}

struct ConditionCode {
    name: String,
    code: usize,
}

#[derive(Debug, PartialEq)]
pub enum ValueKind {
    Symbol,
    Constant,
}

pub struct IntermediateCode {
    pub address: usize,
    pub opcode: usize,
    pub reg: Option<usize>,
    pub kind: ValueKind,
    pub value: usize,
}

#[derive(Debug)]
enum ErrorType {
    InvalidValue,
    UnknownMnemonic,
    InvalidOperand,
    MissingLabel,
    MissingConditionCode,
    UndefinedSymbol(String)
}

pub struct Error {
    line_number: usize,
    error_type: ErrorType,
}

pub struct Assembler {
    pub symbol_table: Vec<Symbol>,
    opcode_table: Vec<Opcode>,
    register_table: Vec<Register>,
    condition_code_table: Vec<ConditionCode>,
    pub intermediate_code_table: Vec<IntermediateCode>,
    pub error_table: Vec<Error>,
    location_counter: usize,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            symbol_table: Vec::new(),
            opcode_table: OPCODETABLE.iter().map(|op_str| Opcode::from(op_str)).collect(),
            register_table: REGISTERTABLE.iter().map(|op_str| Register::from(op_str)).collect(),
            condition_code_table: CONDITIONTABLE.iter().map(|op_str| ConditionCode::from(op_str)).collect(),
            intermediate_code_table: Vec::new(),
            error_table: Vec::new(),
            location_counter: 0,
        }
    }

    pub fn pass1(&mut self, source_lines: &[String]) {
        for (line_number, line) in source_lines.iter().enumerate() {
            let mut tokens = line.split_whitespace();
            let mut label = None;
            let mut mnemonic = "";

            if let Some(token) = tokens.next() {
                if token.ends_with(":") {
                    // If the token ends with a colon, it is a label
                    label = Some(&token[..token.len() - 1]);
                    mnemonic = tokens.next().unwrap_or("");
                } else {
                    mnemonic = token;
                }
            }

            if let Some(label) = label {
                self.add_symbol_as_label(label.to_string());
            }

            if self.handle_start_and_end(mnemonic, &mut tokens) {
                continue;
            }

            if let Some(opcode_entry) = self.opcode_table.iter().find(|op| op.name == mnemonic) {
                self.process_opcode(opcode_entry.code, &mut tokens, line_number);
            } else {
                self.error_table.push(Error {
                    line_number,
                    error_type: ErrorType::UnknownMnemonic,
                });
            }
        }

        self.check_undefined_symbols();
    }

    fn handle_start_and_end(&mut self, mnemonic: &str, tokens: &mut std::str::SplitWhitespace) -> bool {
        match mnemonic {
            "START" => {
                if let Some(addr_str) = tokens.next() {
                    self.location_counter = addr_str.parse().expect("Invalid START address");
                }
                true
            }
            "END" => {
                true
            }
            _ => false
        }
    }

    fn process_opcode(&mut self, opcode_code: usize, tokens: &mut std::str::SplitWhitespace, line_number: usize) {
        match opcode_code {
            0 => self.generate_intermediate_code(0, None, ValueKind::Constant, 0),
            1 | 2 | 3 | 8 | 4 | 5 | 6 => {
                let (reg_code, kind, value) = self.process_operands(tokens);
                self.generate_intermediate_code(opcode_code, reg_code, kind, value);
            }
            9 | 10 => {
                if let Some(operand) = tokens.next() {
                    let value = self.add_symbol(operand.to_string());
                    self.generate_intermediate_code(opcode_code, None, ValueKind::Symbol, value);
                }
            }
            7 => {
                if let Some(cond_code) = tokens.next() {
                    let reg_code = self.condition_code_table.iter().find(|c| c.name == cond_code).map(|c| c.code);
                    
                    if let Some(label) = tokens.next() {
                        let value = self.add_symbol(label.to_string());
                        self.generate_intermediate_code(7, reg_code, ValueKind::Symbol, value);
                    } else {
                        self.error_table.push(Error {
                            line_number,
                            error_type: ErrorType::MissingLabel,
                        });
                    }
                } else {
                    self.error_table.push(Error {
                        line_number,
                        error_type: ErrorType::MissingConditionCode,
                    });
                }
            }
            11 => self.process_ds(tokens),
            12 => self.process_dc(tokens, line_number),
            _ => unreachable!()
        }
    }

    fn process_ds(&mut self, tokens: &mut std::str::SplitWhitespace) {
        if let Some(size_str) = tokens.next() {
            let size: usize = size_str.parse().expect("Invalid DS size");
            self.location_counter += size;
        }
    }

    fn process_dc(&mut self, tokens: &mut std::str::SplitWhitespace, line_number: usize) {
        if let Some(value_str) = tokens.next() {
            let value: usize = value_str.parse().expect("Invalid DC value");
            self.intermediate_code_table.push(IntermediateCode {
                address: self.location_counter,
                opcode: 12,
                reg: None,
                kind: ValueKind::Constant,
                value,
            });
            self.location_counter += 1;
        } else {
            self.error_table.push(Error {
                line_number,
                error_type: ErrorType::InvalidOperand,
            });
        }
    }

    fn add_symbol(&mut self, name: String) -> usize {
        if let Some(symbol) = self.symbol_table.iter_mut().find(|sym| sym.name == name) {
            symbol.used = true;
            symbol.address
        } else {
            self.symbol_table.push(Symbol {
                name: name.clone(),
                address: 0,
                defined: false,
                used: true,
            });
            self.symbol_table.len() - 1
        }
    }

    fn add_symbol_as_label(&mut self, name: String) -> usize {
        if let Some(symbol) = self.symbol_table.iter_mut().find(|sym| sym.name == name) {
            symbol.defined = true;
            symbol.address = self.location_counter;
            symbol.address
        } else {
            let address = self.location_counter;
            self.symbol_table.push(Symbol {
                name: name.clone(),
                address,
                defined: true,
                used: false,
            });
            address
        }
    }

    #[inline]
    fn process_operands(&mut self, tokens: &mut dyn Iterator<Item = &str>) -> (Option<usize>, ValueKind, usize) {
        let mut reg_code = None;
        let mut kind = ValueKind::Symbol;
        let mut value = 0;
    
        if let Some(register_str) = tokens.next() {
            if let Some(register) = self.register_table.iter().find(|r| r.name == register_str) {
                reg_code = Some(register.code);
            } else {
                self.error_table.push(Error {
                    line_number: self.location_counter, 
                    error_type: ErrorType::InvalidValue,
                });
                return (reg_code, kind, value); 
            }
        }

        if let Some(operand_str) = tokens.next() {
            if let Ok(constant_value) = operand_str.parse::<usize>() {
                kind = ValueKind::Constant;
                value = constant_value;
            } else {
                value = self.add_symbol(operand_str.to_string());
            }
        }
    
        (reg_code, kind, value)
    }

    fn generate_intermediate_code(&mut self, opcode: usize, reg: Option<usize>, kind: ValueKind, value: usize) {
        self.intermediate_code_table.push(IntermediateCode {
            address: self.location_counter,
            opcode,
            reg,
            kind,
            value,
        });
        self.location_counter += 1;
    }

    fn check_undefined_symbols(&mut self) {
        for symbol in &self.symbol_table {
            if symbol.used && !symbol.defined {
                self.error_table.push(Error {
                    line_number: 0, 
                    error_type: ErrorType::UndefinedSymbol(symbol.name.clone()),
                });
            }
        }
    }

    pub fn print_intermediate_code(&self) {
        println!("Intermediate Code Table:");
        for entry in &self.intermediate_code_table {
            println!(
                "Address: {}, Opcode: {}, Reg: {:?}, Kind: {:?}, Value: {}",
                entry.address, entry.opcode, entry.reg, entry.kind, entry.value
            );
        }
    }

    pub fn print_symbol_table(&self) {
        println!("Symbol Table:");
        for symbol in &self.symbol_table {
            println!(
                "Name: {}, Address: {}, Defined: {}, Used: {}",
                symbol.name, symbol.address, symbol.defined, symbol.used
            );
        }
    }

    pub fn print_error_table(&self) {
        println!("Error Table:");
        for symbol in &self.error_table {
            println!(
                "Line number: {}, Error type: {:?}",
                symbol.line_number, symbol.error_type
            );
        }
    }
}
