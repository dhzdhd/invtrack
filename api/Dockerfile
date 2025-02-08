# Stage 1: Build the Rust binary
FROM rust:1.84-slim AS builder

RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .

RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Stage 2: Create AWS Lambda environment and setup Rust binary and environment
# https://docs.aws.amazon.com/lambda/latest/dg/images-create.html#runtimes-images-provided
FROM public.ecr.aws/lambda/provided:al2023

RUN dnf update -y && \
    dnf install -y \
    gcc \
    gcc-c++ \
    make \
    wget \
    tar \
    openssl-devel \
    postgresql-devel

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/api /var/runtime/bootstrap

ENV LANG=en_US.UTF-8
ENV TZ=:/etc/localtime
ENV PATH=/var/lang/bin:/usr/local/bin:/usr/bin/:/bin:/opt/bin
ENV LD_LIBRARY_PATH=/var/lang/lib:/lib64:/usr/lib64:/var/runtime:/var/runtime/lib:/var/task:/var/task/lib:/opt/lib
ENV LAMBDA_TASK_ROOT=/var/task
ENV LAMBDA_RUNTIME_DIR=/var/runtime
ENV RUNTIME_ENTRYPOINT=/var/runtime/bootstrap
ENV DATABASE_URL=hi

EXPOSE 8080

WORKDIR /var/task

ENTRYPOINT ["/lambda-entrypoint.sh"]
CMD [ "/var/runtime/bootstrap" ]
