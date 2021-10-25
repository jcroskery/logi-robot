# rpr (Raspberry Pi Robot)
This code runs on a Raspberry PI, and interfaces with the [rprc](https://github.com/jcroskery/rprc/) repository.
## Robot Specs
The Raspberry Pi is connected to a GPIO breakout board. 
All the robot's electronics are connected to the board --- making it very crowded!
I used an old Meccanoid G15 set lying around to construct the frame of the robot. 
I also used the servos, wheels, and lights from this Meccano kit. 
In addition, the robot features an ultrasonic sensor mounted on a stepper motor as well as a combined gyroscope/accelerometer.
## Controls
This robot is controlled by a website. I used an old computer to host the site on my local network and used an iPad to control the robot. 
## Performance
- The robot can successfully navigate a flat obstacle course thanks to its ability to move both forward and backward. 
- The servos were a huge pain to get working --- they would only respond to signals if the bit delay used to communicate with them was precisely 417 ns. Eventually I managed to get the servos working about 90 % of the time.
- The stepper motor rotates 360° and the ultrasonic sensor creates a graph showing the distance to obstacles within a meter of the robot.
- The gyroscope/accelerometer only displays its raw readings in a table --- I found that attempting to calculate the velocity of the robot with the accelerometer yielded imprecise results.
# Disassembly
Eventually, I took apart the robot. It was a successful creation, but I wanted to make something new with my Raspberry Pi.
