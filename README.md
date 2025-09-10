# Todd C. Miller `readpassphrase` source
This crate provides a vendored implementation of the BSD [readpassphrase][0] source code. That’s all it does. It mainly exists to be optionally depended upon by [readpassphrase-3][1] so that the ISC license needn’t mandatorily be pulled in by that crate’s dependents.

# License
This crate is made available under the terms of the ISC license. The trivial Rust wrapper is released into the public domain.

[0]: https://man.openbsd.org/readpassphrase
[1]: https://crates.io/crates/readpassphrase-3
