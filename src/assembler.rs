use std::{collections::HashMap, fs::read_to_string, vec};

use substring::Substring;

use crate::m6502;

#[derive(Debug)]
enum AddressingMode {
    Implied,
    // Relative, has special treatment in assembler.
    Immidiate,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    // Indirect, Special case for JMP, same as absolute for assembler.
    IndirectX,
    IndirectY,
}

pub struct Assembler;
impl Assembler {
    //returns binary and starting location
    pub fn assemble(program: &str) -> (Vec<u8>, u16) {
        //big spaghetti incoming (not recommended to read or try to understand):
        let mut program_location = "programs/".to_owned();
        program_location.push_str(program);
        program_location.push_str(".txt");
        let p = match read_to_string(program_location) {
            Ok(x) => x,
            Err(_) => {
                println!("Program does not exist");
                return (Vec::new(), 0);
            },
        };
        let (mut lex_codes, offset) = Lexer::lex(p).unwrap();
        //the key is the negative identefier for the label and the value is the
        let mut labeled: HashMap<i32, i32> = HashMap::new();
        //if a call to a label happend before instantiation the value of the label needs to be one less.

        let mut inter_vec = Vec::new();
        let mut location = 0;
        let mut branch = false;
        for code in lex_codes.iter_mut() {
            //if label instance.
            if *code < 0 && *code % 2 == 0 {
                labeled.insert(*code, location + offset);
                continue;
            }
            inter_vec.push(*code);
            location += 1;
        }

        let mut label_found = false;
        for (i, code) in inter_vec.iter_mut().enumerate() {
            let label_call = &(*code - 1); // the call will be the label that is found -1
            if labeled.contains_key(label_call) {
                *code = *labeled.get(label_call).unwrap();
                label_found = true;
            }
            if branch {
                if !label_found {
                    branch = false;
                    //if the code is not a label then it was a value and not a branch instrucion. This is temporart, because this solution
                    //makes it not possible to use values for branch instuctions, only labels.
                    continue;
                }
                if (*code - i as i32 - offset) > 0xff {
                    //make it jump to the value. Special case will be the negative minus 0xffff (0xffff can't be a value ever)
                    *code = *code - 0xffff * 2;
                    for (_, offset) in labeled.iter_mut() {
                        if *offset < i as i32 {
                            continue;
                        }
                        // the special branch adds 6 instructions. an offset is added.
                        *offset += 6;
                    }
                } else {
                    // minus 1 because it needs to jump over the last index.
                    *code -= i as i32 + 1 + offset;
                }
                branch = false;
            } else if Lexer::is_branch_instruction(code) {
                branch = true;
            }
            label_found = false;
        }

        branch = false;
        //if the label is above 0xff the value will be stored in the inter form. In the binary this will be separated into the to parts of that value.
        let mut overflow_value = 0;
        let mut overflow = false;
        let mut binary: Vec<u8> = Vec::new();
        for code in inter_vec.iter() {
            if *code >= 0 {
                if Lexer::is_branch_instruction(code) {
                    branch = true;
                }
                if overflow {
                    if *code != 0 {
                        panic!("overflow assembly with the second value not zero");
                    } else {
                        binary.push((overflow_value >> 8) as u8);
                    }
                    overflow = false;
                } else if *code > 0xff {
                    overflow_value = *code;
                    overflow = true;
                    binary.push((overflow_value & 0x00ff) as u8);
                } else {
                    binary.push(*code as u8);
                }
            } else if *code <= -0xffff {
                //means that the branch was more than a page and thus will be turned into a jump.
                let branch_loc = (*code + 0xffff * 2) as u16;
                let not_branch_loc = binary.len() as u16 + 7 + offset as u16; //plus 7 to skip second JMP

                //if the condition is met skip the the JMP over the branch (3 byte instruction)
                binary.push(3);
                binary.push(m6502::JMP_ABS);
                binary.push((not_branch_loc & 0x00ff) as u8);
                binary.push((not_branch_loc >> 8) as u8);
                //JMP to branch loc
                binary.push(m6502::JMP_ABS);
                binary.push((branch_loc & 0x00ff) as u8);
                binary.push((branch_loc >> 8) as u8);
            } else if branch {
                //this is to send the code even if its negative (for Branch back).
                binary.push(*code as u8);
            }
        }
        (binary, offset as u16)
    }
}

