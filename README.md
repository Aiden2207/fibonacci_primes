# Fibonacci Prime Tester

This is the controller for the [Generate Fibonacci Primes Quickly](https://codegolf.stackexchange.com/questions/248746/generate-fibonacci-primes-quickly) challenge.
 The challenge is now live! Feel free to enter.
## Installation

You need an up to date version of rust to install this code. To install just the tool run `cargo install --git https://github.com/Aiden2207/fibonacci_primes.git`. If you want to run all the competitors, I recommend installing via `git clone https://github.com/Aiden2207/fibonacci_primes.git` followed by `cargo install --path path/to/cloned/repo`.

## Usage

To use, simply run `fibonacci_primes [path/to/test/directory]`. The default directory is the current directory, and the [`competitors`](competitors) folder has the current competitors. In that folder, configuration is provided by a `config.json` file. The specification of the `config.json` is as follows:

- The top level object should have a single field with the name `competitors` with an array of `Competitor`s as its field
- `Competitor`: an object with three fields; `name`, as string corresponding to a directory at the same level as the `config.json`, which is will be set as the working directory; `setup`, an optional array of `Command`s, which run setup for the timed command; and `run`, a single `Command` that is the actual timed command.
- `Command`: an object with two fields; `command`, the actual executable being run; and `args`, an optional string array of arguments being passed to the command.


## Security

There is none. While the controller should be robust against anything that a competitor might accidentally do it does not attempt to provide any other form of security protections. It is up to the end user to ensure any code they run is either safe or properly sandboxed.

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
