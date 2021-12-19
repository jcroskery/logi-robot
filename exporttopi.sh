#Every time
cross build --release --target=armv7-unknown-linux-gnueabihf
ssh -t pi@169.254.163.150 'systemctl --user stop logi'
scp /home/justus/Rust/Rust/rpr/target/armv7-unknown-linux-gnueabihf/release/rpr pi@169.254.163.150:/home/pi/logi/rpr
ssh -t pi@169.254.163.150 'systemctl --user start logi'
