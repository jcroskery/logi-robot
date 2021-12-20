FROM rustembedded/cross:aarch64-unknown-linux-gnu-0.2.1
ENV LD_LIBRARY_PATH=/opt/opencv/lib/
ENV OpenCV_DIR=/opt/opencv/
COPY opencv /opt/
COPY sources .
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ bionic main restricted universe multiverse" >> "/etc/apt/sources.list" && \
    cat sources >> /etc/apt/preferences && \
    apt-get update && \
    apt-get install --assume-yes clang-6.0:armhf libclang-6.0-dev:armhf gcc-5-arm-linux-gnueabihf  g++-5-arm-linux-gnueabihf
    
