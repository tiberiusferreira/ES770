mod hardware;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::gpio::{Gpio, Trigger};
use crossbeam_channel::{unbounded, Sender, Receiver};
use crate::hardware::motors::{Motor, MotorDirection};


fn main() -> Result<(), Box<dyn Error>> {
//    let (s, r): (Sender<Events>, Receiver<Events>) = unbounded();
    let mut left_motor = hardware::motors::LeftMotor::new();
    left_motor.set_direction(MotorDirection::Forward);
    for i in 0..=2{
        left_motor.set_power_0_to_1(i as f64/10.0);
        std::thread::sleep(Duration::from_secs(2));
    }
    left_motor.set_direction(MotorDirection::Backwards);
    for i in 0..=2{
        left_motor.set_power_0_to_1(i as f64/10.0);
        std::thread::sleep(Duration::from_secs(2));
    }

    left_motor.set_power_0_to_1(0.0);
//    let left_wheel = gpio.get(20).expect("Error opening pin 16");
//    let mut left_wheel = left_wheel.into_input();
//    let async_sender = s.clone();
//    left_wheel.set_async_interrupt(Trigger::RisingEdge, move |_level|{
//        async_sender.send(Events::LeftWheelTick).expect("Error sending event to main loop");
//    }).expect("Error setting interrupt on pin 20");

    Ok(())
}

