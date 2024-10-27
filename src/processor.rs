use std::{fs::OpenOptions, io::{self}};
use std::io::Write;

use crate::instruction::{Code, Instruction};

pub struct PROC {
    pub(crate) IR: Instruction,
    pub(crate) PC: u16,
    pub(crate) AC: i16,
    pub(crate) OP: i16, 
}
impl PROC {
    pub fn new() -> Self {
        Self {
            IR: Instruction { raw: 0 },
            PC: 0,
            AC: 0,
            OP: 0
        }
    }

    pub fn run(&mut self, mem: &mut MEM, mut interactive: bool) {
        if interactive {
            println!("_________  ________   _   ____  ___");
            println!("| ___ \\  \\/  /  __ \\ | | | |  \\/  |");
            println!("| |_/ / .  . | /  \\/ | | | | .  . |");
            println!("|  __/| |\\/| | |     | | | | |\\/| |");
            println!("| |   | |  | | \\__/\\ \\ \\_/ / |  | |");
            println!("\\_|   \\_|  |_\\/____/  \\___/|_|  |_| by poneciak");
            println!("Welcome to interactive mode type 'h' for more info");
            println!("Every command is displayed before execution to execute visible step type 'n' ");
            println!("");
        }
        let mut mem_snap_c = 0;
        'main: loop {
            self.IR = mem.get(self.PC);
            self.PC += 1;
            if interactive {
                loop {
                    unsafe {
                        println!("{}: {:?} {:?} {} AC={}", self.PC - 1, self.IR.inner.code(), self.IR.inner.adrt(), self.IR.inner.adr(), self.AC);
                    }
                    print!("> ");
                    io::stdout().flush().unwrap();
                    let input = read_cmd();
                    match &*input {
                        "q" | "quit" => break 'main,
                        "n" | "next" => break, // continues execution
                        "m" => {
                            println!("{:<2}Created memory snapshot in 'mem{}.snap", "", mem_snap_c);
                            println!("");
                            mem_snap_c += 1;
                            mem.dump_all(Some(format!("mem{}.snap", mem_snap_c)));
                            continue;
                        }
                        "f" | "finish" => {
                            interactive = false;
                            break
                        }
                        "h" | "help" => {
                            println!("{:<2}h | help - print help message", "");
                            println!("{:<2}q | quit - quit interactive mode", "");
                            println!("{:<2}n | next - forward program execution", "");
                            println!("{:<2}f | finish - finish program execution without interactive mode", "");
                            println!("{:<2}m - makes snapshot of memory in mem.snap file", "");
                            println!("");
                        }
                        _ => println!("Unknown command. Press 'h' for help")
                    }
                }
                unsafe {
                    println!("{:<2}executing {}: {:?} {:?} {} AC={}", "", self.PC - 1, self.IR.inner.code(), self.IR.inner.adrt(), self.IR.inner.adr(), self.AC);
                }
            }
            match unsafe { self.IR.inner.adrt() } {
                crate::instruction::Adrt::DOT => self.OP = unsafe { self.IR.inner.adr_val() },
                crate::instruction::Adrt::AT => self.OP = unsafe { mem.get_raw(self.IR.inner.adr_val() as u16) as i16 },
                crate::instruction::Adrt::STAR => self.OP = unsafe { mem.get_raw(mem.get_raw(self.IR.inner.adr() as u16)) as i16 },
                crate::instruction::Adrt::PLUS => self.OP = unsafe { mem.get_raw((self.AC + self.IR.inner.adr_val()) as u16) as i16 },
            }
            match unsafe { self.IR.inner.code() } {
                crate::instruction::Code::NULL => continue,
                crate::instruction::Code::STOP => break,
                crate::instruction::Code::LOAD => self.AC = self.OP,
                crate::instruction::Code::STORE => mem.set(self.OP as u16, self.AC as u16, interactive),
                crate::instruction::Code::JUMP => self.PC = self.OP as u16,
                crate::instruction::Code::JNEG => if (self.AC as i16) < 0 { self.PC = self.OP as u16; },
                crate::instruction::Code::JZERO => if self.AC == 0 { self.PC = self.OP as u16; },
                crate::instruction::Code::ADD => self.AC += self.OP,
                crate::instruction::Code::SUB => self.AC -= self.OP,
                crate::instruction::Code::AND => self.AC &= self.OP,
                crate::instruction::Code::OR => self.AC |= self.OP,
                crate::instruction::Code::NOT => self.AC = !self.AC,
                crate::instruction::Code::XOR => self.AC ^= self.OP,
                crate::instruction::Code::SHL => self.AC = self.AC << self.OP,
                crate::instruction::Code::SHR => self.AC = self.AC >> self.OP,
            }
            if interactive {
                println!("{:<2}PC={}, AC={}", "", self.PC, self.AC);
                println!("");
            }
        }
    }
}

pub struct MEM {
    pub(crate) inner: [Instruction; 512],
}

impl MEM {

    pub(crate) fn new(inner: [Instruction; 512]) -> Self {
        Self { inner }
    }

    pub(crate) fn set(&mut self, i: u16, val: u16, interactive: bool) {
        if i >= 512 {
            panic!("Adress out of bounds. Tried to access memory at index: {}", i);
        }
        if interactive { println!("{:<2}Modified MEM[{}] = {}", "", i, val); }
        self.inner[i as usize] = Instruction { raw: val };    
    }

    pub(crate) fn get(&self, i: u16) -> Instruction {
        if i >= 512 {
            panic!("Adress out of bounds. Tried to access memory at index: {}", i);
        }
        self.inner[i as usize]
    }

    pub(crate) fn get_raw(&self, i: u16) -> u16 {
        unsafe { self.get(i).raw }
    }

    pub fn dump_all(&self, filename: Option<String>) {
        if let Some(name) = filename {
            let mut file = OpenOptions::new()
                .write(true)
                .append(false)
                .create(true)
                .open(name)
                .expect("Expected output file creation");
            for i in 0..512 {
                let binary_raw = format!("{:0>15b}", unsafe { self.inner[i as usize].raw } );
                if unsafe { self.inner[i as usize].inner.code() } == Code::NULL {
                    writeln!(file, "{:<3}\t{:>15}\t\t{}", i, binary_raw, unsafe { self.inner[i as usize].raw }).unwrap();
                } else {
                    unsafe { 
                        let inner = self.inner[i as usize].inner;
                        let sgn = if inner.sign() == 1 { '-' } else { '+' };
                        writeln!(file, "{:<3}\t{:>15}\t\t{} {:?} {:?} {}", i, binary_raw, sgn, inner.code(), inner.adrt(), inner.adr()).unwrap();
                    }
                }
            }
            return;
        }
        for i in 0..512 {
            println!("{} {:b}", i, unsafe { self.inner[i as usize].raw });
        }
    }

    pub fn print_range(&self, range: std::ops::Range<usize>) {
        for i in range {
            println!("{} {:?}", i, unsafe { self.inner[i as usize].inner });
        }
    }
}


fn read_cmd() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}