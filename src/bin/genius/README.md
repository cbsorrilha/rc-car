# Spec: Three-Color Genius Game

Status: approved by Cesar on `2026-08-27` — ready for technical planning; implementation has not started.

## Objective

Build a playable three-color Genius game on the existing `rc-car` ESP32-S3 prototype. The ESP32 presents a growing LED sequence and the player reproduces it with an Xbox controller. The challenge exists primarily to practice Rust domain modeling, modules, state transitions, ownership, embedded boundaries, and automated tests.

The accepted user-visible result is:

> A player can start the firmware, observe a growing sequence of blue, green, and red LEDs, reproduce it with X, A, and B, receive distinct success or error feedback, and reset the game at any time with Y.

## Controls And Hardware Contract

| Xbox button | Serial command | Game meaning |
|---|---:|---|
| X | `B` | Blue |
| A | `G` | Green |
| B | `R` | Red |
| Y | `Y` | Reset |

The existing blue, green, and red LEDs are sequence LEDs. The two additional LEDs are referred to by role, not color:

- **correct LED:** round completed successfully;
- **error LED:** wrong input.

Existing GPIO assignments must be preserved unless Cesar explicitly approves a wiring change.

## Game Rules

### Startup

1. The three sequence LEDs blink together three times using `100 ms` on and `100 ms` off.
2. All sequence LEDs remain off for `1,000 ms`.
3. The game clears prior progress, creates a one-color sequence, and starts the first round.

The correct and error LEDs are not part of the startup animation.

### Sequence Playback

1. Each round replays the entire existing sequence in order.
2. A successfully completed round appends exactly one randomly selected color; repeated adjacent colors are allowed.
3. Each sequence LED remains on for `500 ms` and off for `250 ms` before the next color.
4. X, A, and B inputs are ignored during playback.
5. Y resets the game even during playback.

### Player Input

1. After playback, the game waits indefinitely for controller input; v1 has no response timeout.
2. Each accepted X, A, or B input briefly lights the corresponding sequence LED using the same `500 ms` on and `250 ms` off timing.
3. The input is compared with the sequence position currently expected.
4. A correct partial input advances to the next expected position without replaying the sequence.
5. Completing the entire sequence finishes the round successfully.
6. A wrong input ends the current game immediately.

### Successful Round

1. The correct LED blinks three times using `100 ms` on and `100 ms` off.
2. If the completed sequence has fewer than `10` colors, one new random color is appended.
3. The complete extended sequence is played and the game waits for the next player response.
4. If the player completes a sequence of `10` colors, no color is appended; the game enters game-completed and waits for Y.

### Wrong Input

1. The error LED blinks three times using `100 ms` on and `100 ms` off.
2. The game enters a game-over state.
3. X, A, and B are ignored while game-over is active.
4. The game remains in game-over until Y is pressed.

### Reset

Y may reset the game from playback, player input, success feedback, error feedback, or game-over.

Reset performs the same visible sequence as startup:

1. immediately abandon the current sequence and input position;
2. blink the three sequence LEDs together three times;
3. leave them off for `1,000 ms`;
4. create a new one-color sequence and begin again.

## Behavioral States

The implementation must make these states observable in the pure game model, even if their Rust names differ:

| State | Responsibility | Accepted inputs |
|---|---|---|
| Startup | Run the startup/reset signal and create round one | Y restarts startup |
| Playing sequence | Present the stored sequence | Y resets; color input ignored |
| Awaiting input | Compare player colors in order | X/A/B compare; Y resets |
| Round success | Signal success and extend the sequence | Y resets |
| Game over | Signal failure on entry, then wait without progressing | Y resets; color input ignored |
| Game completed | Preserve the completed 10-color result and wait | Y resets; color input ignored |

Hardware timing must not be embedded in the pure state-transition logic. The model should emit intents such as “show blue,” “signal success,” or “reset”; the firmware adapter performs GPIO and delays.

`RoundError` is intentionally not a separate state. A wrong color transitions directly from awaiting input to game over. Entering game over is the one-time signal for the firmware adapter to blink the error LED; remaining in game over must not repeat that feedback. This keeps the persistent state distinct from the transition effect without adding a redundant intermediate state.

