pub mod heap;

#[derive(Debug)]
#[derive(Clone)]
struct Person {
    name: String,
    age: u32
}

fn main() {
    let mut person01 = Person {
        name: String::from("person01"),
        age: 21
    };
    let mut person02 = Person {
        name: String::from("person02"),
        age: 21
    };
    let mut person03 = Person {
        name: String::from("person03"),
        age: 21
    };
    let mut person04 = Person {
        name: String::from("person04"),
        age: 21
    };
    let mut person05 = Person {
        name: String::from("person05"),
        age: 21
    };
    let mut person06 = Person {
        name: String::from("person06"),
        age: 21
    };
    let mut heap = heap::Heap::<Person>::new(person01);
}