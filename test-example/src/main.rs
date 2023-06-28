
// A simple function
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}


fn main() -> Result<(),anyhow::Error> {
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if at least two argument is provided
    if args.len() != 3 {
        return Err(anyhow::anyhow!("Please provide two numbers."))
    }

    // Access the first argument
    let a: i32 = args[1].parse()?;
    let b: i32 = args[2].parse()?;
    let res = add_two_numbers(a, b);
    println!("{a} + {b} = {res}");
    Ok(())
}



// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(add_two_numbers(2, 3), 5);
        assert_eq!(add_two_numbers(-10, 10), 0);
        assert_eq!(add_two_numbers(5, -3), 2);
    }
}