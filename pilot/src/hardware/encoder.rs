use rppal::gpio::{Gpio, Trigger, InputPin};
use crossbeam_channel::{Sender, Receiver, unbounded};
use std::sync::{Arc, RwLock};
use std::time::{Instant, Duration};
use arraydeque::{ArrayDeque, Wrapping};

struct WheelTickDataInternal {
    wheel_speed_historic_data: ArrayDeque<[f64; 5], Wrapping>,
    last_tick: Instant,
}

pub struct WheelTickData{
    pub left_tps: f64,
    pub right_tps: f64
}
pub struct WheelEncoders {
    left_wheel_data: Arc<RwLock<WheelTickDataInternal>>,
    right_wheel_data: Arc<RwLock<WheelTickDataInternal>>,
    right_wheel_pin: InputPin,
    left_wheel_pin: InputPin
}

enum Events{
    LeftWheelTick,
    RightWheelTick,
}

impl WheelEncoders {
    pub fn new() -> Self{
        let default_tick_data = || WheelTickDataInternal {
            last_tick: Instant::now(),
            wheel_speed_historic_data: ArrayDeque::new()
        };

        let gpio = Gpio::new().unwrap();

        let right_wheel = gpio.get(20).expect("Error opening pin 20");
        let right_wheel = right_wheel.into_input();


        let left_wheel = gpio.get(21).expect("Error opening pin 20");
        let left_wheel = left_wheel.into_input();

        Self{
            left_wheel_data: Arc::new(RwLock::new(default_tick_data())),
            right_wheel_data: Arc::new(RwLock::new(default_tick_data())),
            right_wheel_pin: right_wheel,
            left_wheel_pin: left_wheel
        }
    }
    pub fn start_listening_to_events(&mut self){
        let (s, r): (Sender<Events>, Receiver<Events>) = unbounded();


        let async_sender = s.clone();
        self.right_wheel_pin.set_async_interrupt(Trigger::RisingEdge, move |_level|{
            async_sender.send(Events::RightWheelTick).expect("Error right wheel tick event");
        }).expect("Error setting interrupt on pin 20");

        let async_sender = s.clone();
        self.left_wheel_pin.set_async_interrupt(Trigger::RisingEdge, move |_level|{
            async_sender.send(Events::LeftWheelTick).expect("Error left wheel tick event");
        }).expect("Error setting interrupt on pin 21");

        let left_wheel_data_handle = self.left_wheel_data.clone();
        let right_wheel_data_handle = self.right_wheel_data.clone();
        std::thread::spawn(move ||{
            let tick_handler = |data_handle: Arc<RwLock<WheelTickDataInternal>>|{
                let mut handle_lock = data_handle.write().expect("increasing wheel_tick");
                let time_elapsed_nano = handle_lock.last_tick.elapsed().as_nanos();
                if time_elapsed_nano > 100*(1_000_000_000/1000){
                    println!("More than 100 ms passed since last tick, setting RPM is 0");
                    handle_lock.wheel_speed_historic_data.push_back(0.0);
                }else if time_elapsed_nano < (1_000_000_000/1_000) {// if less than 1 ms, debounce it
                    // Do nothing
                }else{
                    let tps = (1_000_000_000.0) as f64 / time_elapsed_nano as f64;
                    handle_lock.wheel_speed_historic_data.push_back(tps);
                }
                handle_lock.last_tick = Instant::now();
            };
            loop{
                let event = r.recv().expect("receiving events");

                match event{
                    Events::LeftWheelTick => {
                        tick_handler(left_wheel_data_handle.clone());
                    },
                    Events::RightWheelTick => {
                        tick_handler(right_wheel_data_handle.clone());
                    },
                }
            }

        });
    }

    pub fn get_speed_tps(&mut self) -> WheelTickData{
        let (left_rps, right_rps) = {
            let left = self.left_wheel_data.read().expect("reading left wheel data");
            let right = self.right_wheel_data.read().expect("reading right wheel data");
            let left_rpm;
            let right_rpm;
            if left.last_tick.elapsed().as_millis() > 80{
                left_rpm = ArrayDeque::new();
            }else{
                left_rpm = left.wheel_speed_historic_data.clone();
            }
            if right.last_tick.elapsed().as_millis() > 80{
                right_rpm = ArrayDeque::new();
            }else{
                right_rpm = right.wheel_speed_historic_data.clone();
            }
            (left_rpm, right_rpm)
        };

        let filter_array = |ar : ArrayDeque<[f64; 5], Wrapping>|{
            if ar.is_empty(){
                return 0.0;
            }
            let raw_mean: f64 = ar.iter().sum::<f64>()/(ar.iter().count() as f64);
            let without_outliers: Vec<&f64> = ar.iter().filter(|e| {
                (((**e as f64)-raw_mean) as f64).abs() < 0.3*raw_mean
            }).collect();
            if without_outliers.is_empty(){
                return raw_mean;
            }
            let mean: f64 = without_outliers.iter().map(|e| *e).sum::<f64>()/(without_outliers.iter().count() as f64);
            mean
        };

        WheelTickData{
            left_tps: filter_array(left_rps),
            right_tps: filter_array(right_rps)
        }
    }
}