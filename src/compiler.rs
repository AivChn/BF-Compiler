use std::{fs, str};
use std::path::Path;
use std::collections::HashMap;
use std::io::Write;
use std::process::Command;
use string_builder::{Builder, ToBytes};



/* 
* Registers and their purpose
*   rbx = index of current cell in tape
*   rcx = temporary holder of rip in end of loop
*   
* Operations:
*   + = cmp tape[rbx], 255 \ je _inc_max \ inc tape[rbx]
*   - = cmp tape[rbx], 0 \ je _dec_min \ dec tape[rbx]
*   > = cmp rbx, 30000 \ je _inc_max_idx \ inc rbx\
*   < = cmp rbx, 0 \ je _dec_min_idx \ dec rbx
*   . = call _print ; _print will be implemented at the end of file
*   , = call _input ; _input will be implemented at the end of file
*   [ = cmp tape[rbx], 0 \ je skipx \ push rip ; x in ax will be replaced with a number assigned at compilation
*   ] = pop rcx \ cmp tape[rbx], 0 \ je skipx \ mov rip, rcx \ skipx: ; x will be replaced with a number assigned at compilation
*
* Pre-created labels:
*   _inc_max = mov tape[rbx], 0 \ ret
*   _dec_max = mov tape[rbx], 255 \ ret
*   _inc_max_idx = mov rbx, 0 \ ret
*   _dec_min_idx = mov rbx, 29999 \ ret
*
*   _print = mov rax, 1 \ mov rdi, 1 \ mov rsi, tape+rbx \ mov rdx, 1 \ syscall \ ret
*   _input = mov rax, 0 \ mov ri, 0 \ mov rsi, tape+rbx \ mov rdx,1 \ syscall \ ret
*
*   throughout the code '|' is used as a unique placeholder to be replaced with a number
*/



// Define Stack type =============================================================================================================
struct Stack {
    stack: Vec<u32>, // Use usize for consistency with ip
    top: isize
}

// Implement Stack functions
impl Stack {
    fn new() -> Self {
        Stack {
            stack: vec![0; 256],
            top: -1
        }
    }

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

    fn top(&mut self) -> Option<u32> {
        if self.top > -1 {
            Some(self.stack[self.top as usize])
        } else {
            println!("Stack empty!");
            None
        }
    }

    fn is_empty(&self) -> bool {self.top == -1}

}



// general code ==================================================================================================================
fn get_token_map() -> HashMap<&'static str, &'static str> {

    let mut tokens: HashMap<&'static str, &'static str> = HashMap::new();

    tokens.insert("+", "    cmp byte tape[rbx], 255\n    jne skip|\n    mov byte tape[rbx], 0\nskip|: \n    inc byte tape[rbx]\n");
    tokens.insert("-", "    cmp byte tape[rbx], 0\n   jne skip|\n    mov byte tape[rbx], 255\nskip|: \n    dec byte tape[rbx]\n");
    tokens.insert(">", "    cmp rbx, 30000\n    jne skip|\n    mov rbx, 0\nskip|: \n    inc rbx\n");
    tokens.insert("<", "    cmp rbx, 0\n    jne skip|\n    mov rbx, 29999\nskip|: \n    dec rbx\n");
    tokens.insert(".", "    call _print\n");
    tokens.insert(",", "    call _input\n");
    tokens.insert("[", "    cmp byte tape[rbx], 0\n    je skip|\n");
    tokens.insert("]", "    cmp byte tape[rbx], 0\n    jne again|\n");
    tokens
}

