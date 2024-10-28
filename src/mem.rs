use std::fs::OpenOptions;
use std::io::{self, Write};


use crate::instruction::Instruction;

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


    pub fn write_to_file(&self, filename: String, range: std::ops::Range<usize>) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(&filename)
            .expect("Expected output file creation");
        self.print_table(&mut file, range)
            .expect("Expected successfull write to file");
        println!("{:<2}Written memory snapshot to {}", "", filename);
        return;
    }

    pub fn print(&self, range: std::ops::Range<usize>) {
        self.print_table(&mut io::stdout(), range)
            .expect("Expected successfull print to stdout");
    }

    fn print_table<W: Write>(&self, out: &mut W, range: std::ops::Range<usize>) -> io::Result<()> {
        let headers = ["ID", "Binary repr", "Instruction repr", "Number repr"];
        let col_widths = [4, 17, 20, 15];

        // headers
        for (header, &width) in headers.iter().zip(&col_widths) {
            write!(out, "{:<width$} ", header, width = width)?;
        }
        writeln!(out)?;

        // separator line
        for &width in &col_widths {
            write!(out, "{:-<width$}", "", width = width)?;
        }
        writeln!(out)?;

        // rows
        for i in range {
            let row: [String; 4];
            unsafe {
                let inner = self.inner[i as usize].inner;
                let sgn = if inner.sign() == 1 { '-' } else { '+' };

                let binary_raw = format!("{:0>15b}", self.inner[i as usize].raw);
                let instruction = format!("{} {:?} {:?} {}", sgn, inner.code(), inner.adrt(), inner.adr());
                let number = format!("{}", self.inner[i as usize].raw);
                row = [
                    i.to_string(),
                    binary_raw,
                    instruction,
                    number
                ];
            }
            for (cell, &width) in row.iter().zip(&col_widths) {
                write!(out, "{:<width$} ", cell, width = width)?;
            }
            writeln!(out)?;
        }
        writeln!(out)?;
        Ok(())
    }
}