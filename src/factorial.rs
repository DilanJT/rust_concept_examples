use std::io;

// Calculate the factorial of a non-negative integer
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        (1..=n).product()
    }
}

fn main() {
    println!("Enter a number to to calculate its factorial: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid non-negative integer.");
            return;
        }
    };

    if number > 20 {
        println!("The factorial of {} is too large to compute with u64.", number);
        return;
    }

    let result = factorial(number);

    println!("The factorial of {} is {}", number, result);


}