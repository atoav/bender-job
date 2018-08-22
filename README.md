# bender_job

bender_job is a rust library, that serializes and deserializes jobs
from `data.json` files. The deserialization yields a Job struct.

It can be loaded in a rust library via the public git mirror:
```rust
job = { git = "https://github.com/atoav/bender-job.git" }
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

