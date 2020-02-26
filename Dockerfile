#### BUILD
FROM balenalib/raspberry-pi:build as buildstep
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
WORKDIR /usr/src/app
COPY Cargo.lock Cargo.toml ./
COPY src src
RUN /bin/bash -c "source $HOME/.cargo/env && RUSTFLAGS='-Ccodegen-units=1' cargo build --release"


#### RUN
FROM balenalib/raspberry-pi:run as runstep
WORKDIR /bin
COPY --from=buildstep /usr/src/app/target/release/rpi-blinker ./
CMD ["/bin/rpi-blinker"]