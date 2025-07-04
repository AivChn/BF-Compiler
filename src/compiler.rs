use std::{collections::HashMap, fs};
use string_builder::Builder;

/*
* Registers and their purpose
*   rbx = index of current cell in tape
*   rcx = temporary holder of rip in end of loop
*
*   x will be replaced with a unique generated number at compile time to make sure labels are
*   unique but also match when needed (looping for example)
*
* Operations:
*   + = inc byte tape[rbx]
*   - = dec byte tape[rbx]
*   > = cmp rbx, 30000\ jne skipx\ mov rbx, 0\ skipx: \ inc rbx
*   < = cmp rbx, 0\ jne skipx\ mov rbx, 30000\ skipx: \ dec rbx
*   . = call _print ; _print will be implemented at the end of file
*   , = call _input ; _input will be implemented at the end of file
*   [ = cmp byte tape[rbx], 0\ je skipx
*   ] = cmp byte tape[rbx], 0\ jne againx
*
* Pre-created labels:
*   _print = mov rax, 1\ mov rdi, 1\ lea rsi, [tape+rbx]\ mov rdx, 1\ syscall\ ret
*   _input = mov rax, 0\ mov rdi, 0\ lea rsi, [tape+rbx]\ mov rdx, 1\ syscall\ ret
*/

// Define Stack type =============================================================================================================
struct Stack {
    stack: Vec<u32>, // Use usize for consistency with ip
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

    // Add value to the stack
    fn push(&mut self, val: u32) -> bool {
        if self.top < (self.stack.len() - 1) as isize {
            self.top += 1;
            self.stack[self.top as usize] = val;
            true
        } else {
            println!("Stack full!");
            false
        }
    }

    // Remove value from the stack
    fn pop(&mut self) -> Option<u32> {
        if self.top > -1 {
            let val = self.stack[self.top as usize];
            self.top -= 1;
            Some(val)
        } else {
            println!("Stack empty!");
            None
        }
    }

    // Peek at the top value of the stack
    fn top(&mut self) -> Option<u32> {
        if self.top > -1 {
            Some(self.stack[self.top as usize])
        } else {
            println!("Stack empty!");
            None
        }
    }

    // Return true if the stack is empty
    fn is_empty(&self) -> bool {
        self.top == -1
    }
}

// general code ==================================================================================================================
fn get_token_map() -> HashMap<&'static str, String> {
    let mut tokens: HashMap<&'static str, String> = HashMap::new();

    tokens.insert("+", "    inc byte tape[rbx]\n".to_string());
    tokens.insert("-", "    dec byte tape[rbx]\n".to_string());
    tokens.insert(">", "    cmp rbx, 30000\n    jne skip|\n    mov rbx, 0\n    skip|: \n    inc rbx\n".to_string());
    tokens.insert("<", "    cmp rbx, 0\n    jne skip|\n    mov rbx, 30000\n    skip|: \n    dec rbx\n".to_string());
    tokens.insert(".", "    call _print\n".to_string());
    tokens.insert(",", "    call _input\n".to_string());
    tokens.insert("[", "    cmp byte tape[rbx], 0\n    je skip|\n".to_string());
    tokens.insert("]", "    cmp byte tape[rbx], 0\n    jne again|\n".to_string());

    tokens
}

