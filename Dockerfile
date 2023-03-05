FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["hilmm"]