## Randomness And Determinism

- Production chooses each appended color from blue, green, or red with equal eligibility.
- The pure game model must accept the next generated color from outside rather than directly depending on ESP32 randomness.
- If a generated color is supplied when the sequence already contains `10` colors, the event is ignored and the state remains unchanged. V1 does not require an error return for this invalid request.
- Host tests provide deterministic colors so every sequence can be reproduced.
- No external random-number dependency may be added without approval.

## Tech Stack

- Rust edition `2024`, minimum declared Rust version `1.88`.
- ESP Rust toolchain from `rust-toolchain.toml`.
- Target `xtensa-esp32s3-none-elf`.
- `no_std` firmware using `esp-hal ~1.1.0`.
- ESP32-S3 DevKitC-1 with the existing GPIO wiring.
- USB Serial/JTAG input from the existing Xbox-to-serial bridge.
- A dependency-free `no_std` pure game-core crate that can also run tests on the host.

No new third-party dependency is part of this feature.

## Commands

From the repository root:

```sh
# Format verification
cargo fmt --all -- --check

# Pure game model tests on the host
cargo +stable test --manifest-path crates/genius-core/Cargo.toml

# ESP32 compilation
cargo check --bin rc-car

# ESP32 lint
cargo clippy --bin rc-car -- -D warnings

# Flash and monitor on connected hardware
cargo run --release --bin rc-car
```

The host-test command becomes required only after the planned `crates/genius-core` crate exists. Creating that internal crate is part of this feature; adding an external dependency is not.

## Project Structure

Expected structure after implementation:

```text
crates/
└── genius-core/
    ├── Cargo.toml             # dependency-free no_std crate
    └── src/
        └── lib.rs             # colors, states, events, transitions and unit tests
src/
├── bin/
│   ├── main.rs               # firmware entrypoint
│   ├── blink_with_xbox/      # existing prototype; preserve until migration is verified
│   └── genius/
│       ├── README.md         # this specification
│       ├── mod.rs            # firmware orchestration
│       └── init.rs           # ESP32 peripherals, serial input, LEDs and delays
└── lib.rs                    # existing reusable hardware helpers
```

The exact number of source files inside `genius-core` may evolve after the first tests. Do not create abstractions without a demonstrated responsibility.

## Code Style

Prefer domain names and exhaustive matches over raw bytes or booleans inside the game model:

```rust
#![no_std]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Blue,
    Green,
    Red,
}

pub enum Input {
    Color(Color),
    Reset,
}
```

- Format with `rustfmt`.
- Use English for code, tests, commits, and repository documentation.
- Keep serial-byte decoding and GPIO types outside the pure game model.
- Prefer exhaustive `match` expressions for states, colors, and inputs.
- Avoid `unsafe`, heap allocation, and cloning unless the spec is revised with a concrete need.
- Use named timing constants in the firmware adapter:

```rust
const SEQUENCE_ON_MS: u64 = 500;
const SEQUENCE_OFF_MS: u64 = 250;
const START_DELAY_MS: u64 = 1_000;
const SIGNAL_ON_MS: u64 = 100;
const SIGNAL_OFF_MS: u64 = 100;
const SIGNAL_BLINKS: usize = 3;
```

“Configurable timing” means changing these named constants at compile time; v1 has no runtime settings interface.

## Testing Strategy

### Host Unit Tests

The pure core must have deterministic unit tests covering at least:

- a new game starts with one supplied color;
- correct partial input advances only the expected position;
- a full correct sequence completes the round;
- round completion preserves the prior sequence and appends one supplied color;
- wrong input transitions directly to game-over and exposes that entry once for error feedback;
- color input is ignored during playback, game-over, and game-completed;
- reset clears sequence progress and input position from every state;
- repeated adjacent colors are accepted;
- completing the 10-color sequence enters game-completed without appending another color;
- supplying another generated color at the supported sequence capacity leaves the state unchanged and never panics or corrupts state.

Tests must not sleep, access GPIO, read serial bytes, or depend on real randomness.

