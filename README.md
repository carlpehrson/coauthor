# Coauthor
Automate the process of adding co-authors to git commits

# Getting started
Currently the only way to get Coauthor to your environment is get it through
cargo (the package manager bundled with rust) directly from source (GitHub).

Check your rust version by running `rustc --version` and ensure you have
version `1.35` or higher. If you don't have rust installed, you can install it
via homebrew (`brew install rust`).

With a valid version of rust installed you can install coauthor to your
environment by executing this in your terminal (current path doesn't matter):

```sh
cargo install --root ~/.cargo --git https://github.com/carlpehrson/coauthor coauthor
```

# Usage
_Currently not documented, please see `coauthor help` for documentation._

# Development
Assuming you have cargo (the package manager bundled with rust) installed
you should only have to:

```sh
# Run the test suite
cargo test

# Run the tool
cargo run
```
