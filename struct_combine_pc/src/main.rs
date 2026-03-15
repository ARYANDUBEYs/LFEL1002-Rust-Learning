struct Monitor {
    brand: String,
}
impl Monitor {
    fn display(&self, text: &str) {
        println!("Monitor [{}] showing: {}", self.brand, text);
    }
}
struct Computer {
    model: String,
    screen: Monitor,
}
impl Computer {
    fn startup(&self) {
        println!("Starting up {}...", self.model);
        self.screen.display("Hello World!");
    }
}
fn main() {
    let my_monitor = Monitor { brand: String::from("Dell") };
    
    let my_pc = Computer {
        model: String::from("Gaming Rig"),
        screen: my_monitor,
    };

    my_pc.startup();
}