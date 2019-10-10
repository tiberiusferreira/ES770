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
use line_follower_controller::*;


extern crate embedded_hal;
use embedded_hal::adc::OneShot;
use ads1x1x::{FullScaleRange, ModeChangeError};
use ads1x1x::DataRate16Bit::Sps860;

extern crate linux_embedded_hal;
#[macro_use(block)]
extern crate nb;
extern crate ads1x1x;



fn main() -> Result<(), Box<dyn Error>> {

    let mut motors = Motors::new();


    let mut line_sensor = LineSensor::new();

    let mut input = String::new();
    println!("Waiting for input to calibrate");
    io::stdin().read_line(&mut input).unwrap();
    let reference_values = line_sensor.read_values();
//    loop{
//        let line_info = line_sensor.find_line(reference_values);
//        println!("{:?}", line_info.position);
//
//    }
    println!("Calibration done with {:?}", reference_values);
//    loop {
//        let reference_values = line_sensor.read_values();
//
//        println!("Calibration done with {:?}", reference_values);
//    }
    println!("Waiting for input to begin");
    io::stdin().read_line(&mut input).unwrap();
    println!("Began!");



    let mut controller = line_follower_controller::SimpleLineFollowerController::new();
    //let mut encoders = hardware::encoder::WheelEncoders::new();
    let mut slept = false;
    //encoders.start_listening_to_events();
    //let wheel_tick_data = encoders.get_speed_tps();
    loop {
//        println!("{:?}", line_sensor.find_line(reference_values));
        let start = Instant::now();

        let line_info = line_sensor.find_line(reference_values);
        let new_conf = controller.process_new_sensor_data(line_info);
        motors.apply_config(new_conf);
        let elapsed = start.elapsed().as_millis();
        println!("elapsed {}", elapsed);
        if elapsed < 15 {
            std::thread::sleep(Duration::from_millis((15 - elapsed as u64)));
        }
    }
    Ok(())
}


