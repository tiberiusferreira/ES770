use super::*;
use hardware::motors::{MotorsConfig, SingleMotorConfig};
use crate::hardware::line_sensor::LinePosition;
use crate::hardware::encoder::WheelTickData;

pub trait LineFollowerController{
    fn new() -> Self;
    fn process_new_sensor_data(&mut self, line_pos: i32, outlier: Option<LinePosition>, encoder_data: WheelTickData) -> MotorsConfig;
}

pub struct SimpleLineFollowerController{
    acc_r: f64,
    acc_l: f64,
    kp_l: f64,
    kp_r: f64,
    ki_l: f64,
    ki_r: f64,
    last_outlier: LinePosition,
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
            last_outlier: LinePosition::LineInTheCenter,
            last_instant: None,
            last_err: 0.0
        }
    }

    fn process_new_sensor_data(&mut self, line_pos: i32, maybe_outlier: Option<LinePosition>, encoder_data: WheelTickData) -> MotorsConfig {

//        let default_speed = 40.0;
//        if encoder_data.right_tps == 0.0{
//            self.acc_r = 0.0;
//        }
//        if encoder_data.left_tps == 0.0{
//            self.acc_l = 0.0;
//        }
//
        let default_power_l = 0.165*0.75; //*1.0;
        let default_power_r = 0.14*0.75;//*1.0;
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
            let kp = 0.0000015*10.0; // * 7.0 works
            let kd = 0.00001*10.0; // * 7.0 works
            let output = kp*err + kd*(err-self.last_err as f64);


            println!("PL: {:1.3} PR: {:1.3}", default_power_l + output, default_power_r - output);
            println!("PKp: {:1.3} Pkd: {:1.3}", kp*err + output, kd*(err-self.last_err as f64));
            println!("Err-last_err: {}", err-self.last_err);
            self.last_err = err;
            (default_power_l - output,default_power_r + output)

        };
//        else{
//            println!("No outlier in controller!");
//            (0.0, 0.0)
////            self.acc_r =0.0;
////            self.acc_l =0.0;
////            return MotorsConfig{
////                left_config: SingleMotorConfig {
////                    direction: MotorDirection::Forward,
////                    power_0_to_1: 0.0
////                },
////                right_config: SingleMotorConfig {
////                    direction: MotorDirection::Forward,
////                    power_0_to_1: 0.0
////                }
////            }
//
//        };
        let elapsed_millis = match self.last_instant{
            None => {
                0.0
            },
            Some(last_instant) => {
                last_instant.elapsed().as_millis() as f64
            },
        };
//        let error_r = (goal_right - encoder_data.right_tps);
//        self.acc_r = self.acc_r + self.ki_r*error_r*elapsed_millis;
//        let output_r = self.kp_r *error_r + self.acc_r;
//        let motor_power_r = output_r.max(0.0).min(1.0) + default_power_r;
//
//        let error_l = (goal_left - encoder_data.left_tps);
//        self.acc_l = self.acc_l + self.ki_l *error_l*elapsed_millis;
//        let output_l = self.kp_l *error_l + self.acc_l;
//        let motor_power_l = output_l.max(0.0).min(1.0) + default_power_l;

//        println!("Speed: {speed_left:3.1} EL: {:2.1} PL :{:1.2}             Speed: {speed_right:3.1} ER: {:2.1} PR :{:1.2}",error_l, motor_power_l, error_r, motor_power_r, speed_right=encoder_data.right_tps, speed_left=encoder_data.left_tps);
//        println!("AL: {:1.2}                        AR :{:1.2}", self.acc_l, self.acc_r);
        self.last_instant = Some(Instant::now());

        MotorsConfig{
            left_config: SingleMotorConfig {
                direction: MotorDirection::Forward,
                power_0_to_1: motor_power_l
            },
            right_config: SingleMotorConfig {
                direction: MotorDirection::Forward,
                power_0_to_1: motor_power_r
            }
        }
    }
}
