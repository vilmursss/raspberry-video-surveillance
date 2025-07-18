FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    gawk \
    wget \
    git \
    diffstat \
    unzip \
    texinfo \
    build-essential \
    chrpath \
    socat \
    cpio \
    python3 \
    xz-utils \
    debianutils \
    iputils-ping \
    locales \
    file \
    lz4 \
    zstd \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN locale-gen en_US.UTF-8

ENV LANG=en_US.UTF-8
ENV LANGUAGE=en_US:en
ENV LC_ALL=en_US.UTF-8

WORKDIR /workdir
