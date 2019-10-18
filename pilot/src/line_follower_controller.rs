use super::*;
use hardware::motors::{MotorsConfig, SingleMotorConfig};
use crate::hardware::line_sensor::{LineInfo};
use crate::hardware::encoder::WheelTickData;
use std::ops::Sub;

pub trait LineFollowerController{
    fn new() -> Self;
    fn process_new_sensor_data(&mut self, line_pos: LineInfo) -> MotorsConfig;
}

pub struct SimpleLineFollowerController{
    kp: f64,
    kd: f64,
    slow_mode_start: Instant,
    default_power_l: f64,
    default_power_r: f64,
    slow_mode_multiplier: f64,
    line_goal: i32,
    last_line_pos: i32,
    last_instant: Instant,
    last_time_had_new_line_pos: Instant,
    last_time_had_less_than_4: Instant,
    last_err: f64
}

impl LineFollowerController for SimpleLineFollowerController{
    fn new() -> Self{
        SimpleLineFollowerController{
            kp: 0.000015*8.5,
            kd: 0.0001*14.0,
            slow_mode_multiplier: 0.5,
            slow_mode_start: Instant::now().sub(Duration::from_millis(5000)),
            default_power_l: 0.165*2.0,
            default_power_r: 0.14*2.0,
            line_goal: 3100, //goes from 0 to 7000, but is not mounted dead center
            last_line_pos: 0,
            last_instant: Instant::now(),
            last_time_had_new_line_pos: Instant::now(),
            last_time_had_less_than_4: Instant::now(),
            last_err: 0.0
        }
    }

    fn process_new_sensor_data(&mut self, mut line_info: LineInfo) -> MotorsConfig {



        if (line_info.outliers.is_empty() && self.last_time_had_new_line_pos.elapsed().as_millis() > 150) ||
            (line_info.outliers.len() == 8 && self.last_time_had_less_than_4.elapsed().as_millis() > 250){

            if line_info.outliers.len() == 8 && self.last_time_had_less_than_4.elapsed().as_millis() > 250 {
                println!("8 outliers Stopping");
            }
//            println!("Stopping");

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

        if line_info.outliers.len() > 4{
            println!("More than 4, going straight");
            return MotorsConfig{
                left_config: SingleMotorConfig {
                    direction: MotorDirection::Forward,
                    power_0_to_1: self.default_power_l
                },
                right_config: SingleMotorConfig {
                    direction: MotorDirection::Forward,
                    power_0_to_1: self.default_power_r
                }
            }
        }else{
            self.last_time_had_less_than_4 = Instant::now();
        }

        if line_info.position < 3100-1000 || line_info.position > 3100+1000{
            self.slow_mode_start = Instant::now();
        }
        if line_info.outliers.is_empty(){
            println!("Using old line pos");
            line_info.position = self.last_line_pos;
        }else{
            self.last_time_had_new_line_pos = Instant::now();
        }

        let (motor_power_l, motor_power_r): (f64, f64) = {

            let err = (self.line_goal- line_info.position) as f64;
            let output = self.kp*err + self.kd*(err-self.last_err as f64);

            self.last_err = err;
            if self.slow_mode_start.elapsed().as_millis() < 500 {
                println!("Slow mode");
                ((self.default_power_l - output)*self.slow_mode_multiplier, (self.default_power_r + output)*self.slow_mode_multiplier)
            }else{
            (self.default_power_l - output, self.default_power_r + output)
            }

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
