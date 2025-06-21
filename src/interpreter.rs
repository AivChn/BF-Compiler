use std::env;
use std::fs;
use text_io::read;

// Define Stack type
struct Stack {
    stack: Vec<usize>, // Use usize for consistency with ip
    top: isize,
}

// Implement Stack functions
impl Stack {
    fn new() -> Self {
        Stack {
            stack: vec![0; 256],
            top: -1,
        }
    }

    // Add value to the top of the stack
    fn push(&mut self, val: usize) -> bool {
        if self.top < (self.stack.len() - 1) as isize {
            self.top += 1;
            self.stack[self.top as usize] = val;
            true
        } else {
            println!("Stack full!");
            false
        }
    }

    // Remove value from the top of the stack
    fn pop(&mut self) -> Option<usize> {
        if self.top > -1 {
            let val = self.stack[self.top as usize];
            self.top -= 1;
            Some(val)
        } else {
            println!("Stack empty!");
            None
        }
    }
}

// Main Interpreter
fn main() {
    // Create infrastructure for interpreter
    let mut data_tape: Vec<u8> = vec![0; 30_000];
    let mut dtp: usize = 0; // Data pointer

    let mut stack: Stack = Stack::new(); // Stack for loops
    let mut ip: usize = 0; // Instruction pointer

    let args: Vec<String> = env::args().collect();
    let path = args.get(1);
    let code: String;

    if path.is_none() {
        // Get code input
        code = read!();
    } else {
        code = fs::read_to_string(path.unwrap())
            .expect("Failed to read the file. Please check the file path and try again.");
    }

    let code_vec: Vec<char> = code.chars().collect();

    while ip < code_vec.len() {
        match code_vec[ip] {
            // Basic manipulation characters
            '+' => data_tape[dtp] = data_tape[dtp].wrapping_add(1),
            '-' => data_tape[dtp] = data_tape[dtp].wrapping_sub(1),
            '>' => {
                dtp = if dtp < data_tape.len() - 1 {
                    dtp + 1
                } else {
                    0
                }
            }
            '<' => {
                dtp = if dtp > 0 {
                    dtp - 1
                } else {
                    data_tape.len() - 1
                }
            }
            '.' => print!("{}", data_tape[dtp] as char),
            ',' => data_tape[dtp] = read!(),

            // Loop logic
            '[' => {
                if data_tape[dtp] != 0 {
                    stack.push(ip); // Push next instruction onto stack
                } else {
                    while code_vec[ip] != ']' {
                        ip += 1;
                    } // Skip loop
                }
            }
            ']' => {
                if data_tape[dtp] == 0 {
                    stack.pop();
                } else {
                    let tmp = stack.pop();
                    if tmp.is_none() {
                        println!("could not get ip from stack");
                    } else {
                        ip = tmp.unwrap() - 1;
                    }
                }
            }

            // Ignore any unknown characters
            _ => {}
        }

        // Increment instruction pointer
        ip += 1;

        // Optional Debugging
        // println!("Stack: [{}]", stack.stack[0]);
        // println!("Data Tape[{}]: {}", dtp, data_tape[dtp]);
    }

    println!();
}
