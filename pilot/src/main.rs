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
    println!("Calibration done with {:?}", reference_values);
    println!("Waiting for input to begin");
    io::stdin().read_line(&mut input).unwrap();
    println!("Began!");



    let mut controller = line_follower_controller::SimpleLineFollowerController::new();
//    let mut encoders = hardware::encoder::WheelEncoders::new();
//    encoders.start_listening_to_events();
    loop {
        let start = Instant::now();

        let line_info = line_sensor.find_line(reference_values);
        let new_conf = controller.process_new_sensor_data(line_info);
        motors.apply_config(new_conf);
        let elapsed = start.elapsed().as_millis() as u64;
//        println!("elapsed {}", elapsed);
        let cycle_period_ms = 18;
        if elapsed < cycle_period_ms {
            std::thread::sleep(Duration::from_millis((cycle_period_ms - elapsed as u64)));
        }


//        let wheel_tick_data = encoders.get_speed_tps();
//        let left_revs_per_sec = wheel_tick_data.left_tps/20.0;
//        let radius = 6.25/2.0;
//        let cm_per_sec_left = left_revs_per_sec*2.0*3.14*radius;
//        let right_revs_per_sec = wheel_tick_data.right_tps/20.0;
//        let cm_per_sec_right = right_revs_per_sec*2.0*3.14*radius;
//        println!("LTicks: {:3.2} Left cm/s: {:3.2}     RTicks: {:3.2} Right cm/s: {:3.2} ", wheel_tick_data.left_tps, cm_per_sec_left, wheel_tick_data.right_tps,  cm_per_sec_right);


    }
//    Ok(())
}


