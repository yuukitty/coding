use std::io;

fn main() {

    loop {
        println!("Input which Fibonacci number you want to see.");
        let (mut a, mut b) = (0,1);
        let mut n = String::new();
    
        io::stdin()
            .read_line(&mut n)
            .expect("Unable to read line.");

        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{n}");
        let mut i = 0;
        while i < n-1 {
            (a, b) = (b, a + b);
            i += 1;
            println!("{a}");
        }
        println!("The nth Fibonacci number is {a}");
        break; 
    }
}
