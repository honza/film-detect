Film Detect
===========

This is a command-line program that takes a Fujifilm jpeg and outputs film
simulation information.

## Usage

``` sh
$ film-detect photo.jpg
Film simulation: Classic Chrome
Grain: Large Weak
Color Chrome: Strong
Color Chrome FX Blue: Weak
White Balance: Auto
White Balance Fine Tune: R:2 B:-5
Dynamic Range: Auto
Shadow: 0
Highlight: 0
Color: +3
Sharpness: +1
Noise Reduction: 0
Clarity: 3
```

``` sh
Usage: film-detect [OPTIONS] <FILE>

Arguments:
  <FILE>  Filename to operate on

Options:
      --json               Output JSON
  -h, --help               Print help
  -V, --version            Print version
```

## Installation

For now, only git:

``` sh
$ git clone https://github.com/honza/film-detect
$ cd film-detect
$ cargo install --path .
```

## License

GPLv3 or later
