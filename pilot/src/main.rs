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

    let mut right_motor = hardware::motors::RightMotor::new();
    right_motor.set_direction(MotorDirection::Forward);
    let mut left_motor = hardware::motors::LeftMotor::new();
    left_motor.set_direction(MotorDirection::Forward);
    right_motor.set_power_0_to_1(0.0);
    left_motor.set_power_0_to_1(0.0);
    std::thread::sleep(Duration::from_secs(2));

    use hardware::line_sensor::LineSensor;
    let mut line_sensor = LineSensor::new();

//    loop{
//        let outlier = line_sensor.read_values();
//        println!("{:?}", outlier);
//
//    }
    let mut input = String::new();
    println!("Waiting for input to calibrate");
    io::stdin().read_line(&mut input).unwrap();
    let reference_values = line_sensor.read_values();

    println!("Calibration done with {:?}", reference_values);
    println!("Waiting for input to begin");
    io::stdin().read_line(&mut input).unwrap();
    println!("Began!");


    let default_power = 0.3;
    right_motor.set_power_0_to_1(default_power);
    left_motor.set_power_0_to_1(default_power);
    loop{
        let start = Instant::now();
        let outliers = line_sensor.find_line(reference_values);
//        println!("{:?}", outliers);
        // 3 should be the closest to the middle
        if outliers.len() == 1{
            let outlier = outliers.get(0).unwrap();
            let (delta_left, delta_right) = single_outlier_to_delta_motor_speed_0_to_1(*outlier);
//            let old_right = right_motor.get_power_0_to_1();
//            let old_left = left_motor.get_power_0_to_1();
            let (left, right) = (((default_power + delta_left)*2.0).max(0.0).min(0.7), (default_power + delta_right).max(0.0).min(0.7));
            right_motor.set_power_0_to_1(right);
            left_motor.set_power_0_to_1(left);
            println!("Motors: {} {}", left, right);
//            println!("Delta {:?} // {:?}", delta_left, delta_right);
//            println!("Final {:?} // {:?}", left, right);
        }else if outliers.len() == 0{
            right_motor.set_power_0_to_1(0.0);
            left_motor.set_power_0_to_1(0.0);
//            println!("No outliers, stopping!");
        }
        println!("Took: {}ms", start.elapsed().as_millis());
    }


//    right_motor.set_power_0_to_1(0.2);
//    left_motor.set_power_0_to_1(0.2);





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

pub fn single_outlier_to_delta_motor_speed_0_to_1(outlier: usize) -> (f64, f64){
    // from 0 to 2, means we have to turn right
    // from 4 to 7, means we have to turn left
    let distance_from_center = (outlier as f64 - 3.0)*0.75;
    // if is negative, we need to turn right
    // if is positive, we need to turn left
    let delta = distance_from_center/10.0;
    if delta > 0.0{
        (delta, delta)
    }else{
        (delta, -delta)
    }


}
