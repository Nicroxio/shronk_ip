# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY src ./src

COPY dbip-city-lite-2023-10.mmdb ./

# Build the application
RUN cargo build --release


# Set the command to run the application
EXPOSE 8080

CMD ["/app/target/release/shronk_ip"]


# CMD ["/app/target/release/shronk_ip"]