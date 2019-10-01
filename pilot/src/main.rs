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

const DEFAULT_MOTOR_POWER: f64 = 0.0;



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
    let mut encoders = hardware::encoder::WheelEncoders::new();
    let mut slept = false;
    encoders.start_listening_to_events();

    loop{
        println!("{}", line_sensor.find_line2(reference_values));
        let start = Instant::now();
        let wheel_tick_data = encoders.get_speed_tps();
//        let maybe_line = line_sensor.find_line(reference_values);
        let line_as_i32 = line_sensor.find_line2(reference_values);
        let new_conf = controller.process_new_sensor_data(line_as_i32, None, wheel_tick_data);
        motors.apply_config(new_conf);
        let elapsed = start.elapsed().as_millis();
        println!("elapsed millis: {}", elapsed);
        if  elapsed < 5{
            println!("slept");
            std::thread::sleep(Duration::from_millis((5-elapsed as u64)));
        }
    }
//    loop{
//        let start = Instant::now();
//        if let Some(outlier) =  line_sensor.find_line(reference_values){
//            let new_motor_config = controller.process_new_sensor_data(Some(outlier), encoders.get_speed_tps());
//            motors.apply_config(new_motor_config);
//            slept = false;
//        } else{
//            if slept == false{
//                std::thread::sleep(Duration::from_millis(350));
//                slept = true;
//            }else {
//                motors.right_motor.set_power_0_to_1(0.0);
//                motors.left_motor.set_power_0_to_1(0.0);
//            }
//            println!("No outliers for some time, stopping!");
//        }
//        println!("Took: {}ms", start.elapsed().as_millis());
//    }
    Ok(())
}


