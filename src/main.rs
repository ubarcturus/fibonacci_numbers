fn main() {
    let nth = 6;
    let fibonacci = calculate_nth_fibonacci(nth);
    println!("{fibonacci} is the {nth}th fibonacci number.");
}

fn calculate_nth_fibonacci(nth: i32) -> u64 {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let fibonacci_of_nth = (phi.powi(nth - 1) / 5.0_f32.sqrt()).round() as u64;
    return fibonacci_of_nth;
}
