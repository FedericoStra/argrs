# argrs

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
