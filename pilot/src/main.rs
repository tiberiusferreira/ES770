mod hardware;
mod line_follower_controller;
use line_follower_controller::*;
use std::error::Error;
use std::{thread, io};
use std::time::{Duration, Instant};

use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::gpio::{Gpio, Trigger};
use crossbeam_channel::{unbounded, Sender, Receiver};
use crate::hardware::motors::{Motor, Motors, MotorDirection};
use hardware::line_sensor::LineSensor;
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

const DEFAULT_MOTOR_POWER: f64 = 0.2;


impl Default for MotorsConfig{
    fn default() -> Self {
        let default_motor_config = SingleMotorConfig { direction: MotorDirection::Forward, speed: DEFAULT_MOTOR_POWER };
        MotorsConfig{
            left_config: default_motor_config.clone(),
            right_config: default_motor_config
        }
    }
}



fn main() -> Result<(), Box<dyn Error>> {

    let mut motors = Motors::new();


    let mut line_sensor = LineSensor::new();



    let mut input = String::new();
    println!("Waiting for input to calibrate");
    io::stdin().read_line(&mut input).unwrap();
    let reference_values = line_sensor.read_values();

    println!("Calibration done with {:?}", reference_values);
    println!("Waiting for input to begin");
    io::stdin().read_line(&mut input).unwrap();
    println!("Began!");


    motors.change_power_both(DEFAULT_MOTOR_POWER);

    let mut controller = line_follower_controller::SimpleLineFollowerController::new();

    let mut slept = false;
    loop{
        let start = Instant::now();
        if let Some(outlier) =  line_sensor.find_line(reference_values){
            let new_motor_config = controller.process_new_sensor_data(Some(outlier));
            motors.apply_config(new_motor_config);
            slept = false;
        } else{
            if slept == false{
                std::thread::sleep(Duration::from_millis(350));
                slept = true;
            }else {
                right_motor.set_power_0_to_1(0.0);
                left_motor.set_power_0_to_1(0.0);
            }
            println!("No outliers for some time, stopping!");
        }
        println!("Took: {}ms", start.elapsed().as_millis());
    }

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

pub fn single_outlier_to_motor_power_0_to_1(outlier: usize) -> (f64, f64){
    // from 0 to 2, means we have to turn right
    // 3 or 4 we are OK
    // from 5 to 7, means we have to turn left
    let distance_from_center = ((outlier as f64 - 3.0)*1.0).min(0.3);
    // if is negative, we need to turn right
    // if is positive, we need to turn left
    let delta = distance_from_center/10.0;
    if delta > 0.0{
        (delta, delta)
    }else{
        (delta, -delta)
    }


}
