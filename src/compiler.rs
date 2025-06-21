use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{fs, str};
use string_builder::{Builder, ToBytes};

/*
* Registers and their purpose
*   rbx = index of current cell in tape
*   rcx = temporary holder of rip in end of loop
*
*   TS stands for tape size, since this compiler supports dynamic or custom sizes it will be
*   inserted at compile time after analysis
*
*   x will be replaced with a unique generated number at compile time to make sure labels are
*   unique but also match when needed (looping for example)
*
* Operations:
*   + = inc byte tape[rbx]
*   - = dec byte tape[rbx]
*   > = cmp rbx, TS\ jne skipx\ mov rbx, 0\ skipx: \ inc rbx
*   < = cmp rbx, 0\ jne skipx\ mov rbx, TS\ skipx: \ dec rbx
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

// Define flags struct =====================================================================
struct Flags {
    tape_size: i32,
}

// general code ==================================================================================================================
fn get_token_map(tape_size: i32) -> HashMap<&'static str, String> {
    let mut tokens: HashMap<&'static str, String> = HashMap::new();

    tokens.insert("+", "    inc byte tape[rbx]\n".to_string());
    tokens.insert("-", "    dec byte tape[rbx]\n".to_string());
    tokens.insert(
        ">",
        format!(
            "    cmp rbx, {}\n    jne skip|\n    mov rbx, 0\n    skip|: \n    inc rbx\n",
            tape_size
        ),
    );
    tokens.insert(
        "<",
        format!(
            "    cmp rbx, 0\n   jne skip|\n    mov rbx, {}\n    skip|: \n    dec rbx\n",
            tape_size
        ),
    );
    tokens.insert(".", "    call _print\n".to_string());
    tokens.insert(",", "    call _input\n".to_string());
    tokens.insert("[", "    cmp byte tape[rbx], 0\n    je skip|\n".to_string());
    tokens.insert("]", "    cmp byte tape[rbx], 0\n    jne again|\n".to_string());

    tokens
}

fn compile(code: String, flags: Flags) -> Option<String> {
    // Script start and end will be appended at the start and end of the script string,
    // these are necessary in every bf program no matter what it does
    let script_start: &str = &format!("section .bss\n   tape resb {}\nsection .text\n    global _start\n_start:\n    mov rbx, 0\n", flags.tape_size);
    let script_end: &str = "\n_end:\n    mov rax, 60\n    mov rdi, 0\n    syscall\n";

    // Print and input labels: this will only be added at the end of the file (before script_end)
    // if they are used. This will be determined by input_flag and print_flag
    let print_label: &str = "_print:\n    mov rax, 1\n    mov rdi, 1\n    lea rsi, [tape+rbx]\n    mov rdx, 1\n    syscall\n    ret\n";
    let input_label: &str = "_input:\n    mov rax, 0\n    mov rdi, 0\n    lea rsi, [tape+rbx]\n    mov rdx, 1\n    syscall\n    ret\n";

    let mut input_flag: bool = false;
    let mut print_flag: bool = false;

    // Gets the token HashMap from the function
    let tokens = get_token_map(flags.tape_size);

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

fn assemble_and_link(asm_file: &str, sys: &str) -> Result<(), String> {
    // Create necessary variables with values dependent on OS
    let nasm_fmt;
    let object_file;
    let output_file;

    // Assign values based on current OS
    if sys == "win" {
        nasm_fmt = "win64";
        object_file = asm_file.replace(".asm", ".obj");
        output_file = asm_file.replace(".asm", ".exe");
    } else {
        nasm_fmt = "elf64";
        object_file = asm_file.replace(".asm", ".o");
        output_file = asm_file.replace(".asm", "");
    }

    // run the assembler
    let nasm_status = Command::new("nasm")
        .arg("-f")
        .arg(nasm_fmt)
        .arg(asm_file)
        .status()
        .expect("Failed to execute NASM");

    if !nasm_status.success() {
        return Err("Failed to assemble the file".to_string());
    }

    // run the linker
    let ld_status = Command::new("ld")
        .arg("-o")
        .arg(output_file)
        .arg(&object_file)
        .status()
        .expect("Failed to execute LD");

    if !ld_status.success() {
        return Err("Failed to link file".to_string());
    }

    // Return OK if nothing stopped the function ahead of time
    Ok(())
}

pub fn main() {}

fn evaluate_flags(code: &String) -> HashMap<String, i32> {
    let lines: Vec<&str> = code.split("\n").collect();
    let mut flags: HashMap<String, i32> = HashMap::new();

    for line in lines.iter() {
        if !line.starts_with("#@") {
            break;
        }

        // Extract the flag from the line. If the value is not a number, set to the error number
        // (-1)
        let flag_parts = line.split("=").collect::<Vec<&str>>();
        let flag: &str = flag_parts[0].strip_prefix("#@").unwrap();
        let value: i32 = flag_parts[1].parse::<i32>().unwrap_or(-1);

        flags.insert(flag.to_string(), value);
    }
    flags
}

fn evaluate_size(code: &String, tape_size: i32) -> i32 {
    match tape_size {
        0 => 30000 as i32,
        -1 => {
            (code.chars().filter(|c| *c == '>').count()
                - code.chars().filter(|c| *c == '<').count()
                + 1) as i32
        }
        _ => tape_size,
    }
}

pub fn run_compiler(path: &String) {
    // Try reading the code file into a string
    let code: String;
    code = fs::read_to_string(path).unwrap_or("None".to_string());

    if code == "None".to_string() {
        println!("Error: could not read file");
        return;
    }

    let flags = evaluate_flags(&code);
    let tape_size = evaluate_size(&code, *flags.get("size").unwrap_or(&-1));

    let flag: Flags = Flags {
        tape_size: tape_size,
    };

    // compile the code to assembly
    let compiled_code = compile(code, flag);

    // Create the path for the assembly file and the runnable file name
    let path = Path::new(path);
    let dir = path.parent();

    let filename = path.file_stem().unwrap().to_str().unwrap_or("\0");
    if filename == "\0" {
        println!("Error: Unable to find filename within path");
        return;
    }

    let asm_file = dir.unwrap().join(filename.to_string() + ".asm");
    let runnable_name = dir.unwrap().join(filename.to_string());

    // Create the file object
    let mut file = fs::File::create(asm_file.as_path()).unwrap();

    // Checks if code was compiled correctly and writes to file if it did
    if compiled_code.is_none() {
        println!("Error while compiling code, compilation failed");
        return;
    }
    let _ = file.write_all(compiled_code.unwrap().to_bytes().as_slice());

    // Tries assembling and linking the file
    match assemble_and_link(asm_file.to_str().unwrap(), runnable_name.to_str().unwrap()) {
        Ok(_) => println!("Successfully assembled and linked!"),
        Err(err) => eprintln!("Error: {err}"),
    };
}
