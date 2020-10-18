FROM liuchong/rustup
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
run mkdir /hop-mill
WORKDIR /hop-mill
ADD ./hop-mill /hop-mill
RUN rustup default nightly
RUN cargo build
CMD ["cargo", "run"]
