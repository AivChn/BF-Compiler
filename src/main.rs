mod compiler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bind = "".to_string();
    let path = args.get(1).unwrap_or(&bind);

    if path.is_empty() {
        println!("No path provided")
    } else if path.ends_with(".bf"){
        compiler::run_compiler(path);
    } else {
        println!("File is not bf! use .bf extension");
    }
}

