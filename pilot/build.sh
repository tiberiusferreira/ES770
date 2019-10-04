#!/bin/bash
cross build --release --target arm-unknown-linux-musleabihf #arm-unknown-linux-gnueabihf
scp /Users/tiberiodarferreira/github/ES770/pilot/target/arm-unknown-linux-musleabihf/release/pilot pi@192.168.15.27:/home/pi/pilot
