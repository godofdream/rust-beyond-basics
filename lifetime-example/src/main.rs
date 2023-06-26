// The struct `Person` represents a person with a name.
struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    // The `greet` method takes a borrowed reference to a `Person` and prints a greeting message.
    fn greet(&self, other_person: &Person) {
        println!("Hello, {}! My name is {}.", other_person.name, self.name);
    }
}

fn main() {
    let person1 = Person { name: "Alice" };
    let person2 = Person { name: "Bob" };
    // the name parameters live as long as their Person struct

    person1.greet(&person2); // Borrowing `person2` as an argument to `greet`

    // Ownership of `person2` is not transferred to `greet`, so it can still be used here
    println!("Person2's name is: {}", person2.name);
}
