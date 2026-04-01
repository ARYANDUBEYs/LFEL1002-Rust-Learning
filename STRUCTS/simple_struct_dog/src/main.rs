struct Dog {
    name: String,
    age: u8,
    is_hungry: bool,
}
impl Dog {
    fn bark(&self)
    {
        println!("woof! woof!");
    }
    fn eat(&mut self)
    {
        self.is_hungry = true;
        println!("{} is full now.", self.name);
    }
}
fn main() {
    let mut my_dog = Dog {
        name : String::from("Buddy"),
        age : 3,
        is_hungry: true,
    };
    println!("My dog's name is {}. He is {} years old." , my_dog.name, my_dog.age);
    my_dog.bark();
    my_dog.eat();
}