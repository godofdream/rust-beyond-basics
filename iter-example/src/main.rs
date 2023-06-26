fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Using for loop
    println!("Using for loop:");
    for num in &numbers {
        let squared = num * num;
        println!("{} squared is {}", num, squared);
    }

    // Using for loop using range
    println!("Using for loop:");
    for num in 10..15 {
        let squared = num * num;
        println!("{} squared is {}", num, squared);
    }

    // Using iterator
    println!("Using iterator:");
    let squared_nums: Vec<i32> = numbers.iter().map(|&num| num * num).collect();
    for squared in &squared_nums {
        println!("Squared number: {}", squared);
    }
}
