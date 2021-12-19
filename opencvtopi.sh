#One time only
CONTAINER_ID=$(docker create cross-build)
docker cp $CONTAINER_ID:/build/ /home/justus/Rust/Rust/rpr/
docker rm $CONTAINER_ID
scp -r /home/justus/Rust/Rust/rpr/build/ pi@169.254.163.150:/build