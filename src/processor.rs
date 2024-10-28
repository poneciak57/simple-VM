use std::io::{self, Write};

use crate::{instruction::Instruction, interactive::{InteractiveCli, InteractiveState}, mem::MEM};

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

    pub fn run(&mut self, mem: &mut MEM, mut is: InteractiveState) {
        if is.is_enabled() {
            InteractiveCli::print_start_baner();
        }

        let mut breakpoints: Vec<usize> = Vec::new();

        loop {
            if is.is_waiting_bp() && breakpoints.contains(&(self.PC as usize)) {
                is = InteractiveState::ENABLED;
            }

            self.IR = mem.get(self.PC);
            self.PC += 1;

            if is.is_enabled() {
                loop {
                    unsafe {
                        println!("{}: {:?} {:?} {} AC={}", self.PC - 1, self.IR.inner.code(), self.IR.inner.adrt(), self.IR.inner.adr(), self.AC);
                    }
                    print!("> ");
                    io::stdout().flush().unwrap();
                    let cli = match InteractiveCli::read() {
                        Ok(cli) => cli,
                        Err(e) => {
                            println!("{}", e.render());
                            continue;
                        }
                    };
                    match cli.cmd {
                        crate::interactive::Command::NEXT => {
                            unsafe {
                                println!("{:<2}executing {}: {:?} {:?} {} AC={}", "", self.PC - 1, self.IR.inner.code(), self.IR.inner.adrt(), self.IR.inner.adr(), self.AC);
                            }
                            break;
                        },
                        crate::interactive::Command::NEXT_BREAKPOINT => {
                            is = InteractiveState::WAITING_FOR_BREAKPOINT;
                            println!("");
                            break;
                        },
                        crate::interactive::Command::ADD_BREAKPOINT { line } => breakpoints.push(line),
                        crate::interactive::Command::DELETE_BREAKPOINT { line } => breakpoints.retain(|&x| x != line),
                        crate::interactive::Command::LIST_BREAKPOINTS => {
                            println!("Current breakpoints:");
                            for i in breakpoints.iter() {
                                let ins = mem.get(*i as u16);
                                unsafe {
                                    println!(" - {}: {:?} {:?} {}", i, ins.inner.code(), ins.inner.adrt(), ins.inner.adr());
                                };
                            }
                        },
                        crate::interactive::Command::QUIT => return,
                        crate::interactive::Command::FINISH => {
                            is = InteractiveState::DISABLED;
                            break;
                        },
                        crate::interactive::Command::MEM { from, to, file } => {
                            if let Some(path) = file {
                                mem.write_to_file(path, from..to);
                            } else {
                                mem.print(from..to);
                            }
                        },
                    }
                    println!("");
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
                crate::instruction::Code::STORE => mem.set(self.OP as u16, self.AC as u16, is.is_enabled()),
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

            if is.is_enabled() {
                println!("{:<2}PC={}, AC={}", "", self.PC, self.AC);
                println!("");
            }
        }
        println!("{:<2}Finished program execution", "");
        mem.write_to_file("mem.out".to_string(), 0..512);
    }
}