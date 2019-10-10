#!/bin/bash
cross build --release --target arm-unknown-linux-musleabihf #arm-unknown-linux-gnueabihf
scp /Users/tiberiodarferreira/github/ES770/pilot/target/arm-unknown-linux-musleabihf/release/pilot pi@raspberrypi.local:/home/pi/pilot
