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
    default_power_l: f64,
    default_power_r: f64,
    line_goal: i32,
    last_line_pos: i32,
    last_instant: Instant,
    last_time_had_new_line_pos: Instant,
    //    last_time_had_less_than_4: Instant,
    last_time_saw_end_line: Instant,
    last_err: f64,
    instant_stopped: Instant,
    times_passed_end_line: u8,
    consecutive_times_maybe_saw_end_line: u32,
    number_times_needs_see_end_line_to_declare_actual_end_line: u32,
    stopped: bool
}

impl SimpleLineFollowerController{
    fn processed_end_line(&mut self, line_info: &LineInfo) -> bool{
        if self.last_time_saw_end_line.elapsed().as_millis() < 1000{
            // that must be at least 1 second between end line detections
            return false;
        }

        if line_info.outliers.len() == 8{
            if self.consecutive_times_maybe_saw_end_line >= self.number_times_needs_see_end_line_to_declare_actual_end_line {
                self.times_passed_end_line += 1;
                self.last_time_saw_end_line = Instant::now();
                self.consecutive_times_maybe_saw_end_line = 0;
                println!("Saw line!");
                if self.times_passed_end_line >= 2 {
                    println!("Saw two times, stopping!");
                    self.stopped = true;
                    self.instant_stopped = Instant::now();
                }
            }else{
                self.consecutive_times_maybe_saw_end_line += 1;
                println!("Maybe saw end line {} times.", self.consecutive_times_maybe_saw_end_line);
            }
            return true;
        }else{
            self.consecutive_times_maybe_saw_end_line = 0;
            return false;
        }
    }

    fn stop_routine(&mut self) -> MotorsConfig{
        if self.instant_stopped.elapsed().as_millis() < 100{
            return MotorsConfig{
                left_config: SingleMotorConfig {
                    direction: MotorDirection::Backwards,
                    power_0_to_1: 0.5
                },
                right_config: SingleMotorConfig {
                    direction: MotorDirection::Backwards,
                    power_0_to_1: 0.5
                }
            }
        }else{
            return MotorsConfig{
                left_config: SingleMotorConfig {
                    direction: MotorDirection::Brake,
                    power_0_to_1: 0.0
                },
                right_config: SingleMotorConfig {
                    direction: MotorDirection::Brake,
                    power_0_to_1: 0.0
                }
            }
        }
    }

    fn go_straight_motor_conf(&self) -> MotorsConfig {
        return MotorsConfig {
            left_config: SingleMotorConfig {
                direction: MotorDirection::Forward,
                power_0_to_1: self.default_power_l
            },
            right_config: SingleMotorConfig {
                direction: MotorDirection::Forward,
                power_0_to_1: self.default_power_r
            }
        }
    }

    fn stopped_motor_conf(&self) -> MotorsConfig {
        return MotorsConfig {
            left_config: SingleMotorConfig {
                direction: MotorDirection::Brake,
                power_0_to_1: 0.5
            },
            right_config: SingleMotorConfig {
                direction: MotorDirection::Brake,
                power_0_to_1: 0.5
            }
        }
    }

    fn no_line_and_been_a_while_since_saw_line(&self, line_info: &LineInfo) -> bool {
        return line_info.outliers.is_empty() && self.last_time_had_new_line_pos.elapsed().as_millis() > 500;
    }


    fn controller_output(&mut self, line_info: &LineInfo) -> MotorsConfig {
        let (motor_power_l, motor_power_r): (f64, f64) = {
            let err = (self.line_goal - line_info.position) as f64;
            let output = self.kp*err + self.kd*(err-self.last_err as f64);

            self.last_err = err;
            (self.default_power_l - output, self.default_power_r + output)
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

impl LineFollowerController for SimpleLineFollowerController{
    fn new() -> Self{
        SimpleLineFollowerController{
            kp: 0.000015*6.5,
            kd: 0.0001*12.0,
            default_power_l: 0.165*1.4,
            default_power_r: 0.14*1.4,
            line_goal: 3100, //goes from 0 to 7000, but is not mounted dead center
            last_line_pos: 0,
            last_instant: Instant::now(),
            last_time_had_new_line_pos: Instant::now(),
//            last_time_had_less_than_4: Instant::now(),
            instant_stopped: Instant::now(),
            last_time_saw_end_line: Instant::now(),
            last_err: 0.0,
            times_passed_end_line: 0,
            consecutive_times_maybe_saw_end_line: 0,
            number_times_needs_see_end_line_to_declare_actual_end_line: 4,
            stopped: false
        }
    }



    fn process_new_sensor_data(&mut self, mut line_info: LineInfo) -> MotorsConfig {

        let mut diffs = Vec::new();
        for outlier in line_info.outliers.clone(){
            diffs.push(outlier.difference_from_reference_percentage);
        }
        println!("{:?}", diffs);

        if self.processed_end_line(&line_info){
            return self.go_straight_motor_conf();
        }


        if self.no_line_and_been_a_while_since_saw_line(&line_info) {
            return self.stopped_motor_conf();
        }

        if self.stopped  {
            return self.stop_routine();
        }

        if line_info.outliers.len() > 4{
            println!("More than 4 outliers, going straight");
            return self.go_straight_motor_conf();
        }


        if line_info.outliers.is_empty(){
            println!("Using old line pos");
            line_info.position = self.last_line_pos;
        }else{
            self.last_time_had_new_line_pos = Instant::now();
        }

        // No special case, we have good line info!

        return self.controller_output(&line_info);
    }
}
