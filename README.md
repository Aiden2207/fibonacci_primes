# Fibonacci Prime Tester

This is the controller for the [Generate Fibonacci Primes Quickly](https://codegolf.meta.stackexchange.com/a/24788/97691) challenge.
 Currently, it's still in the sandbox, but will hopefully be up sometime soonish.

## Installation

You need an up to date version of rust to install this code. To install just the tool run `cargo install --git https://github.com/Aiden2207/fibonacci_primes.git`. If you want to run all the competitors, I recommend installing via `git clone https://github.com/Aiden2207/fibonacci_primes.git` followed by `cargo install --path path/to/cloned/repo`.

## Usage

To use, simply run `fibonacci_primes [path/to/test/directory]`. The default directory is the current directory, and the [`competitors`](competitors) folder has the current competitors. In that folder, configuration is provided by a `config.json` file. I'm too lazy to write the full specification right now, but I'll do so before I launch the challenge.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
