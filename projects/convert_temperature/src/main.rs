use std::io;

fn main() {
    println!("Input your temperature (C)");

    loop {
        let mut temp_c = String::new();
        io::stdin()
            .read_line(&mut temp_c)
            .expect("Invalid input.");

        let temp_c: i32 = match temp_c.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    println!("{temp_c}C is equal to {}F",convert_to_f(temp_c));
    break;
    }
}

fn convert_to_f(temp_c: i32) -> i32 {
    temp_c * 9 / 5 + 32
}
