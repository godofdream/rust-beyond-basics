// A const function that calculates the factorial of a number
const fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    // Using the factorial const function at compile-time
    const N: u32 = 5;
    const FACTORIAL_N: u32 = factorial(N);

    println!("Factorial of {}: {}", N, FACTORIAL_N);

    // Using the factorial const function at runtime
    let number: u32 = 6;
    let factorial_number = factorial(number);

    println!("Factorial of {}: {}", number, factorial_number);
}
