use std::rc::Rc;

// Define a struct to hold some data
struct Data {
    value: i32,
}

fn main() {
    // Create a new instance of Data and wrap it in an Rc
    let data = Rc::new(Data { value: 42 });

    // Clone the Rc, increasing the reference count to 2
    let data_ref1 = Rc::clone(&data);

    // Clone the Rc again, increasing the reference count to 3
    let data_ref2 = Rc::clone(&data);

    // Access the value inside the Data struct via the Rc references
    println!("Value: {}", data.value);
    println!("Value via ref1: {}", data_ref1.value);
    println!("Value via ref2: {}", data_ref2.value);

    // When the Rc references go out of scope, the reference count decreases
    // When the count reaches 0, the memory is deallocated

    // Drop the first reference
    drop(data_ref1); // Reference count: 2

    // Drop the second reference
    drop(data_ref2); // Reference count: 1

    // Since the reference count is 1, we can still access the value via the original Rc
    println!("Value via original Rc: {}", data.value);

    // When the original Rc goes out of scope, the memory is deallocated
    // Since there are no more references, the reference count becomes 0
}