fn compile(code: String) -> Option<String> {
    // Script start and end will be appended at the start and end of the script string,
    // these are necessary in every bf program no matter what it does
    let script_start: &str = &format!("section .bss\n   tape resb 30000\nsection .text\n    global _start\n_start:\n    mov rbx, 0\n");
    let script_end: &str = "\n_end:\n    mov rax, 60\n    mov rdi, 0\n    syscall\n";

    // Print and input labels: this will only be added at the end of the file (before script_end)
    // if they are used. This will be determined by input_flag and print_flag
    let print_label: &str = "_print:\n    mov rax, 1\n    mov rdi, 1\n    lea rsi, [tape+rbx]\n    mov rdx, 1\n    syscall\n    ret\n";
    let input_label: &str = "_input:\n    mov rax, 0\n    mov rdi, 0\n    lea rsi, [tape+rbx]\n    mov rdx, 1\n    syscall\n    ret\n";

    let mut input_flag: bool = false;
    let mut print_flag: bool = false;

    // Gets the token HashMap from the function
    let tokens = get_token_map();

    // Creates a string builder, this is where the asm code will be saved to
    let mut script_builder: Builder = Builder::default();
    script_builder.append(script_start);

    // This is needed to make sure every label in the code is unique. The same label needs to be used in the opening of a loop ('[')
    // and the ending of one (']'), as can be seen in the operations table, "skip|" is used as the label and | is the placeholder
    let mut skip_counter: Stack = Stack::new();
    let mut max_skips: u32 = 0;

    // Checks if any loop was closed. If not, the labels skip0 needs to be added at the end of file in case one was opened.
    let mut was_loop_closed: bool = false;

    // Iterates through the code one character at a time
    for (i, ch) in code.chars().enumerate() {
        // Gets the compiled code of the
        let mut curr_token = tokens
            .get(ch.to_string().as_str())
            .unwrap_or(&"".to_string())
            .to_string();
        match ch {
            // Replacing the placeholder with a unique number and pushing it to the stack for usage for the closing brackets
            '[' => {
                skip_counter.push(max_skips + 1);
                skip_counter.push(max_skips);
                max_skips += 1;
                let again = "again|: \n".replace("|", &skip_counter.top().unwrap().to_string());
                curr_token = again + curr_token.replace("|", &max_skips.to_string()).as_str();
                max_skips += 1;
            }

            // Replacing the placeholder with a unique number and removing the top value from the stack
            ']' => {
                // If the stack is empty throw an error
                let again_num = skip_counter.pop();
                if again_num.is_none() {
                    println!("Error: closed loop without one open at {}", &i.to_string());
                    return None;
                }

                let skip_num = skip_counter.pop();
                if skip_num.is_none() {
                    println!("Error: closed loop without one open at {}", &i.to_string());
                    return None;
                }

                was_loop_closed = true;

                curr_token = curr_token.replace("|", &again_num.unwrap().to_string())
                    + "skip|: \n"
                        .replace("|", &skip_num.unwrap().to_string())
                        .as_str();
            }

            // Activates the flags if the command is used
            '.' => print_flag = true,
            ',' => input_flag = true,

            '+' | '-' => curr_token = curr_token,
            '>' | '<' => {
                max_skips += 1;
                curr_token = curr_token.replace("|", &max_skips.to_string().as_str())
            }

            // If the character isn't a bf instruction, just ignore
            _ => {
                continue;
            }
        }

        script_builder.append(curr_token);
    }

    // If the stack isn't empty then there are unclosed loops. This might be intentional, so throw a warning
    if !skip_counter.is_empty() {
        println!(
            "Warning: {} unclosed loops!",
            &(skip_counter.top + 1).to_string()
        )
    }

    // If no loop was closed in the code add the "skip0:" label in case one was opened
    if !was_loop_closed {
        script_builder.append(format!("skip{}:\n", max_skips))
    }

    // Add the end of script instructions
    script_builder.append(script_end);

    // Add the print and input labels if needed
    if print_flag {
        script_builder.append(print_label);
    }
    if input_flag {
        script_builder.append(input_label);
    }

    // Returns a string representation of the script
    Some(script_builder.string().unwrap())
}


pub fn run_compiler(path: &String) -> Result<String, String> {
    // Try reading the code file into a string
    let code: String;
    code = match fs::read_to_string(path) {
        Ok(code) => code,
        Err(error) => return Err(format!("Could not read file: {}", error))
    };

    match compile(code) {
        Some(compiled_code) => Ok(compiled_code),
        _ => Err("Compilation failed for unknown reasons".to_string())
    }
}