### Target Verification

- `cargo check` and Clippy validate the ESP32 integration.
- A hardware demonstration validates GPIO timing, serial decoding, controller mapping, and reset responsiveness.
- Passing host tests does not prove wiring or controller behavior; passing the hardware demo does not replace state-transition tests.

## Boundaries

### Always

- Preserve the existing GPIO assignments unless the spec is explicitly revised.
- Keep game decisions independent from GPIO, serial, delays, and randomness.
- Ignore X/A/B during sequence playback, game-over, and game-completed.
- Accept Y as reset from every state.
- Run format, host tests, target check, and Clippy before declaring completion.
- Let Cesar write the implementation; tutoring should use progressive hints and review his attempts.

### Ask First

- Add any third-party dependency.
- Change GPIO assignments or the USB serial command contract.
- Change the sequence capacity or define a special win state.
- Replace or remove the existing `blink_with_xbox` prototype before Genius passes the hardware demonstration.
- Modify build configuration, toolchain configuration, or the Xbox bridge repository.

### Never

- Add motor control, direct Xbox Bluetooth, sound, display, scoring persistence, networking, or multiplayer to this feature.
- Count color input received during playback or after game-over.
- Block Y reset until an animation finishes.
- Use real delays or hardware in host unit tests.
- Hide failing checks, remove tests to make verification pass, or claim embedded correctness from host tests alone.

## Acceptance Criteria

1. On boot, the three sequence LEDs blink together exactly three times, remain off for `1,000 ms`, and then begin a one-color round.
2. Sequence playback preserves order and uses `500 ms` on plus `250 ms` off per color.
3. X, A, and B map to blue, green, and red respectively through serial commands `B`, `G`, and `R`.
4. Color input during playback is ignored and does not alter the expected input position.
5. Each accepted player color lights the corresponding sequence LED once.
6. Correct partial input advances exactly one position.
7. Completing a round blinks the correct LED three times, appends exactly one color, and replays the complete extended sequence.
8. Wrong input blinks the error LED three times and leaves the game waiting for Y without extending the sequence.
9. Y interrupts and resets the game from every state, including animations, then follows the startup flow.
10. The sequence may contain the same color consecutively.
11. Completing a sequence of exactly 10 colors blinks the correct LED three times, does not append an eleventh color, and waits for Y in game-completed.
12. The pure game model passes deterministic host tests without ESP32 hardware, sleeps, serial input, or real randomness.
13. The firmware passes formatting, target compilation, and Clippy checks.
14. On hardware, an end-to-end demonstration completes at least three rounds, demonstrates one wrong input, and demonstrates Y reset during playback, game-over, and game-completed.
15. No motor-control, Bluetooth, audio, display, persistent score, or networking behavior is added.

## Expected Artifacts

- This approved specification.
- A dependency-free pure game-core crate with host tests.
- ESP32 Genius orchestration under `src/bin/genius/`.
- Serial decoding for `B`, `G`, `R`, and `Y`.
- A concise README update or demonstration note recording the verified hardware behavior.

## Assessment Dimensions

The tutor reviews the finished challenge separately for:

1. functional behavior against acceptance criteria;
2. understanding of Rust modules and crate boundaries;
3. ownership and bounded sequence storage;
4. state modeling and exhaustive transitions;
5. separation of pure logic from embedded effects;
6. deterministic tests and target verification;
7. explanation of at least one important design decision by Cesar.

## Approved Decisions

1. V1 supports a maximum sequence of `10` colors. Completing round 10 signals success and waits for Y without appending another color.
2. Pure logic lives in the internal, dependency-free `crates/genius-core` crate so it can be tested independently from `esp-hal`.
3. Startup and reset blink only the three sequence LEDs; the dedicated correct and error LEDs remain off.
4. Sequence and player-color flashes use `500 ms` on and `250 ms` off. Startup, correct, and error signals use `100 ms` on and `100 ms` off.
5. `RoundError` is not a separate state; the transition into game-over triggers error feedback exactly once.
6. Supplying a generated color at the 10-color capacity is ignored without returning an error.
