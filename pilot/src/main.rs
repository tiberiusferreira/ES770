
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::gpio::{Gpio, Trigger};
use crossbeam_channel::{unbounded, Sender, Receiver};

// Servo configuration. Change these values based on your servo's verified safe
// minimum and maximum values.
//
// Period: 20 ms (50 Hz). Pulse width: min. 1200 µs, neutral 1500 µs, max. 1800 µs.
const PERIOD_US: u64 = 10;

#[derive(Debug)]
pub enum Events{
    RightWheelTick,
    LeftWheelTick,
}

struct State{
    left_wheel: Pwm,
    right_wheel: Pwm,
    left_speed_ticks_tmp: u32,
    left_curr_duty: f64,
    right_curr_duty: f64,
    right_speed_ticks_tmp: u32,
    right_speed_ticks_per_sec: u32,
    left_speed_ticks_per_sec: u32,
    last_update: std::time::Instant,
    desired_speed_ticks_per_sec: u32
}

impl State{
    fn set_left_duty(&mut self, duty_0_to_1: f64){
        if duty_0_to_1 > 0.6 && self.left_speed_ticks_per_sec < 3{
            self.left_wheel.set_duty_cycle(0.6).expect("Error setting duty!");
            self.left_curr_duty = duty_0_to_1;
        }else{
            self.left_wheel.set_duty_cycle(duty_0_to_1).expect("Error setting duty!");
            self.left_curr_duty = duty_0_to_1;
        }
    }
    fn set_right_duty(&mut self, duty_0_to_1: f64){
        if duty_0_to_1 > 0.6 && self.right_speed_ticks_per_sec < 3{
            self.right_wheel.set_duty_cycle(0.6).expect("Error setting duty!");
            self.right_curr_duty = duty_0_to_1;
        }else{
            self.right_wheel.set_duty_cycle(duty_0_to_1).expect("Error setting duty!");
            self.right_curr_duty = duty_0_to_1;
        }
    }
    fn handle_event(&mut self, event: Events){
        match event {
            Events::RightWheelTick => {
                self.right_speed_ticks_tmp = self.right_speed_ticks_tmp +1;
            },
            Events::LeftWheelTick => {
                self.left_speed_ticks_tmp = self.left_speed_ticks_tmp +1;
            },
        }
    }

    fn update_speed_reset_counters(&mut self){
        let time_since_last_sec = dbg!(self.last_update.elapsed().as_micros() as f64/1_000_000.0);

        dbg!(self.right_speed_ticks_per_sec);
        self.right_speed_ticks_per_sec = dbg!(((self.right_speed_ticks_tmp as f64)/time_since_last_sec) as u32);
        self.left_speed_ticks_per_sec = ((self.left_speed_ticks_tmp as f64)/time_since_last_sec) as u32;


        self.left_speed_ticks_tmp = 0;
        self.right_speed_ticks_tmp = 0;
        self.last_update = std::time::Instant::now();
    }

    fn update_motors_duty(&mut self){
        let amount_right_too_slow = (self.desired_speed_ticks_per_sec as f64 - self.right_speed_ticks_per_sec as f64);
        println!("right speed: {}", self.right_speed_ticks_per_sec);
        let amount_right_too_slow_norm = amount_right_too_slow/100.0;
        let mut output = amount_right_too_slow_norm/5.0 + self.right_curr_duty;
        if (output) >= 1.0{
            self.set_right_duty(1.0);
            output = 1.0;
        }  else if output <= 0.0 {
            self.set_right_duty(0.0);
            output = 0.0;

        }else{
            self.set_right_duty(output);
        }

        println!("duty right set to : {}", output);

        let amount_left_too_slow = (self.desired_speed_ticks_per_sec as f64 - self.left_speed_ticks_per_sec as f64);
        println!("left speed: {}", self.left_speed_ticks_per_sec);
        let amount_left_too_slow_norm = amount_left_too_slow/100.0;
        let mut output = amount_left_too_slow_norm/5.0 + self.left_curr_duty;
        if (output) >= 1.0{
            self.set_left_duty(1.0);
            output = 1.0;
        }  else if output <= 0.0 {
            self.set_left_duty(0.0);
            output = 0.0;

        }else{
            self.set_left_duty(output);
        }


        println!("duty left set to : {}", output);


    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let (s, r): (Sender<Events>, Receiver<Events>) = unbounded();



    let gpio = Gpio::new().expect("Could not get GPIO!");

    let right_wheel = gpio.get(16).expect("Error opening pin 16");
    let mut right_wheel = right_wheel.into_input();
    let async_sender = s.clone();
    right_wheel.set_async_interrupt(Trigger::RisingEdge, move |_level|{
        async_sender.send(Events::RightWheelTick).expect("Error sending event to main loop");
    }).expect("Error setting interrupt on pin 16");



    let left_wheel = gpio.get(20).expect("Error opening pin 16");
    let mut left_wheel = left_wheel.into_input();
    let async_sender = s.clone();
    left_wheel.set_async_interrupt(Trigger::RisingEdge, move |_level|{
        async_sender.send(Events::LeftWheelTick).expect("Error sending event to main loop");
    }).expect("Error setting interrupt on pin 20");



    let pwm0 = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_micros(PERIOD_US),
        Duration::from_micros(0),
        Polarity::Normal,
        true,
    )?;

    let pwm1 = Pwm::with_period(
        Channel::Pwm1,
        Duration::from_micros(PERIOD_US),
        Duration::from_micros(0),
        Polarity::Normal,
        true,
    )?;

    let mut state = State{
        left_wheel: pwm0,
        right_wheel: pwm1,
        left_speed_ticks_tmp: 0,
        left_curr_duty: 0.0,
        right_curr_duty: 0.0,
        right_speed_ticks_tmp: 0,
        right_speed_ticks_per_sec: 0,
        left_speed_ticks_per_sec: 0,
        last_update: std::time::Instant::now(),
        desired_speed_ticks_per_sec: 0
    };

    state.desired_speed_ticks_per_sec = 20;

    loop{
          while let Ok(event) = r.try_recv(){
//              println!("Got event! {:?}", event);
              state.handle_event(event);
          }
        state.update_speed_reset_counters();
        state.update_motors_duty();
        std::thread::sleep(Duration::from_millis(300));

    }

    Ok(())
}

