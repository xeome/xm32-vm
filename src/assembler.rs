use std::collections::HashMap;

#[derive(Debug)]
pub struct Assembler {
    pub instructions: HashMap<&'static str, i32>,
    registers: HashMap<&'static str, i32>,
}

impl Assembler {
    pub fn new() -> Self {
        let mut instructions = HashMap::new();
        instructions.insert("MOVR", 10); // Moves the value of one register into another
        instructions.insert("MOVV", 11); // Moves a value into a register
        instructions.insert("ADD", 20); // Adds the value of one register to another
        instructions.insert("SUB", 21); // Subtracts the value of one register from another
        instructions.insert("PUSH", 30); // Pushes a value onto the stack
        instructions.insert("POP", 31); // Pops a value off of the stack
        instructions.insert("JP", 40); // Jumps to an address
        instructions.insert("JL", 41); // Jumps to an address if the last comparison was less than
        instructions.insert("CALL", 42); // Calls a subroutine
        instructions.insert("RET", 50); // Returns from a subroutine
        instructions.insert("PRINT", 60); // Prints a value to the screen
        instructions.insert("DRAW", 61); // Draws a pixel to the screen
        instructions.insert("CLS", 62); // Clears the screen
        instructions.insert("SLP", 70); // Sleeps for a number of milliseconds
        instructions.insert("HALT", 255); // Halts the program

        let mut registers = HashMap::new();
        registers.insert("R0", 0);
        registers.insert("R1", 1);
        registers.insert("R2", 2);
        registers.insert("R3", 3);
        registers.insert("R4", 4);
        registers.insert("R5", 5);
        registers.insert("R6", 6);
        registers.insert("R7", 7);
        registers.insert("R8", 8);
        registers.insert("R9", 9);

        Assembler {
            instructions,
            registers,
        }
    }

    pub fn assemble(&self, code: &str) -> Vec<i32> {
        let tokens = self.get_tokens(code);
        self.get_bytecode(&tokens)
    }

    // Removes comments and empty lines
    fn get_tokens(&self, code: &str) -> Vec<Vec<String>> {
        let lines: Vec<Vec<String>> = code
            .lines()
            .map(|line| line.split("//").next().unwrap_or("").trim().to_uppercase()) // Remove comments
            .filter(|line| !line.is_empty()) // Remove empty lines
            .map(|line| line.split_whitespace().map(|s| s.to_string()).collect()) // Split into tokens
            .collect(); // Collect into a vector

        lines
    }

    pub fn get_bytecode(&self, tokens: &Vec<Vec<String>>) -> Vec<i32> {
        let mut bytes = Vec::new();

        for line in tokens {
            for (i, token) in line.iter().enumerate() {
                let token = token.trim().to_uppercase();

                match i {
                    0 => {
                        let token = *self.instructions.get(token.as_str()).unwrap_or(&-1);
                        bytes.push(token);
                    }
                    _ if token.starts_with('R') => {
                        let token = *self
                            .registers
                            .get(token.split(',').next().unwrap_or(""))
                            .unwrap_or(&-1);
                        bytes.push(token);
                    }
                    _ => {
                        let token = token.parse::<i32>().unwrap_or(-1);
                        bytes.push(token);
                    }
                }
            }
        }

        bytes
    }
}
