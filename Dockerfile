FROM espressif/idf:v4.4.6

# Set working directory
WORKDIR /app

# Copy the Rust code
COPY . .

# Install Rust and targets
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    /root/.cargo/bin/rustup toolchain install esp && \
    /root/.cargo/bin/rustup target add xtensa-esp32s3-espidf

# Set environment
ENV PATH="/root/.cargo/bin:${PATH}"

# Build the project
RUN /root/.cargo/bin/cargo +esp build --release --target xtensa-esp32s3-espidf -Z build-std=core,alloc,std,panic_abort

# Output binary location
RUN ls -lh /app/target/xtensa-esp32s3-espidf/release/sma_test

CMD ["/bin/bash"]
