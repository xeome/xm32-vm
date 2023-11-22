use log::{error, info, warn};

pub struct VM {
    regs: [i32; 4],
    stack: Vec<i32>,
    program: Vec<i32>,
    pc: usize,
    halted: bool,
    txt_output: String,
}

impl VM {
    pub fn new() -> Self {
        VM {
            regs: [0; 4],
            stack: Vec::new(),
            program: Vec::new(),
            pc: 0,
            halted: false,
            txt_output: String::new(),
        }
    }

    pub fn init(&mut self, prg: Vec<i32>, txt_out: Option<String>) {
        self.program = prg;
        self.txt_output = txt_out.unwrap_or_default();
        self.pc = 0;
        self.halted = false;
        self.stack.clear();
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
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
                println!("{}", val);
                self.pc += 2;
            }
            // HALT
            255 => {
                self.pc += 1;
                info!("Program halted");
                self.halted = true;
            }
            _ => {
                // println!("Unknown instruction: {} at {}", instr, self.pc);
                error!("Unknown instruction: {} at {}", instr, self.pc);
                self.halted = true;
            }
        }
        if self.pc >= self.program.len() {
            self.halted = true;
            warn!("Program counter out of bounds")
        }
    }
}
