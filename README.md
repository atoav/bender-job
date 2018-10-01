# bender_job

bender_job is a rust library, that serializes and deserializes jobs
from `data.json` files. The deserialization yields a Job struct.

It can be loaded into a rust project via git by putting this in your Cargo.toml:
```rust
[dependencies]
bender_job = { git = "ssh://git@code.hfbk.net:4242/bendercode/bender-job.git" }
```
To update this run
```rust
cargo clean
cargo update
```

### Testing
The libary is implemented with a extensive amount of tests to make
sure that repeated deserialization/serialization won't introduce
losses or glitches to the `data.json`. The tests can be run with
```rust
cargo test
```

### Documentation
If you want to view the documentation run
```rust
cargo doc --no-deps --open
```

### Installation
To run cargo, make sure you have rust installed. Go to [rustup.rs](http://rustup.rs) and follow the instructions there

