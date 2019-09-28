use super::*;
use hardware::motors::{MotorsConfig, SingleMotorConfig};
use crate::hardware::line_sensor::LinePosition;

pub trait LineFollowerController{
    fn new() -> Self;
    fn process_new_sensor_data(&mut self, outlier: Option<LinePosition>) -> MotorsConfig;
}

pub struct SimpleLineFollowerController{

}

impl LineFollowerController for SimpleLineFollowerController{
    fn new() -> Self{
        SimpleLineFollowerController{}
    }

    fn process_new_sensor_data(&mut self, maybe_outlier: Option<LinePosition>) -> MotorsConfig {
        let new_motor_config: MotorsConfig;
        let default_power = 0.5;
        if let Some(outlier) = maybe_outlier{
            match outlier{
                LinePosition::LineToTheFarLeft => {
                    // go to the left, hard
                    new_motor_config = MotorsConfig{
                        left_config: SingleMotorConfig {
                            direction: MotorDirection::Backwards,
                            power_0_to_1: default_power
                        },
                        right_config: SingleMotorConfig {
                            direction: MotorDirection::Forward,
                            power_0_to_1: default_power
                        }
                    }
                },
                LinePosition::LineToTheLeft => {
                    new_motor_config = MotorsConfig{
                        left_config: SingleMotorConfig {
                            direction: MotorDirection::Backwards,
                            power_0_to_1: default_power
                        },
                        right_config: SingleMotorConfig {
                            direction: MotorDirection::Forward,
                            power_0_to_1: default_power
                        }
                    }
                },
                LinePosition::LineInTheCenter => {
                    new_motor_config = MotorsConfig{
                        left_config: SingleMotorConfig {
                            direction: MotorDirection::Forward,
                            power_0_to_1: default_power
                        },
                        right_config: SingleMotorConfig {
                            direction: MotorDirection::Forward,
                            power_0_to_1: default_power
                        }
                    }
                },
                LinePosition::LineToTheRight => {
                    new_motor_config = MotorsConfig{
                        left_config: SingleMotorConfig {
                            direction: MotorDirection::Forward,
                            power_0_to_1: default_power
                        },
                        right_config: SingleMotorConfig {
                            direction: MotorDirection::Backwards,
                            power_0_to_1: default_power
                        }
                    }

                },
                LinePosition::LineToTheFarRight => {
                    new_motor_config = MotorsConfig{
                        left_config: SingleMotorConfig {
                            direction: MotorDirection::Forward,
                            power_0_to_1: default_power
                        },
                        right_config: SingleMotorConfig {
                            direction: MotorDirection::Backwards,
                            power_0_to_1: default_power
                        }
                    }
                },
            }
        }else{
            println!("No outlier in controller!");
            new_motor_config = MotorsConfig{
                left_config: SingleMotorConfig {
                    direction: MotorDirection::Forward,
                    power_0_to_1: 0.0
                },
                right_config: SingleMotorConfig {
                    direction: MotorDirection::Forward,
                    power_0_to_1: 0.0
                }
            }
        }
        new_motor_config
    }
}
