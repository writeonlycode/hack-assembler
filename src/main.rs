use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let (filename, _) = path.split_once(".").expect("Error removing extension.");

    let program = fs::read_to_string(path).expect("Error reading file...");
    let result = parse_program(program);

    let filename = format!("{}.hack", filename);
    fs::write(filename, result).expect("Error writing file.");
}

fn comp_codes(keyword: &str) -> &str {
    match keyword {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",

        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => ""
    }
}

fn dest_codes(keyword: &str) -> &str {
    match keyword {
        "" => "000",
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "ADM" => "111",
        _ => ""
    }
}

fn jump_codes(keyword: &str) -> &str {
    match keyword {
        "" => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => ""
    }
}

fn parse_program(program: String) -> String {

    let mut symbols: HashMap<String, u16> = HashMap::from([
        ("R0".to_string(), 0),
        ("R1".to_string(), 1),
        ("R2".to_string(), 2),
        ("R3".to_string(), 3),
        ("R4".to_string(), 4),
        ("R5".to_string(), 5),
        ("R6".to_string(), 6),
        ("R7".to_string(), 7),
        ("R8".to_string(), 8),
        ("R9".to_string(), 9),
        ("R10".to_string(), 10),
        ("R11".to_string(), 11),
        ("R12".to_string(), 12),
        ("R13".to_string(), 13),
        ("R14".to_string(), 14),
        ("R15".to_string(), 15),

        ("SP".to_string(), 0),
        ("LCL".to_string(), 1),
        ("ARG".to_string(), 2),
        ("THIS".to_string(), 3),
        ("THAT".to_string(), 4),

        ("SCREEN".to_string(), 16384),
        ("KBD".to_string(), 24576)]);

    // 1st pass to resolve labels
    let mut current_line: u16 = 0;

    for line in program.lines() {
        let line = &remove_whitespace(line);
        let line = &remove_comment(line);

        if is_label(line) {
            parse_label(line, &mut symbols, current_line);
        } else if is_instruction(line) {
            current_line += 1;
        }
    }

    // 2nd pass to actually compile the program
    let mut result = String::new();
    let mut current_address: u16 = 16;

    for line in program.lines() {
        let line = &remove_whitespace(line);
        let line = &remove_comment(line);

        if is_instruction(line) {
            result.push_str(&parse_instruction(line, &mut symbols, &mut current_address));
        }
    }

    result
}

fn remove_whitespace(line: &str) -> String {
    line.replace(" ", "")
}

fn remove_comment(line: &str) -> String {
    match line.split_once("//") {
        Some((line, _)) => line.to_string(),
        None => line.to_string(),
    }
}

fn is_label(line: &str) -> bool {
    line.starts_with("(")
}

fn parse_label(line: &str, symbols: &mut HashMap<String, u16>, current_line: u16) {
    let line = line.trim_matches('(').trim_matches(')');
    symbols.insert(line.to_string(), current_line);
}

fn is_instruction(line: &str) -> bool {
    !line.starts_with("(") && !line.starts_with("//") && !line.is_empty()
}

fn parse_instruction(line: &str, symbols: &mut HashMap<String, u16>, current_address: &mut u16) -> String {
    if is_a_instruction(line) {
        parse_a_instruction(line, symbols, current_address)
    } else {
        parse_c_instruction(line)
    }
}

fn is_a_instruction(line: &str) -> bool {
    line.starts_with("@")
}

fn parse_a_instruction(line: &str, symbols: &mut HashMap<String, u16>, current_address: &mut u16) -> String {
    let value = line.strip_prefix("@").expect("Error converting A Instruction");

    let value: u16 = match value.parse() {
        Ok(v) => v,
        Err(_) => parse_variable(value, symbols, current_address)
    };

    format!("0{:015b}\n", value)
}

fn parse_variable(variable: &str, symbols: &mut HashMap<String, u16>, current_address: &mut u16) -> u16 {

    if !symbols.contains_key(variable) {
        symbols.insert(variable.to_string(), *current_address);
        *current_address += 1;
    }

    symbols[variable]
}

fn parse_c_instruction(line: &str) -> String {
    let (dest, comp_jump) = match line.split_once("=") {
        Some(value) => value,
        None => ("", line)
    };

    let (comp, jump) = match comp_jump.split_once(";") {
        Some(value) => value,
        None => (comp_jump, "")
    };

    format!("111{}{}{}\n", comp_codes(comp), dest_codes(dest), jump_codes(jump))
}
