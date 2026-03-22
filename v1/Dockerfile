FROM debian:oldstable-slim

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    valgrind \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash rustuser
USER rustuser
WORKDIR /home/rustuser

ENV RUSTUP_HOME=/home/rustuser/.rustup
ENV CARGO_HOME=/home/rustuser/.cargo
ENV PATH=/home/rustuser/.cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

WORKDIR /home/rustuser/app

CMD ["/bin/bash"]
