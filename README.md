# TI Tools

TI Tools is a CLI tool designed for converting 8xp files (used by TI-83 and TI-84 calculators) to text files and vice versa. It also supports various other features for working with 8xp files.

## TODO

- [x] make rename command use Program
  - [x] add tests for rename command
- [] add option to encode command to specify much mode
- [] finish implementing encode command
- [] actually use the checksum when decoding (currently using length from metadata)
- [] add better error handling
- [] improve speed
- [] organize tests

## Acknowledgments

This project would not be possible without the help of the following:

- [TI Toolkit Token Sheet](https://github.com/TI-Toolkit/tokens)
- [TI Basic Developer Wiki](http://tibasicdev.wikidot.com/tokens)
- [TI-83+ Link Protocol Guide v1.1](https://merthsoft.com/linkguide/ti83+/fformat.html)

## Installation

### Build from Source

Clone the repository and build the project using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```sh
git clone https://github.com/cqb13/ti-tools.git
cd ti-tools
cargo build --release
# The binary will be located at target/release/ti-tools
```

To add the binary to your PATH, run:

```sh
cargo install --path .
```

### Pre-built Binaries

Pre-built binaries are available for Windows, macOS, and Linux on the [releases page](https://github.com/cqb13/ti-tools/releases).

## Usage

```sh
    ti-tools [COMMAND] [OPTIONS]
```

## Commands

```
    help
        Prints help information
                                  <COMMAND>    A command to help with
    version
        Prints version information
    decode
        Converts 8xp to txt
                                  <INPUT>      The input path to an 8xp file
        -o           --output     <OUTPUT>     The output path to a txt file
        -d           --display-mode <DISPLAY_MODE> The characters to translate the tokens to [pretty, accessible, ti] | Default: accessible
        -m           --model      <MODEL>      The model of calculator (use models command to see the supported models) | Default: latest
        -c           --content    <>           Display the content of the input file
        -p           --preview    <>           Preview the output file in the terminal
    encode
        Converts txt to 8xp
                                  <INPUT>      The input path to an 8xp file
        -o           --output     <OUTPUT>     The output path to a 8xp file
        -m           --model      <MODEL>      The model of calculator (use models command to see the supported models) | Default: latest
        -c           --content    <>           Display the content of the input file
        -p           --preview    <>           Preview the output file in the terminal
    rename
        Renames the program name in a 8xp file
                                  <INPUT>      The input path to an 8xp file
        -n           --name       <NAME>       New program number (8 or less uppercase alphabetic characters)
        -f           --new-file   <>           Create a new file with the same name as the program
        -d           --delete-old <>           Delete the old file
    models
        Prints the supported TI calculator models
```

## Examples

### Decode

```sh
ti-tools decode ./src/tests/programs/TOCCATA.8xp -p -c -o ./TOCCATA.txt
```

### Encode

```sh
ti-tools encode ./TOCCATA.txt -p -c -o ./TOCCATA.8xp
```

## Contributing

Contributions are welcome! Feel free to fork this repository and submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
