# USFM

An parser for the Unified Standard Markup Format (USFM).

## Setup

`usfm` is written in Rust, and requires a nightly build,
due to it's use of the `peg` syntax extensions.

I recommend getting the latest nightly Rust using multirust
(https://github.com/brson/multirust).
Install multirust, clone and enter this repository, then run:

    multirust override nightly
    cargo build


Note that this only builds the library, it doesn't use it.
There's currently no reference binary that uses this library.
