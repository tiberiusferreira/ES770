use super::*;
use hardware::motors::{MotorsConfig, SingleMotorConfig};
use crate::hardware::line_sensor::{LinePosition, LineInfo};
use crate::hardware::encoder::WheelTickData;

pub trait LineFollowerController{
    fn new() -> Self;
    fn process_new_sensor_data(&mut self, line_pos: LineInfo) -> MotorsConfig;
}

pub struct SimpleLineFollowerController{
    acc_r: f64,
    acc_l: f64,
    kp_l: f64,
    kp_r: f64,
    ki_l: f64,
    ki_r: f64,
    last_line_pos: i32,
    last_instant: Instant,
    last_time_had_new_line_pos: Instant,
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
            last_line_pos: 0,
            last_instant: Instant::now(),
            last_time_had_new_line_pos: Instant::now(),
            last_err: 0.0
        }
    }

    fn process_new_sensor_data(&mut self, mut line_info: LineInfo) -> MotorsConfig {

        if (line_info.outliers.is_empty() && self.last_time_had_new_line_pos.elapsed().as_millis() > 250) || line_info.outliers.len() > 3{
            println!("Stopping");

            return MotorsConfig{
                left_config: SingleMotorConfig {
                    direction: MotorDirection::Brake,
                    power_0_to_1: 0.0
                },
                right_config: SingleMotorConfig {
                    direction: MotorDirection::Brake,
                    power_0_to_1: 0.0 }
            }
        }
        if line_info.outliers.is_empty(){
            println!("Using old line pos");
            line_info.position = self.last_line_pos;
        }else{
            self.last_time_had_new_line_pos = Instant::now();
        }
        let default_power_l = 0.165*4.5; //*1.0;
        let default_power_r = 0.14*4.5;//*1.0;

        let (motor_power_l, motor_power_r): (f64, f64) = {

            let err = (3100- line_info.position) as f64;
            let kp = 0.000015*5.0; // * 7.0 works
            let kd = 0.0001*9.0; // * 7.0 works
            let mut output = kp*err + kd*(err-self.last_err as f64);

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
