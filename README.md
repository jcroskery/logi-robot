# logi-robot
This code runs on a Raspberry PI, and interfaces with the [rprc](https://github.com/jcroskery/rprc/) repository.
## Robot Specs
The Raspberry Pi is connected to a GPIO breakout board. 
I used an old Meccanoid G15 set lying around to construct the frame of the robot. 
I also used the motors and wheels from this Meccano kit. 
In addition, the robot features a Logitech C920 webcam, an ultrasonic sensor, and a combined gyroscope/accelerometer.
## Controls
This robot is controlled by a website. Any device with local internet access can control the robot. 
## Performance
- The robot can successfully navigate a flat obstacle course thanks to its ability to move both forward and backward. 
- The gyroscope/accelerometer only displays its raw readings in a table --- I found that attempting to calculate the velocity of the robot with the accelerometer yielded imprecise results.
## Picture
![logi.jpg](https://github.com/jcroskery/logi-robot/blob/logi/logi.JPG?raw=true)
