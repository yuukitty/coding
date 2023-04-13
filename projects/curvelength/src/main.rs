use std::io;

fn main() {
    loop {
        println!("Input your upper bound");
        let mut input1 = String::new();
        io::stdin()
            .read_line(&mut input1)
            .expect("Invalid input");

        let x_upper:f32 = match input1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Input your lower bound");
        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("Invalid input");

        let x_lower:f32 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let length = calculate_curve_length(x_lower,x_upper);
        println!("Length of the curve from x={} to x={} is {}",x_lower,x_upper,length);
        break;
    }
}

fn calculate_curve_length(x_lower:f32,x_upper:f32) -> f32 {
    let h = 1.0 / 500.0;
    let (mut x1, mut x2) = (x_lower, x_lower + h);
    let mut a:f32;
    let b = h.powf(2.0);
    let mut totalsum = 0.0;

    while x2 <= x_upper {
        a = ((2.0*(x2)+4.0).sqrt()-(2.0*(x1)+4.0).sqrt()).powf(2.0);
        totalsum += (a + b).sqrt();
        (x1,x2) = (x2, x2 + h);
    }
    totalsum
}
