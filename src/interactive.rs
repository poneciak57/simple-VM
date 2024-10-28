use std::io;

use clap::{Error, Parser, Subcommand};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InteractiveState {
    DISABLED,
    ENABLED,
    WAITING_FOR_BREAKPOINT,
}

impl InteractiveState {
    pub fn is_enabled(&self) -> bool { return *self == Self::ENABLED }
    pub fn is_disabled(&self) -> bool { return *self == Self::DISABLED }
    pub fn is_waiting_bp(&self) -> bool { return *self == Self::WAITING_FOR_BREAKPOINT }
}

#[derive(Parser, Debug)]
#[command(name = ">", about = "An interactive debuging tool", long_about = None)]
pub struct InteractiveCli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Proceed to the next instruction (alias: n)
    #[command(alias = "n")]
    NEXT,

    /// Continue until the next breakpoint (alias: nb)
    #[command(alias = "nb")] 
    NEXT_BREAKPOINT,

    /// Add a breakpoint at a specified line (alias: ab)
    #[command(alias = "ab")]
    ADD_BREAKPOINT {
        /// Line number for the breakpoint
        line: usize,
    },

    /// Delete a breakpoint at a specified line if exists (alias: db)
    #[command(alias = "db")]
    DELETE_BREAKPOINT {
        /// Line number of the breakpoint to delete
        line: usize,
    },

    /// List all breakpoints (alias: lb)
    #[command(alias = "lb")]
    LIST_BREAKPOINTS,

    /// Aborts execution (alias: q)
    #[command(alias = "q")]
    QUIT,

    /// Disables interactive mode and finishes execution (alias: f)
    #[command(alias = "f")]
    FINISH,

    /// Print memory from a start address to an end address (alias: m)
    #[command(alias = "m")]
    MEM {
        /// Inclusive start address of memory
        #[arg(short = 'f', long = "from", short, default_value_t = 0)]
        from: usize,
        /// Exclusive end address of memory
        #[arg(short, long, default_value_t = 512)]
        to: usize,
        /// File where u want to store a snapshot
        #[arg(long = "file")]
        file: Option<String>
    },
}

impl InteractiveCli {

    pub fn read() -> Result<InteractiveCli, Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut args: Vec<&str> = input.trim().split_whitespace().collect();
        args.insert(0, "");
        InteractiveCli::try_parse_from(args)
    }

    pub fn print_start_baner() {
        println!("_________  ________   _   ____  ___");
        println!("| ___ \\  \\/  /  __ \\ | | | |  \\/  |");
        println!("| |_/ / .  . | /  \\/ | | | | .  . |");
        println!("|  __/| |\\/| | |     | | | | |\\/| |");
        println!("| |   | |  | | \\__/\\ \\ \\_/ / |  | |");
        println!("\\_|   \\_|  |_\\/____/  \\___/|_|  |_| by poneciak");
        println!("Welcome to interactive mode type 'help' for more info");
        println!("Every instruction is displayed and then you can type your command");
        println!("");
    }
}