use std::any::{Any, TypeId};

fn main() {
    let boxed: Box<dyn Any> = Box::new(100i32);

    let boxed_id = boxed.type_id();
    println!("boxed id: {:?}", boxed_id);
    assert_eq!(boxed_id, TypeId::of::<Box<dyn Any>>());

    let actual_id = (&*boxed).type_id();
    println!("actual id: {:?}", actual_id);
    assert_eq!(actual_id, TypeId::of::<i32>());

    let number = 100i32;
    let number_id = number.type_id();
    println!("number id: {number_id:?}");

    print_if_string_box(Box::new(100i32));
    print_if_string_box(Box::new("Hello String".to_owned()));

    print_if_string_ref(&100);
    print_if_string_ref(&"Hello String".to_owned());
}

fn print_if_string_box(s: Box<dyn Any>) {
    if let Ok(string) = s.downcast::<String>() {
        println!("This is a string: {string}");
    } else {
        println!("This is not a string");
    }
}

fn print_if_string_ref(s: &dyn Any) {
    if let Some(string) = s.downcast_ref::<String>() {
        println!("This is a string: {string}");
    } else {
        println!("This is not a string");
    }
}
