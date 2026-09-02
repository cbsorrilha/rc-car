# Xbox Bridge

macOS development tool that reads the Xbox controller through `gilrs` and writes
one-byte commands to the ESP32 USB serial port. It is a workspace member, but it
is not firmware and must not be compiled for the ESP32 target.

## Run

From the repository root on an Apple Silicon Mac:

```sh
cargo +stable run -p xbox-bridge --target aarch64-apple-darwin --release
```

The repository defaults Cargo to the ESP32-S3 target, so both `+stable` and the
explicit macOS target are required for this host tool.

The serial-port path is currently configured in `src/main.rs`. Stop the serial
monitor before opening the port with the bridge.
