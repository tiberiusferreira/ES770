use ads1x1x::{FullScaleRange, Ads1x1x, interface::I2cInterface, channel};
use ads1x1x::DataRate16Bit::{Sps860};
use std::time::{Duration, Instant};
use linux_embedded_hal::i2cdev::core::I2CDevice;
use ads1x1x::ic::Resolution16Bit;
use ads1x1x::ic::Ads1115;
use linux_embedded_hal::I2cdev;
use crate::embedded_hal::adc::OneShot;

pub struct LineSensor{
    adc: Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>,
    adc1: Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>,
}


impl LineSensor{
    pub fn new() -> Self{

        use ads1x1x::{Ads1x1x, SlaveAddr};
        use linux_embedded_hal::I2cdev;

        let dev = I2cdev::new("/dev/i2c-1").unwrap();
        let address = SlaveAddr::Alternative(false, false);
        let mut adc = Ads1x1x::new_ads1115(dev, address);
        adc.set_data_rate(Sps860).unwrap();
        adc.set_full_scale_range(FullScaleRange::Within4_096V).unwrap();

        let dev2 = I2cdev::new("/dev/i2c-1").unwrap();
        let address2 = SlaveAddr::Alternative(false, true);
        let mut adc2 = Ads1x1x::new_ads1115(dev2, address2);
        adc2.set_data_rate(Sps860).unwrap();
        adc2.set_full_scale_range(FullScaleRange::Within4_096V).unwrap();

        std::thread::sleep(Duration::from_millis(500));
        Self{
            adc,
            adc1: adc2,
        }
    }

    pub fn read_values(&mut self) -> [i16; 8]{
        let mut output_left_right: [i16; 8] = [0; 8];
        let start = Instant::now();

        self.adc.read(&mut channel::SingleA0).unwrap_err();
        self.adc1.read(&mut channel::SingleA0).unwrap_err();
        let ch0 = block!(self.adc.read(&mut channel::SingleA0)).unwrap();
        let ch0_2 = block!(self.adc1.read(&mut channel::SingleA0)).unwrap();


        self.adc.read(&mut channel::SingleA1).unwrap_err();
        self.adc1.read(&mut channel::SingleA1).unwrap_err();
        let ch1 = block!(self.adc.read(&mut channel::SingleA1)).unwrap();
        let ch1_2 = block!(self.adc1.read(&mut channel::SingleA1)).unwrap();

        self.adc.read(&mut channel::SingleA2).unwrap_err();
        self.adc1.read(&mut channel::SingleA2).unwrap_err();
        let ch2 = block!(self.adc.read(&mut channel::SingleA2)).unwrap();
        let ch2_2 = block!(self.adc1.read(&mut channel::SingleA2)).unwrap();

        self.adc.read(&mut channel::SingleA3).unwrap_err();
        self.adc1.read(&mut channel::SingleA3).unwrap_err();
        let ch3 = block!(self.adc.read(&mut channel::SingleA3)).unwrap();
        let ch3_2 = block!(self.adc1.read(&mut channel::SingleA3)).unwrap();



//            println!("New Conversion took {} ms", start.elapsed().as_millis());


        output_left_right[7] = ch1_2;
        output_left_right[6] = ch0_2;
        output_left_right[5] = ch3;
        output_left_right[4] = ch2;
        output_left_right[3] = ch1;
        output_left_right[2] = ch0;
        output_left_right[1] = ch2_2;
        output_left_right[0] = ch3_2;
        output_left_right
    }

    pub fn find_line(&mut self, reference_values: [i16; 8]) -> LineInfo{
        let values = self.read_values();
        let outliers = self.get_outliers(values, reference_values);

        let mut abs_diff_values: [i32; 8] = [0; 8];
        for (pos, value) in values.iter().enumerate(){
            abs_diff_values[pos] = (value - reference_values[pos]).abs().into();
        }
        let mut numerator = 0;
        for (pos, value) in abs_diff_values.iter().enumerate(){
            numerator = numerator + 1000*(pos as i32)*(*value);
        }
        let sum: i32 = abs_diff_values.iter().sum();
        let res = numerator/sum;
        LineInfo{
            position: res,
            outliers
        }

    }

    pub fn get_outliers(&mut self, values: [i16; 8], reference_values: [i16; 8]) -> Vec<Outlier>{
        let mut outliers_vec: Vec<Outlier> = Vec::with_capacity(8);
        // Get all the outliers (more than 35% difference from reference value)
        for (pos, value) in values.iter().enumerate(){
            let reference_value = reference_values.get(pos).unwrap();
            if (value-reference_value).abs() > ((0.35*(*reference_value) as f64) as i16){
                let diff = (100*(value-*reference_value).abs() as u32/ (*reference_value) as u32) as u8;
                outliers_vec.push(Outlier{
                    position: pos as u8,
                    difference_from_reference_percentage: diff
                });
            }
        }

        return outliers_vec;

    }
}

#[derive(Debug, Clone)]
pub struct Outlier{
    position: u8,
    difference_from_reference_percentage: u8
}

#[derive(Debug, Clone)]
pub struct LineInfo{
    pub position: i32,
    pub outliers: Vec<Outlier>
}