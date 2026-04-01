use std::io::{self, Write};
use embedded_hal::digital::v2::OutputPin;
use std::time::Duration; //For timing
use std::thread; //pause

pub struct GpioController {          // Three pins
    pin_a: MyGpioPin,
    pin_b: MyGpioPin,
    pin_c: MyGpioPin,
} 

pub struct MyGpioPin {
    is_on: bool,                          //ON / OFF ; pins will be ON or OFF
} 

impl GpioController {
    pub fn new() -> GpioController {                     // Returns a new built GpioController struct to the orginal struct
        let pin_a = MyGpioPin { is_on: false };
        let pin_b = MyGpioPin { is_on: false };
        let pin_c = MyGpioPin { is_on: false }; 

        GpioController { pin_a, pin_b, pin_c }
    } 

    pub fn turn_on_led(&mut self, led: char) {
        match led {
            'A' => self.pin_a.set_high(),
            'B' => self.pin_b.set_high(),
            'C' => self.pin_c.set_high(),
            _ => Ok(()),
        };  
    } 

    pub fn turn_off_led(&mut self, led: char) {
        match led {
            'A' => self.pin_a.set_low(),
            'B' => self.pin_b.set_low(),
            'C' => self.pin_c.set_low(),
            _ => Ok(()),
        };
    }
} 

impl OutputPin for MyGpioPin {
    type Error = (); 

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.is_on = true;
        println!("Set GPIO pin high");
        Ok(())
    } 

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.is_on = false;
        println!("Set GPIO pin low");
        Ok(())
    }
} 

fn main() {
    let mut gpio_controller = GpioController::new(); 

    print!("Which LED would you like to turn on (A, B, or C)? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if let Some(led_choice) = input.trim().chars().next() {
        
        gpio_controller.turn_on_led(led_choice);
        thread::sleep(Duration::from_millis(1000));
        gpio_controller.turn_off_led(led_choice);
    } 
    else {
        println!("You didn't enter anything!");
    }
}