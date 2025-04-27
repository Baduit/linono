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

# Install python
RUN apt install python3 python3-pip python3.12-venv -y

# Install linono in a venv
ENV VIRTUAL_ENV=/linono/venv
RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"
RUN python3 -m pip install ./linono_pyextractor
RUN python3 -m pip install ./linono_cli