pub struct Lexer;
impl Lexer {
    //returns the lex codes and the offset (origin of the program) if error it will return the token that
    //caused the error (with an error message).
    pub fn lex(input: String) -> Result<(Vec<i32>, i32), String> {
        let mut tokens = Vec::new();
        let mut labels: HashMap<String, i32> = HashMap::new();
        let mut label_counter = -1;
        let mut start_offset = 0;

        for line in input.lines() {
            //identify if label:
            if !Self::labeld_line(line) {
                continue;
            }
            if line.starts_with(".") {
                if line.contains(".org") {
                    let value = line.substring(4, line.len()).trim();
                    start_offset = Lexer::value_reader(value, false, &labels);
                }
            } else if line.contains("=") {
                let mut name = String::new();
                let mut value_str = String::new();
                let mut start_of_value = false;
                for c in line.chars() {
                    if start_of_value {
                        value_str += &c.to_string();
                    } else if c == '=' {
                        start_of_value = true;
                    }
                    if !start_of_value {
                        name += &c.to_string();
                    }
                }
                name = name.trim().to_string();
                let value = Self::value_reader(&value_str.trim(), false, &labels);
                labels.insert(name, value);
            } else {
                let mut name = String::new();
                let mut fn_label_token = false;
                for c in line.chars() {
                    if c != ':' {
                        name += &c.to_string();
                    } else {
                        fn_label_token = true;
                        break;
                    }
                }
                if !fn_label_token {
                    return Err(format!("No ':' token at the end of {}", name));
                }
                labels.insert(name.trim().to_string(), label_counter);
                label_counter -= 2;
                //program memory.
            }
        }

        for line in input.lines() {
            // println!("{}", line.chars().nth(0).unwrap());
            // if line.chars().nth(0).unwrap() == '\n' || line.chars().nth(0).unwrap() == ';' {
            //     continue;
            // }
            //identify if label:
            if Self::labeld_line(line) {
                let mut name = String::new();
                let mut fn_label_token = false;
                for c in line.chars() {
                    if c != ':' {
                        name += &c.to_string();
                    } else {
                        fn_label_token = true;
                        break;
                    }
                }
                if !fn_label_token {
                    continue;
                }
                if labels.contains_key(&name) {
                    //to push the instanctiation of the label: (it is an even number)
                    tokens.push(*labels.get(&name).unwrap() - 1);
                }
                continue;
            }
            let mut args = line.trim().split(" ");
            let token = args.nth(0).unwrap();
            //tokenizer
            match token.to_ascii_uppercase().as_str() {
                "LDA" => {
                    //for some weired CPM evil reason unwrap pops the nth element
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Immidiate => {
                            tokens.push(m6502::LDA_IMM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::LDA_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageX => {
                            tokens.push(m6502::LDA_ZPX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::LDA_ABS);
                        }
                        AddressingMode::AbsoluteX => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::LDA_ABX);
                        }
                        AddressingMode::AbsoluteY => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::LDA_ABY);
                        }
                        AddressingMode::IndirectX => {
                            tokens.push(m6502::LDA_IDX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::IndirectY => {
                            tokens.push(m6502::LDA_IDY as i32);
                            tokens.push(digit);
                        }
                        _ => return Err("Inapropriate sytax for LDA".to_string()),
                    }
                }
                "ADC" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Immidiate => {
                            tokens.push(m6502::ADC_IMM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::ADC_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageX => {
                            tokens.push(m6502::ADC_ZPX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::ADC_ABS);
                        }
                        AddressingMode::AbsoluteX => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::ADC_ABX);
                        }
                        AddressingMode::AbsoluteY => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::ADC_ABY);
                        }
                        AddressingMode::IndirectX => {
                            tokens.push(m6502::ADC_IDX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::IndirectY => {
                            tokens.push(m6502::ADC_IDY as i32);
                            tokens.push(digit);
                        }
                        _ => return Err("Inapropriate sytax for ADC".to_string()),
                    }
                }
                "ASL" => {
                    let value = args.nth(0).unwrap_or("");
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Implied => { //it is accumalator addressing but it doesent need a its own type because it is the same as implied
                            tokens.push(m6502::ASL_A as i32);
                        }
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::ASL_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageX => {
                            tokens.push(m6502::ASL_ZPX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::ASL_ABS);
                        }
                        AddressingMode::AbsoluteX => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::ASL_ABX);
                        }
                        _ => return Err("Inapropriate sytax for ASL".to_string()),
                    }
                }
                "BCC" => {
                    let value = args.nth(0).unwrap();
                    tokens.push(m6502::BCC as i32);
                    tokens.push(Self::value_reader(value, true, &labels));
                }
                "BCS" => {
                    let value = args.nth(0).unwrap();
                    tokens.push(m6502::BCS as i32);
                    tokens.push(Self::value_reader(value, true, &labels));
                }
                "BEQ" => {
                    let value = args.nth(0).unwrap();
                    tokens.push(m6502::BEQ as i32);
                    tokens.push(Self::value_reader(value, true, &labels));
                }
                "BMI" => {
                    let value = args.nth(0).unwrap();
                    tokens.push(m6502::BMI as i32);
                    tokens.push(Self::value_reader(value, true, &labels));
                }
                "BNE" => {
                    let value = args.nth(0).unwrap();
                    tokens.push(m6502::BNE as i32);
                    tokens.push(Self::value_reader(value, true, &labels));
                }
                "BPL" => {
                    let value = args.nth(0).unwrap();
                    tokens.push(m6502::BPL as i32);
                    tokens.push(Self::value_reader(value, true, &labels));
                }
                "BRK" => {
                    tokens.push(m6502::BRK as i32);
                }
                "BVC" => {
                    let value = args.nth(0).unwrap();
                    tokens.push(m6502::BVC as i32);
                    tokens.push(Self::value_reader(value, true, &labels));
                    tokens.push(0);
                }
                "BVS" => {
                    let value = args.nth(0).unwrap();
                    tokens.push(m6502::BVS as i32);
                    tokens.push(Self::value_reader(value, true, &labels));
                }
                "CLC" => {
                    tokens.push(m6502::CLC as i32);
                }
                "CLD" => {
                    tokens.push(m6502::CLD as i32);
                }
                "CLI" => {
                    tokens.push(m6502::CLI as i32);
                }
                "CLV" => {
                    tokens.push(m6502::CLV as i32);
                }
                "CPM" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Immidiate => {
                            tokens.push(m6502::CPM_IMM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::CPM_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageX => {
                            tokens.push(m6502::CPM_ZPX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::CPM_ABS);
                        }
                        AddressingMode::AbsoluteX => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::CPM_ABX);
                        }
                        AddressingMode::AbsoluteY => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::CPM_ABY);
                        }
                        AddressingMode::IndirectX => {
                            tokens.push(m6502::CPM_IDX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::IndirectY => {
                            tokens.push(m6502::CPM_IDY as i32);
                            tokens.push(digit);
                        }
                        _ => return Err("Inapropriate sytax for CPM".to_string()),
                    }
                }
                "CPX" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Immidiate => {
                            tokens.push(m6502::CPX_IMM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::CPX_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::CPX_ABS)
                        }
                        _ => return Err("Inapropriate sytax for CPX".to_string()),
                    }
                }
                "DEX" => {
                    tokens.push(m6502::DEX as i32);
                }
                "DEY" => {
                    tokens.push(m6502::DEY as i32);
                }
                "EOR" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Immidiate => {
                            tokens.push(m6502::EOR_IMM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::EOR_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageX => {
                            tokens.push(m6502::EOR_ZPX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::EOR_ABS);
                        }
                        AddressingMode::AbsoluteX => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::EOR_ABX);
                        }
                        AddressingMode::AbsoluteY => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::EOR_ABY);
                        }
                        AddressingMode::IndirectX => {
                            tokens.push(m6502::EOR_IDX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::IndirectY => {
                            tokens.push(m6502::EOR_IDY as i32);
                            tokens.push(digit);
                        }
                        _ => return Err("Inapropriate sytax for EOR".to_string()),
                    }
                }
                "INX" => {
                    tokens.push(m6502::INX as i32);
                }
                "INY" => {
                    tokens.push(m6502::INY as i32);
                }
                "JMP" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        //if the address is smaller than 0xff it will be zeropage, JMP does not care about that.
                        AddressingMode::Absolute | AddressingMode::Zeropage => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::JMP_ABS);
                        }
                        //is indirect but for addressing_type algo it will look
                        //like IDY. Good enough for a special case.
                        AddressingMode::IndirectY => {
                            tokens.push(m6502::JMP_IND as i32);
                            tokens.push(digit);
                        }
                        _ => return Err("Inapropriate sytax for JMP".to_string()),
                    }
                }
                "JSR" => {
                    let value = args.nth(0).unwrap();
                    let digit = Self::value_reader(value, true, &labels);
                    Self::absolute_addressing(&mut tokens, digit, m6502::JSR);
                }
                "RTS" => {
                    tokens.push(m6502::RTS as i32);
                }
                "LDX" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Immidiate => {
                            tokens.push(m6502::LDX_IMM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::LDX_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageY => {
                            tokens.push(m6502::LDX_ZPY as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::LDX_ABS);
                        }
                        AddressingMode::AbsoluteX => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::LDX_ABY);
                        }
                        _ => return Err("Inapropriate sytax for LDX".to_string()),
                    }
                }
                "LDY" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Immidiate => {
                            tokens.push(m6502::LDY_IMM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::LDY_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageX => {
                            tokens.push(m6502::LDY_ZPX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::LDY_ABS);
                        }
                        AddressingMode::AbsoluteX => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::LDY_ABX);
                        }
                        _ => return Err("Inapropriate sytax for LDY".to_string()),
                    }
                }
                "LSR" => {
                    tokens.push(m6502::LSR as i32);
                }
                "NOP" => {
                    tokens.push(m6502::NOP as i32);
                }
                "ORA" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Immidiate => {
                            tokens.push(m6502::ORA_IMM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::ORA_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageX => {
                            tokens.push(m6502::ORA_ZPX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::ORA_ABS);
                        }
                        AddressingMode::AbsoluteX => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::ORA_ABX);
                        }
                        AddressingMode::AbsoluteY => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::ORA_ABY);
                        }
                        AddressingMode::IndirectX => {
                            tokens.push(m6502::ORA_IDX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::IndirectY => {
                            tokens.push(m6502::ORA_IDY as i32);
                            tokens.push(digit);
                        }
                        _ => return Err("Inapropriate sytax for ORA".to_string()),
                    }
                }
                "PHA" => {
                    tokens.push(m6502::PHA as i32);
                }
                "PLA" => {
                    tokens.push(m6502::PLA as i32)
                }
                "PHP" => {
                    tokens.push(m6502::PHP as i32);
                }
                "PLP" => {
                    tokens.push(m6502::PLP as i32);
                }
                "ROL" => {
                    tokens.push(m6502::ROL_A as i32);
                }
                "ROR" => {
                    tokens.push(m6502::ROR_A as i32);
                }
                "RTI" => {
                    tokens.push(m6502::RTI as i32);
                }
                "STA" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::STA_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageX => {
                            tokens.push(m6502::STA_ZPX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::STA_ABS);
                        }
                        AddressingMode::AbsoluteX => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::STA_ABX);
                        }
                        AddressingMode::AbsoluteY => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::STA_ABY);
                        }
                        AddressingMode::IndirectX => {
                            tokens.push(m6502::STA_IDX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::IndirectY => {
                            tokens.push(m6502::STA_IDY as i32);
                            tokens.push(digit);
                        }
                        _ => return Err("Inapropriate sytax for STA".to_string()),
                    }
                }
                "STX" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::STX_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageY => {
                            tokens.push(m6502::STX_ZPY as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::STX_ABS)
                        }
                        _ => return Err("Inapropriate sytax for STX".to_string()),
                    }
                }
                "STY" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::STY_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageX => {
                            tokens.push(m6502::STY_ZPX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::STY_ABS)
                        }
                        _ => return Err("Inapropriate sytax for STY".to_string()),
                    }
                }
                "SBC" => {
                    let value = args.nth(0).unwrap();
                    let (addressing_type, digit) = Self::addressing_type(value, &labels).unwrap();
                    match addressing_type {
                        AddressingMode::Immidiate => {
                            tokens.push(m6502::SBC_IMM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Zeropage => {
                            tokens.push(m6502::SBC_ZPM as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::ZeropageX => {
                            tokens.push(m6502::SBC_ZPX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::Absolute => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::SBC_ABS);
                        }
                        AddressingMode::AbsoluteX => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::SBC_ABX);
                        }
                        AddressingMode::AbsoluteY => {
                            Self::absolute_addressing(&mut tokens, digit, m6502::SBC_ABY);
                        }
                        AddressingMode::IndirectX => {
                            tokens.push(m6502::SBC_IDX as i32);
                            tokens.push(digit);
                        }
                        AddressingMode::IndirectY => {
                            tokens.push(m6502::SBC_IDY as i32);
                            tokens.push(digit);
                        }
                        _ => return Err("Inapropriate sytax for SBC".to_string()),
                    }
                }
                "SEC" => {
                    tokens.push(m6502::SEC as i32);
                }
                "SED" => {
                    tokens.push(m6502::SED as i32);
                }
                "SEI" => {
                    tokens.push(m6502::SEI as i32);
                }
                "TAX" => {
                    tokens.push(m6502::TAX as i32);
                }
                "TAY" => {
                    tokens.push(m6502::TAY as i32);
                }
                "TSX" => {
                    tokens.push(m6502::TSX as i32);
                }
                "TXA" => {
                    tokens.push(m6502::TXA as i32);
                }
                "TXS" => {
                    tokens.push(m6502::TXS as i32);
                }
                "TYA" => {
                    tokens.push(m6502::TYA as i32);
                }

                _ => return Err(format!("{} is an ivalid instruction", token)),
            }
        }
        // println!("{:?}", tokens);
        // println!("{:?}", labels);
        Ok((tokens, start_offset))
    }
    fn labeld_line(line: &str) -> bool {
        if line.starts_with(" ") || line.starts_with("\t") {
            return false;
        }
        true
    }

    fn value_reader(value: &str, labeled: bool, labels: &HashMap<String, i32>) -> i32 {
        let hex = value.starts_with("$");
        //clean if no other arguments (just value).
        if !labeled {
            if hex {
                let value_number = value.substring(1, value.len());
                return i32::from_str_radix(value_number, 16).unwrap();
            } else {
                return i32::from_str_radix(value, 10).unwrap();
            }
        } else {
            if labels.contains_key(value) {
                return *labels.get(value).unwrap();
            } else {
                if hex {
                    let value_number = value.substring(1, value.len());
                    return i32::from_str_radix(value_number, 16).unwrap();
                } else {
                    return i32::from_str_radix(value, 10).unwrap();
                }
            }
        }
    }
    fn addressing_type(
        value: &str,
        labels: &HashMap<String, i32>,
    ) -> Option<(AddressingMode, i32)> {
        if value.starts_with("#") {
            let digit = Self::value_reader(value.substring(1, value.len()), true, labels);
            return Some((AddressingMode::Immidiate, digit));
        }
        let zpm_absolute = Self::zeropage_absolute_type(value, labels);
        let indirect = Self::indirect_type(value, labels);
        if indirect.is_some() {
            return Some(indirect.unwrap());
        } else if zpm_absolute.is_some() {
            return Some(zpm_absolute.unwrap());
        } else {
            return Some((AddressingMode::Implied, 0));
        }
    }
    fn zeropage_absolute_type(
        value: &str,
        labels: &HashMap<String, i32>,
    ) -> Option<(AddressingMode, i32)> {
        let mut digit_str = String::new();
        let mut two_arg = false;
        let mut x = false;
        for c in value.chars() {
            if c == ',' {
                two_arg = true;
            } else if c == 'X' {
                x = true;
            } else if c != '(' && c != ')' && c != 'Y' {
                digit_str += &c.to_string();
            }
        }
        if digit_str.is_empty() {
            return None;
        }
        let digit = if labels.contains_key(&digit_str) {
            *labels.get(&digit_str).unwrap()
        } else {
            Self::value_reader(&digit_str, false, labels)
        };

        if digit <= 0xff {
            //zp
            if two_arg {
                if x {
                    return Some((AddressingMode::ZeropageX, digit));
                } else {
                    return Some((AddressingMode::ZeropageY, digit));
                }
            } else {
                return Some((AddressingMode::Zeropage, digit));
            }
        } else {
            //absolute
            if two_arg {
                if x {
                    return Some((AddressingMode::AbsoluteX, digit));
                } else {
                    return Some((AddressingMode::AbsoluteY, digit));
                }
            } else {
                return Some((AddressingMode::Absolute, digit));
            }
        }
    }
    fn indirect_type(value: &str, labels: &HashMap<String, i32>) -> Option<(AddressingMode, i32)> {
        let mut value_end = 0;
        let mut x = false;
        if !(value.starts_with("(")) {
            return None;
        }
        for (i, c) in value.chars().enumerate() {
            if c == ')' || c == ',' {
                value_end = i - 1;
            }
            if c == 'X' {
                x = true;
                break;
            }
        }
        let digit_str = value.substring(1, value_end);
        let digit = if labels.contains_key(digit_str) {
            *labels.get(digit_str).unwrap()
        } else {
            Self::value_reader(&digit_str, false, labels)
        };
        if digit > 0xff {
            panic!("Indirect input is bigger than a byte");
        }
        if x {
            return Some((AddressingMode::IndirectX, digit));
        } else {
            return Some((AddressingMode::IndirectY, digit));
        }
    }
    fn absolute_addressing(tokens: &mut Vec<i32>, value: i32, op_code: u8) {
        tokens.push(op_code as i32);
        if value < 0 {
            //if the value is negative bytes break. This solution is hopes that no more than 0xff labels are used :)
            tokens.push(-(-value & 0x00ff));
            tokens.push((-value) >> 8);
            return;
        }
        tokens.push(value & 0x00ff);
        tokens.push((value) >> 8);
    }
    fn is_branch_instruction(op_code: &i32) -> bool {
        match op_code {
            //all the branch op_codes
            0x90 | 0xb0 | 0xf0 | 0xd0 | 0x30 | 0x10 | 0x33 | 0x40 | 0x50 | 0x70 => return true,
            _ => return false,
        }
    }
}
