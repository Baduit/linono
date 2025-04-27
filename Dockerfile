FROM ubuntu:24.10

WORKDIR /linono
COPY . /linono/

ENV DEBIAN_FRONTEND noninteractive
RUN apt update

# Install essential
RUN apt install build-essential -y

# Install Curl to be able to install other tools
RUN apt install curl -y

# Install rust toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Rye
RUN curl -sSf https://rye.astral.sh/get | RYE_INSTALL_OPTION="--yes" bash
ENV PATH=/root/.rye/shims:$PATH


# Build
RUN rye sync
