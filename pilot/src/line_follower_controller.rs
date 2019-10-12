use super::*;
use hardware::motors::{MotorsConfig, SingleMotorConfig};
use crate::hardware::line_sensor::{LineInfo};
use crate::hardware::encoder::WheelTickData;

pub trait LineFollowerController{
    fn new() -> Self;
    fn process_new_sensor_data(&mut self, line_pos: LineInfo) -> MotorsConfig;
}

pub struct SimpleLineFollowerController{
    kp: f64,
    kd: f64,
    default_power_l: f64,
    default_power_r: f64,
    line_goal: i32,
    last_line_pos: i32,
    last_instant: Instant,
    last_time_had_new_line_pos: Instant,
    last_err: f64
}

impl LineFollowerController for SimpleLineFollowerController{
    fn new() -> Self{
        SimpleLineFollowerController{
            kp: 0.000015*3.0,
            kd: 0.0001*7.5,
            default_power_l: 0.165*1.0,
            default_power_r: 0.14*1.0,
            line_goal: 3100, //goes from 0 to 7000, but is not mounted dead center
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

        let (motor_power_l, motor_power_r): (f64, f64) = {

            let err = (self.line_goal- line_info.position) as f64;
            let output = self.kp*err + self.kd*(err-self.last_err as f64);

            self.last_err = err;
            (self.default_power_l - output,self.default_power_r + output)

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
