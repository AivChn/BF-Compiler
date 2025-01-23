use text_io::read;

fn main() {
    let x: i32 = read!("{}\r\n");
    let y: i32 = read!("{}\r\n");

    println!("{x} * {y} = {}", x*y);
}
