# bender_job

bender-job is a rust library, that serializes and deserializes jobs
from `data.json` files. The deserialization yields a Job struct.

It can be loaded into a rust project via git by putting this in your Cargo.toml:
```
[dependencies]
bender-job = { git = "https://github.com/atoav/bender-job.git" }
```
To update this run
```
cargo clean
cargo update
```

### Testing
The libary is implemented with a extensive amount of tests to make
sure that repeated deserialization/serialization won't introduce
losses or glitches to the `data.json`. The tests can be run with
```
cargo test
```
*Note:* some tests might fail on your system, because the test jobs use absolute \
paths. Run `cargo test` a _second_ time to test with updated paths

### Documentation
If you want to view the documentation run
```
cargo doc --no-deps --open
```

### Installation
To run cargo, make sure you have rust installed. Go to [rustup.rs](http://rustup.rs) and follow the instructions there


License: MIT
