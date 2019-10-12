use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Pwm, Channel, Polarity};
use std::time::Duration;

pub struct Motors{
    pub left_motor:  LeftMotor,
    pub right_motor:  RightMotor,
}

impl Motors{
    pub fn new() -> Self{
        let mut right_motor = RightMotor::new();
        right_motor.set_direction(MotorDirection::Forward);
        let mut left_motor = LeftMotor::new();
        left_motor.set_direction(MotorDirection::Forward);
        right_motor.set_power_0_to_1(0.0);
        left_motor.set_power_0_to_1(0.0);
        Motors{
            left_motor,
            right_motor
        }
    }

    pub fn apply_config(&mut self, config: MotorsConfig){
        self.right_motor.set_direction(config.right_config.direction);
        self.right_motor.set_power_0_to_1(config.right_config.power_0_to_1);

        self.left_motor.set_direction(config.left_config.direction);
        self.left_motor.set_power_0_to_1(config.left_config.power_0_to_1);
    }

}

#[derive(Debug, Clone)]
pub struct MotorsConfig{
    pub left_config:  SingleMotorConfig,
    pub right_config:  SingleMotorConfig,
}

#[derive(Debug, Clone)]
pub struct SingleMotorConfig{
    pub direction: MotorDirection,
    pub power_0_to_1: f64
}


#[derive(Debug, Clone)]
pub enum MotorDirection{
    Forward,
    Backwards,
    Neutral,
    Brake
}

pub trait Motor{
    fn new() -> Self;
    fn get_direction(&self) -> MotorDirection;
    fn set_direction(&mut self, direction: MotorDirection);
    fn set_power_0_to_1(&self, power: f64);
    fn get_power_0_to_1(&self) -> f64;
}


fn static_set_direction(direction_pin_0: &mut OutputPin, direction_pin_1: &mut OutputPin, direction: &MotorDirection) {
    match direction {
        MotorDirection::Forward => {
            direction_pin_0.set_low();
            direction_pin_1.set_high();
        },
        MotorDirection::Backwards => {
            direction_pin_0.set_high();
            direction_pin_1.set_low();
        },
        MotorDirection::Neutral => {
            direction_pin_0.set_low();
            direction_pin_1.set_low();
        },
        MotorDirection::Brake => {
            direction_pin_0.set_high();
            direction_pin_1.set_high();
        },
    }
}


fn static_get_direction(direction_pin_0: &OutputPin, direction_pin_1: &OutputPin) -> MotorDirection {
    match (direction_pin_0.is_set_high(), direction_pin_1.is_set_high()){
        (false, true) => MotorDirection::Forward,
        (true, false) => MotorDirection::Backwards,
        (false, false) => MotorDirection::Neutral,
        (true, true) => MotorDirection::Brake,
    }
}


pub struct LeftMotor{
    direction_pin_0: rppal::gpio::OutputPin,
    direction_pin_1: rppal::gpio::OutputPin,
    enable_pin: rppal::pwm::Pwm
}

impl Motor for LeftMotor{
    fn new() -> Self {
        let gpio = Gpio::new().expect("Could not get GPIO!");
        let direction_pin_0 = gpio.get(13).expect("Error opening pin 13").into_output();
        let direction_pin_1 = gpio.get(26).expect("Error opening pin 26").into_output();


        let enable_pin = Pwm::with_frequency(
            Channel::Pwm1,
            400.0,
            0.0,
            Polarity::Normal,
            true,
        ).unwrap();
        // Make sure the pin was exported
        std::thread::sleep(Duration::from_millis(500));
        Self{
            direction_pin_0,
            direction_pin_1,
            enable_pin
        }
    }

    fn get_direction(&self) -> MotorDirection {
        static_get_direction(&self.direction_pin_0, &self.direction_pin_1)
    }

    fn set_direction(&mut self, direction: MotorDirection) {
        static_set_direction(&mut self.direction_pin_0, &mut self.direction_pin_1, &direction);
    }

    fn set_power_0_to_1(&self, power: f64) {
        self.enable_pin.set_duty_cycle(power).unwrap();
    }

    fn get_power_0_to_1(&self) -> f64{
        self.enable_pin.duty_cycle().unwrap()
    }
}





pub struct RightMotor{
    direction_pin_0: rppal::gpio::OutputPin,
    direction_pin_1: rppal::gpio::OutputPin,
    enable_pin: rppal::pwm::Pwm
}

impl Motor for RightMotor{
    fn new() -> Self {
        let gpio = Gpio::new().expect("Could not get GPIO!");
        let direction_pin_0 = gpio.get(5).expect("Error opening pin 13").into_output();
        let direction_pin_1 = gpio.get(6).expect("Error opening pin 26").into_output();


        let enable_pin = Pwm::with_frequency(
            Channel::Pwm0,
            400.0,
            0.0,
            Polarity::Normal,
            true,
        ).unwrap();
        // Make sure the pin was exported
        std::thread::sleep(Duration::from_millis(500));
        Self{
            direction_pin_0,
            direction_pin_1,
            enable_pin
        }
    }

    fn get_direction(&self) -> MotorDirection {
        static_get_direction(&self.direction_pin_0, &self.direction_pin_1)
    }

    fn set_direction(&mut self, direction: MotorDirection) {
        static_set_direction(&mut self.direction_pin_0, &mut self.direction_pin_1, &direction);
    }

    fn set_power_0_to_1(&self, power: f64) {
        self.enable_pin.set_duty_cycle(power).unwrap();
    }

    fn get_power_0_to_1(&self) -> f64{
        self.enable_pin.duty_cycle().unwrap()
    }
}