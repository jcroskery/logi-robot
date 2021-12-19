FROM rustembedded/cross:aarch64-unknown-linux-gnu-0.2.1
ENV LD_LIBRARY_PATH=/build/lib/
ENV OpenCV_DIR=/build/
RUN dpkg --add-architecture arm64 && \
    echo "deb [arch=amd64] http://us.archive.ubuntu.com/ubuntu/ bionic main restricted universe" >> /etc/apt/sources.list && \
    touch /etc/apt/preferences.d/bionic.pref && \
    echo "Package: *" >> /etc/apt/preferences.d/bionic.pref && \
    echo "Pin: release n=xenial" >> /etc/apt/preferences.d/bionic.pref && \
    echo "Pin-Priority: -10" >> /etc/apt/preferences.d/bionic.pref && \
    echo "" >> /etc/apt/preferences.d/bionic.pref && \
    echo "Package: clang" >> /etc/apt/preferences.d/bionic.pref && \
    echo "Pin: release n=trusty" >> /etc/apt/preferences.d/bionic.pref && \
    echo "Pin-Priority: 500" >> /etc/apt/preferences.d/bionic.pref && \
    echo "" >> /etc/apt/preferences.d/bionic.pref && \
    echo "Package: libclang-dev" >> /etc/apt/preferences.d/bionic.pref && \
    echo "Pin: release n=trusty" >> /etc/apt/preferences.d/bionic.pref && \
    echo "Pin-Priority: 500" >> /etc/apt/preferences.d/bionic.pref && \
    apt-get update && \
    apt-get install --assume-yes g++-arm-linux-gnueabi g++-arm-linux-gnueabihf gcc-arm-linux-gnueabihf cmake clang libclang-dev && \
    git clone https://github.com/opencv/opencv.git && \
    mkdir build && \
    cd build && \
    cmake -DENABLE_VFPV3=ON -DENABLE_NEON=ON -DCMAKE_TOOLCHAIN_FILE=../opencv/platforms/linux/arm-gnueabi.toolchain.cmake ../opencv && \ 
    make -j4 && \
    make install