fn compile(code: String) -> Option<String> {
    // script start and end will be appended at the start and end of the script string, these are necessary in every bf program no matter what it does
    let script_start: &str = "section .bss\n   tape resb 30000\nsection .text\n    global _start\n_start:\n    mov rbx, 0\n";
    let script_end: &str = "\n_end:\n    mov rax, 60\n    mov rdi, 0\n    syscall\n";

    // print and input labels: this will only be added at the end of the file (before script_end) if they are used. this will be determined by input_flag and print_flag
    let print_label: &str = "_print:\n    mov rax, 1\n    mov rdi, 1\n    lea rsi, [tape+rbx]\n    mov rdx, 1\n    syscall\n    ret\n";
    let input_label: &str = "_input:\n    mov rax, 0\n    mov rdi, 0\n    lea rsi, [tape+rbx]\n    mov rdx, 1\n    syscall\n    ret\n";

    let mut input_flag: bool = false;
    let mut print_flag: bool = false;

    // gets the token hashmap from the function
    let tokens = get_token_map();

    // creates a string builder, this is where the asm code will be saved to
    let mut script_builder: Builder = Builder::default();
    script_builder.append(script_start);

    // this is needed to make sure every lable in the code is unique. the same lable needs to be used in the opening of a loop ('[')
    // and the ending of one (']'), as can be seen in the operations table, "skip|" is used as the lable and | is the placeholder
    let mut skip_counter: Stack = Stack::new();
    let mut max_skips: u32 = 0;

    // checks if any loop was closed. if not, the labels skip0 needs to be added at the end of file in case one was opened.
    let mut was_loop_closed: bool = false;

    // iterates through the code one character at a time
    for (i, ch) in code.chars().enumerate(){
        // gets the compiled code of the
        let mut curr_token = tokens.get(&ch.to_string().as_str()).unwrap_or(&"_").to_string();
        match ch {

            // replacing the placeholder with a unique number and pushing it to the stack for usage for the closing brackets
            '[' => {
                skip_counter.push(max_skips+1);
                skip_counter.push(max_skips);
                max_skips += 1;
                let again = "again|: \n".replace("|", &skip_counter.top().unwrap().to_string());
                curr_token = again + curr_token.replace("|", &max_skips.to_string()).as_str();
                max_skips += 1;
            },

            // replacing the placeholder with a unique number and removing the top value from the stack
            ']' => {
                // if the stack is empty throw an error
                let again_num = skip_counter.pop();
                if again_num.is_none() {println!("Error: closed loop without one open at {}", &i.to_string()); return None;}

                let skip_num = skip_counter.pop();
                if skip_num.is_none() {println!("Error: closed loop without one open at {}", &i.to_string()); return None;}

                was_loop_closed = true;

                curr_token = curr_token.replace("|", &again_num.unwrap().to_string()) +
                    "skip|: \n".replace("|", &skip_num.unwrap().to_string()).as_str();
            },

            // activates the flags if the command is used
            '.' => { print_flag = true },
            ',' => { input_flag = true },

            // replacing the placeholder with a unique value and increasing max_skips by one
            // (no need for stack since value is used only in this part of the code)
            '+' | '-' | '>' | '<' => {
                curr_token = curr_token.replace("|", &max_skips.to_string());
                max_skips += 1;
            }

            // if the character isn't a bf instruction, just ignore
            _ => {continue;}

        }

        script_builder.append(curr_token);
    }

    // if the stack isn't empty then there are unclosed loops. this might be intentional, so throw a warning
    if !skip_counter.is_empty() {
        println!("Warning: {} unclosed loops!", &(skip_counter.top+1).to_string())
    }

    // if no loop was closed in the code add the "skip0:" lable in case one was opened
    if !was_loop_closed {
        script_builder.append(format!("skip{}:\n", max_skips))
    }

    // add the end of script instructions
    script_builder.append(script_end);

    // add the print and input labels if needed
    if print_flag {
        script_builder.append(print_label);
    }
    if input_flag {
        script_builder.append(input_label);
    }

    // returns a string representation of the script
    Some(script_builder.string().unwrap())
}

fn assemble_and_link(asm_file: &str, output_file: &str) -> Result<(), String> {

    let nasm_status = Command::new("nasm").arg("-f").
        arg("elf64").arg(asm_file).
        status().expect("Failed to execute NASM");

    if !nasm_status.success() {
        return Err("Failed to assemble the file".to_string());
    }

    let object_file = asm_file.replace(".asm", ".o");

    let ld_status = Command::new("ld").arg("-o").arg(output_file).
        arg(&object_file).status().expect("Failed to execute LD");

    if !ld_status.success() {
        return Err("Failed to link file".to_string());
    }

    Ok(())

}

pub fn run_comiler(path: &String) {

    let code: String;
    code = fs::read_to_string(path).unwrap_or("None".to_string());

    if code == "None".to_string() {
        println!("Error: could not read file");
        return;
    }

    let compiled_code = compile(code);

    let path = Path::new(path);
    let dir = path.parent();
    let filename =  path.file_stem().unwrap().to_str().unwrap_or("\0");
    if filename == "\0" {println!("Error: Unable to find filename within path"); return;}
    let asm_file = dir.unwrap().join(filename.to_string() + ".asm");

    let mut file = fs::File::create(asm_file.as_path()).unwrap();
    let runnable_name = dir.unwrap().join(filename.to_string());

    if compiled_code.is_none() {println!("Error while compiling code, compilation failed"); return;}
    let _ = file.write_all(compiled_code.unwrap().to_bytes().as_slice());

    match assemble_and_link(asm_file.to_str().unwrap(), runnable_name.to_str().unwrap()) {
        Ok(_) => println!("Successfully assembled and linked!"),
        Err(err) => eprintln!("Error: {err}")
    };

}
