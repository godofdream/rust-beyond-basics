
// Define a trait named `Animal` with two methods: `speak` and `move_`.
trait Animal {
    fn speak(&self);
    fn move_(&self);
}

// Implement the `Animal` trait for the `Dog` struct.
struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("The dog says: Woof!");
    }

    fn move_(&self) {
        println!("The dog runs.");
    }
}

// Implement the `Animal` trait for the `Cat` struct.
struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("The cat says: Meow!");
    }

    fn move_(&self) {
        println!("The cat walks.");
    }
}

// A function that accepts any type implementing the `Animal` trait.
fn perform_actions<T: Animal>(animal: T) {
    animal.speak();
    animal.move_();
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    perform_actions(dog); // Invoke `perform_actions` with a `Dog` instance.
    perform_actions(cat); // Invoke `perform_actions` with a `Cat` instance.
}
