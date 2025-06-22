use std::io::Write;
use std::{fs, path::Path, process::Command};

mod compiler;

struct TargetConfig {
    nasm_format: &'static str,
    object_ext: &'static str, 
    exec_ext: &'static str,
    linker: &'static str
}

fn get_target_config() -> Result<TargetConfig, String>{
    match std::env::consts::OS {
        "linux" => Ok(TargetConfig {
            nasm_format: "elf64",
            object_ext: ".o", 
            exec_ext: "", 
            linker: "ld",
        }),
        "macos" => Ok(TargetConfig {
            nasm_format: "macho64",
            object_ext: ".o", 
            exec_ext: "", 
            linker: "ld",
        }),
        "windows" => Ok(TargetConfig {
            nasm_format: "win64",
            object_ext: ".obj",
            exec_ext: "", 
            linker: "lld-link",
        }),
        unsupported => return Err(format!("Unsopported OS: {}", unsupported))
    }
}

fn assemble_and_link(asm_file: &str) -> Result<(), String> {
    // get the OS specific configuration
    let config = match get_target_config(){
        Ok(config) => config,
        Err(error) => return Err(error)
    };

    // Get the assembling format and extensions based on OS
    let object_file = asm_file.replace(".asm", config.object_ext);
    let executable_file = asm_file.replace(".asm", config.exec_ext);

    // run the assembler
    let nasm_status = match Command::new("nasm")
        .arg("-f").arg(config.nasm_format)
        .arg(asm_file)
        .status() {
            Ok(status) => status,
            Err(error) => return Err(format!("Failed to assemble file: {}", error))
        };

    if !nasm_status.success() {
        return Err("Failed to assemble the file".to_string());
    }

    // run the linker
    let ld_status = match Command::new(config.linker)
        .arg("-o").arg(executable_file)
        .arg(&object_file)
        .status() {
            Ok(status) => status,
            Err(error) => return Err(format!("Failed to Link file: {}", error))
        };

    if !ld_status.success() {
        return Err("Failed to link file".to_string());
    }

    // Return OK if nothing stopped the function ahead of time
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let path = args.get(1).expect("Could not find path!");
    if !path.ends_with(".bf") {
        panic!("Not a valid file!, use .bf as extension");
    }

    // compile the code
    let compiled = match compiler::run_compiler(path) {
        Ok(compiled_code) => compiled_code,
        Err(error) => panic!("{}", error)
    };

    // Create the path for the assembly file and the runnable filename
    let path = Path::new(path);
    let dir = path.parent().expect("Could not get parent directory of file!");

    let filename = path.file_stem().expect("Path file stem doesn't exist")
        .to_str().expect("Failed to extract path file stem");

    let asm_file = dir.join(Path::new(&[filename, ".asm"].concat()));

    // Create the file object
    let mut file = match fs::File::create(asm_file.as_path()) {
        Ok(file) => file,
        Err(error) => panic!("Could not load file: {}", error)
    };

    if let Err(error) = file.write_all(compiled.as_bytes()){
        panic!("Could not write assembly to file: {}", error);
    }

    // Tries assembling and linking the file
    match assemble_and_link(asm_file.to_str().unwrap()){
        Ok(_) => println!("Successfully assembled and linked!"),
        Err(err) => panic!("Error: {err}"),
    };
}
