use std::any::{Any, TypeId};
use std::fmt::Debug;

#[derive(Debug)]
struct MyStruct {
    data: i32,
}

impl MyStruct {
    fn do_something(&self) {
        println!("Doing something with MyStruct: {}", self.data);
    }
}

// A function that takes a boxed trait object and downcasts it to the expected type
fn process_object<T: Any + Debug>(obj: &T) {
    let obj_any = obj as &dyn Any;
    // Downcast to MyStruct
    if let Some(my_struct) = obj_any.downcast_ref::<MyStruct>() {
        my_struct.do_something();
    } else if let Some(some_string) = obj_any.downcast_ref::<String>() {
        println!("{some_string}");
    } else {
        println!("Failed to downcast object!");
    }
}

fn main() {
    // Check the type of the object
    let type_id = TypeId::of::<MyStruct>();
    println!("Type ID: {:?}", type_id);

    // Process the object
    process_object(&MyStruct { data: 42 });

    // Check the type of the string
    let type_id = TypeId::of::<String>();
    println!("Type ID: {:?}", type_id);

    let hello = "hello world".to_string();
    // Process the string
    process_object(&hello);
}
