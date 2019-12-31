# Graphic Info
![](https://github.com/x-gate/graphic-info/workflows/Rust/badge.svg)

The GraphicInfo* parser.

## Usage

```
graphic-info 0.1.0
Vincent Chi <song374561@chivincent.net>

USAGE:
    graphic-info <GraphicInfo.bin> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <GraphicInfo.bin>    The path of GraphicInfo.bin

SUBCOMMANDS:
    dump    Dump all of graphic info into sqlite file.
    help    Prints this message or the help of the given subcommand(s)
    info    Show the information of graphic info file.
```

### Try it on! 

- Show the information of graphic info file:
    - `cargo run ./resources/GraphicInfo.test.bin`
    - `cargo run ./resources/GraphicInfo.test.bin info`
- Dump graphic info file into sqlite:
    - `cargo run ./resources/GraphicInfo.test.bin dump`
    - `cargo run ./resources/GraphicInfo.test.bin dump -o GraphicInfo.sqlite`

## LICENSE

This software in under [GPLv2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html) license.