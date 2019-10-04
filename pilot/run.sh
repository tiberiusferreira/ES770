#!/bin/bash
cross build --target arm-unknown-linux-musleabihf #arm-unknown-linux-gnueabihf
scp /Users/tiberiodarferreira/github/ES770/pilot/target/arm-unknown-linux-musleabihf/debug/pilot pi@192.168.15.27:/home/pi/pilot
ssh pi@raspberrypi.local './pilot'
