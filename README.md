# job

bender-job is a rust library, that serializes and deserializes jobs
from `data.json` files. The deserialization yields a Job struct.

The libary is implemented with a extensive amount of tests to make
sure that repeated deserialization/serialization won't introduce
losses or glitches to the `data.json`. The tests can be run with
`cargo test`


