use super::*;
use hardware::motors::{MotorsConfig, SingleMotorConfig};
use crate::hardware::line_sensor::LinePosition;
use crate::hardware::encoder::WheelTickData;

pub trait LineFollowerController{
    fn new() -> Self;
    fn process_new_sensor_data(&mut self, line_pos: i32, outlier: Option<LinePosition>) -> MotorsConfig;
}

pub struct SimpleLineFollowerController{
    acc_r: f64,
    acc_l: f64,
    kp_l: f64,
    kp_r: f64,
    ki_l: f64,
    ki_r: f64,
    last_outlier: u32,
    last_instant: Option<Instant>,
    last_err: f64
}

impl LineFollowerController for SimpleLineFollowerController{
    fn new() -> Self{
        SimpleLineFollowerController{
            acc_r: 0.0,
            acc_l: 0.0,
            kp_l: 1.0/(100.0),
            kp_r: 1.0/(100.0),
            ki_l: 1.0/(250_000.0),
            ki_r: 1.0/(200_000.0),
            last_outlier: 0,
            last_instant: None,
            last_err: 0.0
        }
    }

    fn process_new_sensor_data(&mut self, line_pos: i32, maybe_outlier: Option<LinePosition>) -> MotorsConfig {

//        let default_speed = 40.0;
//        if encoder_data.right_tps == 0.0{
//            self.acc_r = 0.0;
//        }
//        if encoder_data.left_tps == 0.0{
//            self.acc_l = 0.0;
//        }
//
        let default_power_l = 0.165*1.2; //*1.0;
        let default_power_r = 0.14*1.2;//*1.0;
//
//        let center_conf =  MotorsConfig{
//            left_config: SingleMotorConfig {
//                direction: MotorDirection::Forward,
//                power_0_to_1: 0.35*wall_power_norm_factor
//            },
//            right_config: SingleMotorConfig {
//                direction: MotorDirection::Forward,
//                power_0_to_1: 0.3*wall_power_norm_factor
//            }
//        };
//
//        let left_conf =  MotorsConfig{
//            left_config: SingleMotorConfig {
//                direction: MotorDirection::Brake,
//                power_0_to_1: 0.1*wall_power_norm_factor
//            },
//            right_config: SingleMotorConfig {
//                direction: MotorDirection::Forward,
//                power_0_to_1: 0.3*wall_power_norm_factor
//            }
//        };
//        let right_conf =  MotorsConfig{
//            left_config: SingleMotorConfig {
//                direction: MotorDirection::Forward,
//                power_0_to_1: 0.35*wall_power_norm_factor
//            },
//            right_config: SingleMotorConfig {
//                direction: MotorDirection::Brake,
//                power_0_to_1: 0.1*wall_power_norm_factor
//            }
//        };
        let (motor_power_l, motor_power_r): (f64, f64) = {
//            let new_outlier;
//            match maybe_outlier {
//                None => {
//                    new_outlier = self.last_outlier.clone();
//                },
//                Some(outlier) => {
//                    new_outlier = outlier;
//                    self.last_outlier = new_outlier.clone();
//                },
//            }
//
//            let pos;
//            match new_outlier{
//                LinePosition::LineToTheFarFarLeft => {
//                    println!("To the far far left!");
//                    pos = -3;
//                },
//                LinePosition::LineToTheFarLeft => {
//                    println!("To the far left!");
//                    pos = -2;
//                },
//                LinePosition::LineToTheLeft => {
//                    println!("To the left!");
//                    pos = -1;
//                },
//                LinePosition::LineInTheCenter => {
//                    println!("Center!");
//                    pos = 0;
//
//                },
//                LinePosition::LineToTheRight => {
//                    println!("Right!");
//                    pos = 1;
//                },
//                LinePosition::LineToTheFarRight => {
//                    println!("Far Right!");
//                    pos = 2;
//                },
//                LinePosition::LineToTheFarFarRight => {
//                    println!("Far Far Right!");
//                    pos = 3;
//                },
//            }
            let err = (3500-line_pos) as f64;
            let kp = 0.000015*5.0; // * 7.0 works
            let kd = 0.0001*5.0; // * 7.0 works
            let mut output = kp*err + kd*(err-self.last_err as f64);

//            output = output.min(0.3).max(-0.3);


            self.last_err = err;
            (default_power_l - output,default_power_r + output)

        };
        let left_config;
        let right_config;
        if motor_power_l > 0.0{
            left_config = SingleMotorConfig {
                direction: MotorDirection::Forward,
                power_0_to_1: motor_power_l
            };
        }else{
            left_config = SingleMotorConfig {
                direction: MotorDirection::Backwards,
                power_0_to_1: -motor_power_l
            };
        }

        if motor_power_r > 0.0{
            right_config = SingleMotorConfig {
                direction: MotorDirection::Forward,
                power_0_to_1: motor_power_r
            };
        }else{
            right_config = SingleMotorConfig {
                direction: MotorDirection::Backwards,
                power_0_to_1: -motor_power_r
            };
        }
        MotorsConfig{
            left_config,
            right_config
        }
    }
}
