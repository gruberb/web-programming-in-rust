FROM ekidd/rust-musl-builder:nightly AS build
COPY . ./
RUN sudo chown -R rust:rust .
RUN cargo build --release

FROM scratch
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/YOUR_APP_NAME /
ENV PORT 8181
EXPOSE ${PORT}
CMD ["/YOUR_APP_NAME"]