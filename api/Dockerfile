# Stage 1: Build the Rust binary
FROM rust:1.84-slim AS builder

# Install cross-compilation dependencies if needed
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .

EXPOSE 8080

# Build for AWS Lambda (x86_64-unknown-linux-gnu)
RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu

# https://docs.aws.amazon.com/lambda/latest/dg/images-create.html#runtimes-images-provided
FROM scratch

ADD aws_lambda_files/0916ed56093e485098093d3c0b4ff1f4f846e5d606cb876ad19044d1d21713b5.tar.xz /
ADD aws_lambda_files/19a47b91636f78233baa1b0f6745195981e9cfe85e62a6baa7b94f3c1112cef2.tar.xz /
ADD aws_lambda_files/dcbca33e18f9091440dc4a8e1779a11393791713d66b602a7cd38254f2dcaa4a.tar.xz /
ADD aws_lambda_files/dd04e2fcc4e60652f76b700fc394cf024cf7a4c2adb41440de9d6f2fcc1e3791.tar.xz /

# Extra
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/api /var/runtime/bootstrap

ENV LANG=en_US.UTF-8
ENV TZ=:/etc/localtime
ENV PATH=/var/lang/bin:/usr/local/bin:/usr/bin/:/bin:/opt/bin
ENV LD_LIBRARY_PATH=/var/lang/lib:/lib64:/usr/lib64:/var/runtime:/var/runtime/lib:/var/task:/var/task/lib:/opt/lib
ENV LAMBDA_TASK_ROOT=/var/task
ENV LAMBDA_RUNTIME_DIR=/var/runtime
ENV RUNTIME_ENTRYPOINT=/var/runtime/bootstrap

EXPOSE 8080

WORKDIR /var/task

# ENTRYPOINT ["/lambda-entrypoint.sh"]
CMD [ "/lambda-entrypoint.sh", "bootstrap" ]
# CMD [ "/usr/local/bin/aws-lambda-rie" "/var/runtime/bootstrap" ]