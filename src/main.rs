use std::io;

fn main() {
    let nth = read_nth();
    let fibonacci = calculate_nth_fibonacci(nth);
    println!("{fibonacci} is the {nth}th Fibonacci number.");
}

fn read_nth() -> i32 {
    println!("What is the desired nth Fibonacci number?");

    let mut attempts = 0;
    loop {
        let mut nth = String::new();
        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line");

        let nth: i32 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if attempts < 3 {
                    println!("Please insert positiv numbers only.");
                } else {
                    println!("Really? Only positiv numbers.");
                }
                attempts += 1;
                continue;
            }
        };
        return nth;
    }
}

fn calculate_nth_fibonacci(nth: i32) -> u64 {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let fibonacci_of_nth = (phi.powi(nth - 1) / 5.0_f32.sqrt()).round() as u64;
    return fibonacci_of_nth;
}
