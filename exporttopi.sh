#Every time
git push
ssh -t pi@192.168.0.234 "cd rpr; git pull; systemctl --user stop logi; /home/pi/.cargo/bin/cargo run"
