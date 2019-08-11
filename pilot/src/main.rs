
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
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 1200;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 20;


pub enum Events{
    RightWheelTick,
}

struct State{

}

fn main() -> Result<(), Box<dyn Error>> {
    let (s, r): (Sender<Events>, Receiver<Events>) = unbounded();



//    let gpio = Gpio::new().expect("Could not get GPIO!");
//
//    let right_wheel = gpio.get(16).expect("Error opening pin 16");
//
//    let mut right_wheel = right_wheel.into_input();
//
//    let async_sender = s.clone();
//    right_wheel.set_async_interrupt(Trigger::RisingEdge, move |_level|{
//        async_sender.send(Events::RightWheelTick).expect("Error sending event to main loop");
//    }).expect("Error setting interrupt on pin 16");
//
//    let mut tick_counter = 0;
//
//    for i in 0..=100 {
//        let event = match r.recv_timeout(Duration::from_secs(10)){
//            Ok(event) => event,
//            Err(err) => {
//                println!("Timed out");
//                continue;
//            },
//        };
//        match event {
//            Events::RightWheelTick => {
//                tick_counter = tick_counter+1;
//                println!("Current count: {}", tick_counter);
//            },
//        }
//    }


        let pwm0 = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_millis(5),
        Polarity::Normal,
        true,
    )?;

//    let pwm1 = Pwm::with_period(
//        Channel::Pwm1,
//        Duration::from_millis(PERIOD_MS),
//        Duration::from_millis(PULSE_MAX_US),
//        Polarity::Normal,
//        true,
//    )?;
//
    thread::sleep(Duration::from_secs(1));
//
//    pwm0.set_pulse_width(Duration::from_millis(10))?;
//    pwm1.set_pulse_width(Duration::from_millis(10))?;
//
//
//    thread::sleep(Duration::from_secs(3));
//
//
//    pwm0.set_pulse_width(Duration::from_millis(5))?;
//    pwm1.set_pulse_width(Duration::from_millis(10))?;
//
//    thread::sleep(Duration::from_secs(3));

    Ok(())
}