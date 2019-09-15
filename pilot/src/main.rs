mod hardware;
use std::error::Error;
use std::{thread, io};
use std::time::{Duration, Instant};

use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::gpio::{Gpio, Trigger};
use crossbeam_channel::{unbounded, Sender, Receiver};
use crate::hardware::motors::{Motor, MotorDirection};
use std::io::Write;

enum Events{
    LeftWheelTick,
    RightWheelTick,
}

extern crate embedded_hal;
use embedded_hal::adc::OneShot;
use ads1x1x::{FullScaleRange, ModeChangeError};
use ads1x1x::DataRate16Bit::Sps860;

extern crate linux_embedded_hal;
#[macro_use(block)]
extern crate nb;
extern crate ads1x1x;

fn main() -> Result<(), Box<dyn Error>> {

    use hardware::line_sensor::LineSensor;
    let mut line_sensor = LineSensor::new();

    loop{
        let values = line_sensor.read_values();
        println!("Measurement: {:?}\n", values);
    }


    //    let mut right_motor = hardware::motors::RightMotor::new();
//    right_motor.set_direction(MotorDirection::Forward);
//    right_motor.set_power_0_to_1(0.15);
//
//    std::thread::sleep(Duration::from_millis(5000));
//    right_motor.set_power_0_to_1(0.0);
//
//    let mut right_motor = hardware::motors::LeftMotor::new();
//    right_motor.set_direction(MotorDirection::Forward);
//    right_motor.set_power_0_to_1(0.15);
//    std::thread::sleep(Duration::from_millis(5000));
//
//    right_motor.set_power_0_to_1(0.0);

//
//    let (s, r): (Sender<Events>, Receiver<Events>) = unbounded();
//    let mut right_motor = hardware::motors::RightMotor::new();
//    right_motor.set_direction(MotorDirection::Forward);
//    right_motor.set_power_0_to_1(0.15);
//
//    let gpio = Gpio::new().unwrap();
//    let right_wheel = gpio.get(20).expect("Error opening pin 20");
//    let mut right_wheel = right_wheel.into_input();
//
//    let async_sender = s.clone();
//    right_wheel.set_async_interrupt(Trigger::RisingEdge, move |_level|{
//        async_sender.send(Events::RightWheelTick).expect("Error sending event to main loop");
//    }).expect("Error setting interrupt on pin 20");
//
//    let mut last_time_tick = Instant::now();
//    let stdout = io::stdout();
//    let mut handle = stdout.lock();
//    for i in 0..=1000{
//        match r.recv().unwrap(){
//            Events::LeftWheelTick => {
//                println!("Left tick");
//            },
//            Events::RightWheelTick => {
//                let seconds_per_tick = last_time_tick.elapsed().as_micros() as f64/1_000_000.0;
//                let ticks_per_second = 1.0/seconds_per_tick;
//                let rpm = ticks_per_second/20.0;
//                handle.write_fmt(format_args!("RPM {}\n", rpm)).unwrap();
//                last_time_tick = Instant::now();
//            },
//        }
//    }
//    right_motor.set_power_0_to_1(0.0);
//    right_wheel.clear_async_interrupt().unwrap();
    Ok(())
}

