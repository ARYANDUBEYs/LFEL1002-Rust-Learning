#[derive(Debug, Clone)]
struct Pet {
    name: String,
    age: i32,
}

fn main() {
    let original_dog = Pet {
        name: String::from("Buddy"),
        age: 3,
    };
    let mut cloned_dog = original_dog.clone();
    cloned_dog.age = 5;

    print_pet(original_dog.clone());
    print_pet(cloned_dog.clone());
    println!("Main still has {:?} and {:?}", original_dog, cloned_dog);
}

fn print_pet(p: Pet) {
    println!("Printing Pet: {} (Age: {})", p.name, p.age);
}