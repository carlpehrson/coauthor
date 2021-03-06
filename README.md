# Coauthor
A tool to automatically add co-authors to git commit messages while doing
pair programming or mob programming.

## Getting started
Currently the only way to get Coauthor to your environment is get it through
cargo (the package manager bundled with rust) directly from source (GitHub).

Check your rust version by running `rustc --version` and ensure you have
version `1.35` or higher. If you don't have rust installed, you can install it
via homebrew (`brew install rust`).

With a valid version of rust installed you can install coauthor to your
environment by executing this in your terminal (current path doesn't matter):

```sh
cargo install --root ~/.cargo --git https://github.com/carlpehrson/coauthor --tag v0.2.0 coauthor
```

Also, if you haven't already you have to export cargos bin folder into your path
to make `coauthor` available throught your environment (typically put in your `~/.profile`):

```
export PATH="$PATH:$HOME/.cargo/bin"
```

## Usage
Coauthor comes with a few commands at your disposal which can be executed with
the `coauthor` prefix, e.g. `coauthor add`. The commands can be divided
in two groups:

*Organizing your list of available coauthors*
- `add`: Starts a prompt to add an coauthor.
- `list`: Lists all stored coauthors.
- `remove [username]`: Removes a coauthor from the local machine.

*Controlling active coauthors*
- `set [username [username ..]]`: Updates the git template with predefined coauthors.
- `current`: Show a list of active coauthors.
- `clear`: Removes all coauthors from the commit template.

You can always type `coauthor help` to see the list of available commands.

## Development
Assuming you have cargo (the package manager bundled with rust) installed
you should only have to:

```sh
# Run the test suite
cargo test

# Run the tool
cargo run
```
