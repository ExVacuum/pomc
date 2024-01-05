# pomc: pomodoro daemon client

![Crates.io Version](https://img.shields.io/crates/v/pomc)

This program provides a basic CLI client for the [pomd](https://github.com/exvacuum/pomd) pomodoro daemon.

## Installation

### Via crates.io
```sh
cargo install pomc
```

### From Source
```sh
cargo install --path .
```

## Usage
```
Usage: pomc <COMMAND>

Commands:
  start          
  pause          
  stop           
  skip           
  get-iteration  
  get-remaining  
  is-running     
  is-on-break    
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
This program could be used in statusbars to monitor pomd status, as well as used directly from a terminal to control the timer.
