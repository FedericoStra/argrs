# argrs

[![crates.io](https://img.shields.io/crates/v/argrs?logo=rust)](https://crates.io/crates/argrs)
[![docs.rs](https://img.shields.io/docsrs/argrs?logo=docsdotrs)](https://docs.rs/argrs)
[![GitHub](https://img.shields.io/static/v1?label=github&message=FedericoStra/argrs&color=brightgreen&logo=github)](https://github.com/FedericoStra/argrs)
[![Dependencies status](https://deps.rs/repo/github/FedericoStra/argrs/status.svg)](https://deps.rs/repo/github/FedericoStra/argrs)
[![BSD-3-Clause license](https://img.shields.io/crates/l/argrs)](https://choosealicense.com/licenses/bsd-3-clause/)

A simple program to list the arguments passed on the command line.

Ported to Rust from [args](https://github.com/FedericoStra/args).

## Demo

```
$ argrs helps you debug command line arguments
0: `argrs`
1: `helps`
2: `you`
3: `debug`
4: `command`
5: `line`
6: `arguments`
```

## Usage

Simply call `argrs` with whatever command line arguments and it'll print them.
This can be useful to verify commands generated with [`find`]/[`fd`] before executing them,
for example

```
$ fd -tf -X argrs wc -l
0: `argrs`
1: `wc`
2: `-l`
3: `./Cargo.lock`
4: `./LICENSE`
5: `./README.md`
6: `./src/main.rs`
7: `./Cargo.toml`
```

[`find`]: https://www.gnu.org/software/findutils/
[`fd`]: https://github.com/sharkdp/fd
