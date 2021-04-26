#!/bin/bash
modprobe v4l2loopback exclusive_caps=1 max_buffers=2
gphoto2 --stdout --capture-movie | ffmpeg -i - -vcodec rawvideo -threads 0 -vf format=yuv420p -f v4l2 /dev/video0
