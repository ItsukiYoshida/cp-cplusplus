FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y \
        python3 \
        python3-pip \
        build-essential \
        cmake \
        ninja-build \
        wget \
        curl \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN cd /tmp && \
    wget https://archives.boost.io/release/1.88.0/source/boost_1_88_0.tar.gz && \
    tar -zxvf boost_1_88_0.tar.gz && \
    cd boost_1_88_0 && \
    chmod +x bootstrap.sh && \
    ./bootstrap.sh --prefix=/opt/boost && \
    ./b2 install -j$(nproc)

ENV BOOST_ROOT=/opt/boost
ENV CMAKE_PREFIX_PATH=$BOOST_ROOT

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspace
