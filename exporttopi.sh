#Every time
git push
ssh -t pi@169.254.163.150 "cd rpr; git pull; systemctl --user stop logi; /home/pi/.cargo/bin/cargo run"
# systemctl --user stop logi; systemctl --user start logi
