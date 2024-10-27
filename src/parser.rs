use crate::{instruction::{Adrt, Code, InnerInstruction, Instruction}, processor::MEM};



pub fn parse(s: &str) -> Box<MEM> {
    let mut tab: [Instruction; 512] = [Instruction { raw: 0 }; 512];
    for line in s.lines() {
        if line.trim().is_empty() {
            continue;
        }
        // parsing index
        let mut chars = line.chars().peekable();
        
        // Parsing index
        let mut index_str = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_digit(10) {
                index_str.push(c);
                chars.next(); // Consume the character
            } else {
                break;
            }
        }
        let index: i32 = index_str.parse().expect("Expected integer index");
        
        // Assert and skip the colon
        assert_eq!(chars.next(), Some(':'));
        
        // Skip whitespace after colon
        while chars.peek() == Some(&' ') {
            chars.next();
        }

        // If we are provided with only number
        if let Some(&c) = chars.peek() {
            if c.is_digit(10) || c == '-' {  // Include negative numbers
                let mut number_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '-' {
                        number_str.push(c);
                        chars.next(); // Consume the character
                    } else {
                        break;
                    }
                }
    
                // We have parsed a number, now we need to set it
                let number: i16 = number_str.parse().expect("Expected integer number");
                tab[index as usize] = Instruction { raw: number as u16 };
                continue; // Skip further processing for this line
            }
        }

        // Parsing command
        let mut command = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_alphabetic() {
                command.push(c);
                chars.next(); // Consume the character
            } else {
                break;
            }
        }
        
        // Skip whitespace after command
        while chars.peek() == Some(&' ') {
            chars.next();
        }

        // Parsing character
        let adrt_char = chars.next().expect("Expected single character");
        
        // Skip whitespace after character
        while chars.peek() == Some(&' ') {
            chars.next();
        }

        // Parsing number
        let mut number_str = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_digit(10) || c == '-' {
                number_str.push(c);
                chars.next(); // Consume the character
            } else {
                break;
            }
        }
        let mut number: i32 = number_str.parse().expect("Expected integer number");

        // Matching adrt
        let adrt = match adrt_char {
            '.' => Adrt::DOT,
            '@' => Adrt::AT,
            '*' => Adrt::STAR,
            '+' => Adrt::PLUS,
            _ => panic!("Unexpected ADRT")
        };
        // Matching code
        let code = match &*command {
            "NULL" => Code::NULL,
            "STOP" => Code::STOP,
            "LOAD" => Code::LOAD,
            "STORE" => Code::STORE,
            "JUMP" => Code::JUMP,
            "JNEG" => Code::JNEG,
            "JZERO" => Code::JZERO,
            "ADD" => Code::ADD,
            "SUB" => Code::SUB,
            "AND" => Code::AND,
            "OR" => Code::OR,
            "NOT" => Code::NOT,
            "XOR" => Code::XOR,
            "SHL" => Code::SHL,
            "SHR" => Code::SHR,
            _ => panic!("Unexpected COMMAND")
        };
        let sign: u16 = if number < 0 { 
            number *= -1;
            1 
        } else { 0 };
        let mut ii = InnerInstruction::new();
        ii.set_sign(sign);
        ii.set_adrt(adrt);
        ii.set_code(code);
        ii.set_adr(number as u16);
        tab[index as usize] = Instruction { inner: ii };
    }
    return Box::new(MEM::new(tab));
}