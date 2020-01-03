# Graphic Info
![](https://github.com/x-gate/graphic-info/workflows/Rust/badge.svg)

The GraphicInfo* parser.

## Usage

```
graphic-info 0.2.0
Vincent Chi <song374561@chivincent.net>

USAGE:
    graphic-info [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build    Build GraphicInfo.bin from sqlite.
    dump     Dump all of graphic info into sqlite file.
    help     Prints this message or the help of the given subcommand(s)
    info     Show the information of graphic info file.
```

### Try it on! 

- Show the information of graphic info file:
    - `cargo run info ./resources/GraphicInfo.test.bin`
- Dump graphic info file into sqlite:
    - `cargo run dump ./resources/GraphicInfo.test.bin`
    - `cargo run dump ./resources/GraphicInfo.test.bin -o GraphicInfo.sqlite`
- Build GraphicInfo.bin from sqlite:
    - `cargo run build -i ./GraphicInfo.sqlite -o ./GraphicInfo.bin`

## LICENSE

This software in under [GPLv2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html) license.