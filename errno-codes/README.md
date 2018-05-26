[![Build Status](https://api.travis-ci.org/im-0/errno-codes.svg?branch=b0.1.0)](https://travis-ci.org/im-0/errno-codes)
[![crates.io](https://img.shields.io/crates/v/errno-codes.svg?maxAge=3600)](https://crates.io/crates/errno-codes)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache_2.0-blue.svg)

# errno-codes

Rust library for dealing with `errno` codes sent over the network from
different operation systems and/or CPU architectures.

[Documentation on docs.rs](https://docs.rs/crate/errno-codes)

## Why?

`errno` is a standard way for handling errors in many standard libraries of
many operating systems. Unfortunately, very few `errno` constant identifiers
are standardized, and a lot of `errno` values with the same meaning use
different numeric constants across different OSes.

On Linux, some CPU architectures may define their own numeric constants (see
`$KERNEL_SRC/arch/$ARCH/include/uapi/asm/errno.h`), different from common
values used by the most of other architectures (see
`$KERNEL_SRC/include/uapi/asm-generic/errno*.h`). It looks like this was done
for source-level compatibility with proprietary Unixes for these CPU
architectures.

Despite compatibility complications, raw `errno` codes are sometimes used in
network protocols. This library simplifies working with such protocols.

## Known errno codes

Source code with constants and mappings generate automatically based on header
files from following OSes:

* Linux, all architectures known by kernel 4.16
* Windows (MinGW)

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
errno-codes = "0.1"
```

See `examples/` directory and crate `errno-codes-tool` for usage examples.
