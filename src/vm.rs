use anyhow::Result;
use log::{error, info, warn};

use crate::{assembler::Assembler, screen::Screen};

pub struct VM {
    regs: [i32; 10],
    stack: Vec<i32>,
    program: Vec<i32>,
    pc: usize,
    halted: bool,
    txt_output: String,
    screen: Screen,
    instructions: Assembler,
}

impl VM {
    pub fn new() -> Self {
        VM {
            regs: [0; 10],
            stack: Vec::new(),
            program: Vec::new(),
            pc: 0,
            halted: false,
            txt_output: String::new(),
            screen: Screen::new(),
            instructions: Assembler::new(),
        }
    }

    pub fn init(&mut self, prg: Vec<i32>, txt_out: Option<String>) {
        self.program = prg;
        self.txt_output = txt_out.unwrap_or_default();
        self.pc = 0;
        self.halted = false;
        self.stack.clear();
        self.screen.clear();
    }

    pub fn run(&mut self) -> Result<()> {
        self.screen
            .window
            .limit_update_rate(Some(std::time::Duration::from_millis(16)));

        while !self.halted {
            self.step();
            self.screen.update()?;
        }
        Ok(())
    }

    pub fn step(&mut self) {
        if self.halted {
            info!("Program halted");
            return;
        }

        let instr = self.program[self.pc];

        match instr {
            // MOVR, RDST, RSRC
            10 => {
                let r1 = self.program[self.pc + 1] as usize;
                let r2 = self.program[self.pc + 2] as usize;
                self.regs[r1] = self.regs[r2];
                self.pc += 3;
            }
            // MOVV, RDST, VAL
            11 => {
                let r1 = self.program[self.pc + 1] as usize;
                let val = self.program[self.pc + 2];
                self.regs[r1] = val;
                self.pc += 3;
            }
            // ADD, RDST, RSRC
            20 => {
                let r1 = self.program[self.pc + 1] as usize;
                let r2 = self.program[self.pc + 2] as usize;
                self.regs[r1] += self.regs[r2];
                self.pc += 3;
            }
            // SUB, RDST, RSRC
            21 => {
                let r1 = self.program[self.pc + 1] as usize;
                let r2 = self.program[self.pc + 2] as usize;
                self.regs[r1] -= self.regs[r2];
                self.pc += 3;
            }
            // PUSH, RSRC
            30 => {
                let r1 = self.program[self.pc + 1] as usize;
                self.stack.push(self.regs[r1]);
                self.pc += 2;
            }
            // POP, RDST
            31 => {
                let r1 = self.program[self.pc + 1] as usize;
                let val = self.stack.pop().unwrap_or(0);
                self.regs[r1] = val;
                self.pc += 2;
            }
            // JP, ADDR
            40 => {
                let addr = self.program[self.pc + 1] as usize;
                self.pc = addr;
            }
            // JL, R1, R2, ADDR
            41 => {
                let r1 = self.program[self.pc + 1] as usize;
                let r2 = self.program[self.pc + 2] as usize;
                let addr = self.program[self.pc + 3] as usize;
                self.pc = if self.regs[r1] < self.regs[r2] {
                    addr
                } else {
                    self.pc + 4
                };
            }
            // CALL, ADDR
            42 => {
                let addr = self.program[self.pc + 1] as usize;
                self.stack.push(self.pc as i32 + 2);
                self.pc += 2;
                self.pc = addr;
            }
            // RET
            50 => {
                let addr = self.stack.pop().unwrap_or(0);
                self.pc += 1;
                self.pc = addr as usize;
            }
            // PRINT
            60 => {
                let r1 = self.program[self.pc + 1] as usize;
                let val = self.regs[r1];
                self.txt_output.push_str(&val.to_string());
                println!("{}", val); // For debugging
                self.pc += 2;
            }
            // DRAW, X, Y, PALETTE_INDEX
            61 => {
                let r1 = self.program[self.pc + 1] as usize;
                let r2 = self.program[self.pc + 2] as usize;
                let r3 = self.program[self.pc + 3] as usize;
                let x = self.regs[r1] as usize;
                let y = self.regs[r2] as usize;
                let color = self.regs[r3] as u8;
                self.screen.set_pixel(x, y, color);
                self.pc += 4;
            }
            // CLS
            62 => {
                self.screen.clear();
                self.pc += 1;
            }
            // SLP, MS
            70 => {
                let ms = self.program[self.pc + 1] as u64;
                std::thread::sleep(std::time::Duration::from_millis(ms));
                self.pc += 2;
            }
            // HALT
            255 => {
                self.pc += 1;
                info!("Program halted");
                self.halted = true;
            }
            _ => {
                error!(
                    "Unknown instruction: {}({}) at {}",
                    instr,
                    self.instructions.get_instruction(instr),
                    self.pc
                );
                self.halted = true;
            }
        }
        if self.pc >= self.program.len() {
            self.halted = true;
            warn!("Program counter out of bounds")
        }
    }

    pub fn get_current_instruction(&self) -> &str {
        self.instructions.get_instruction(self.program[self.pc])
    }
}
