# H-Bridge

## Right Motor

echo "18" > /sys/class/gpio/export  ena a PWM1 RIGHTs
echo "5" > /sys/class/gpio/export    in 1
echo "6" > /sys/class/gpio/export    in 2
echo "13" > /sys/class/gpio/export  in 3
echo "26" > /sys/class/gpio/export  in 4
echo "19" > /sys/class/gpio/export  ena b PWM0 LEFT

# Wheel Encoder
echo "20" > /sys/class/gpio/export    Right Wheel
echo "21" > /sys/class/gpio/export    Left Wheel