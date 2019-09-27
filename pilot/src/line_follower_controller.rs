use super::*;
use hardware::motors::MotorsConfig;
use crate::hardware::line_sensor::LinePosition;

trait LineFollowerController{
    fn new() -> Self;
    fn process_new_sensor_data(&mut self, outlier: Option<LinePosition>) -> MotorsConfig;
}

pub struct SimpleLineFollowerController{

}

impl LineFollowerController for SimpleLineFollowerController{
    pub fn new() -> Self{
        SimpleLineFollowerController{}
    }

    pub fn process_new_sensor_data(&mut self, maybe_outlier: Option<LinePosition>) -> MotorsConfig {
        let new_motor_config: MotorsConfig;
        if let Some(outlier) = maybe_outlier{
            match outlier{
                LinePosition::LineToTheFarLeft => {

                },
                LinePosition::LineToTheLeft => {},
                LinePosition::LineInTheCenter => {},
                LinePosition::LineToTheRight => {},
                LinePosition::LineToTheFarRight => {},
            }
        }
        unimplemented!()
    }
}
