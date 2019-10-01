#!/bin/bash
cross build --target arm-unknown-linux-musleabihf #arm-unknown-linux-gnueabihf
scp /Users/tiberiodarferreira/github/ES770/pilot/target/arm-unknown-linux-musleabihf/debug/pilot pi@raspberrypi.local:/home/pi/pilot
ssh pi@raspberrypi.local './pilot'
