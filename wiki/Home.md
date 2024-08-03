TI Tools is a CLI tool designed for converting 8xp files (used by TI-83 and TI-84 calculators) to text files and vice versa. It also supports various other features for working with 8xp files.

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
    help - Prints help information
        command       <COMMAND>                     (optional) The command you want help with

    version - Prints version information

    decode - Converts 8xp/82p/83p to txt
        input         <INPUT>                       (required) The input path to an 8xp/82p/83p file
        -o            --output       <OUTPUT>                  The output path to a txt file
        -d            --display-mode <DISPLAY_MODE>            The characters to translate the tokens to [pretty, accessible, ti] | Default: accessible
        -c            --content                                Display the content of the input file
        -p            --preview                                Display the decoded output
        -m            --mass                                   Changes input required from file to directory for mass file decoding

    encode - Converts txt to 8xp
        input         <INPUT>                       (required) The input path to an txt file
        -o            --output       <OUTPUT>                  The output path to an 8xp/82p/83p file
        -e            --encode-mode  <ENCODE_MODE>             The mode used to parse tokens [min, max, smart] | Default: smart
        -c            --content                                Display the content of the input file
        -p            --preview                                Display the decoded output
        -m            --mass                                   Changes input required from file to directory for mass file encoding

    rename - Renames the program name in a 8xp/82p/83p file
        input         <INPUT>                       (required) The input path to an 8xp/82p/83p file
        -n            --name         <NAME>                    The new program name (8 or less alphabetic characters)
        -f            --new-file     <NEW_FILE>                Save the renamed program to a new file
        -d            --delete-old                             Delete the old file

    comment - Write a custom comment to an 8xp/82p/83p file
        input         <INPUT>                       (required) The input path to an 8xp/82p/83p file
        -c            --comment      <COMMENT>                 The new program comment (42 or less characters)
        -f            --new-file     <NEW_FILE>                Save the program with the updated comment to a new file
        -d            --delete-old                             Delete the old file

    lock - Lock an 8xp/82p/83p file
        input         <INPUT>                       (required) The input path to an 8xp/82p/83p file
        -f            --new-file     <NEW_FILE>                Save the locked program to a new file
        -d            --delete-old                             Delete the old file

    unlock - unlock an 8xp/82p/83p file
        input         <INPUT>                       (required) The input path to an 8xp/82p/83p file
        -f            --new-file     <NEW_FILE>                Save the unlocked program to a new file
        -d            --delete-old                             Delete the old file

    archive - Set the program to be sent to Archive
        input         <INPUT>                       (required) The input path to an 8xp/82p/83p file
        -f            --new-file     <NEW_FILE>                Save the archived program to a new file
        -d            --delete-old                             Delete the old file

    unarchive - Set the program to be sent to RAM
        input         <INPUT>                       (required) The input path to an 8xp/82p/83p file
        -f            --new-file     <NEW_FILE>                Save the un-archived program to a new file
        -d            --delete-old                             Delete the old file

    details - Displays information about an 8xp/82p/83p file
        input         <INPUT>                       (required) The input path to an 8xp/82p/83p file

    models - Prints the supported TI calculator models
```

## Examples

### Decode

```sh
ti-tools decode ./tests/programs/TOCCATA.8xp -p -c -o ./TOCCATA.txt
```

```sh
ti-tools decode ./tests/programs -o ./programs --mass
```

### Encode

```sh
ti-tools encode ./TOCCATA.txt -p -c -o ./TOCCATA.8xp
```

```sh
ti-tools encode ./programs -o ./programs --mass
```

## Troubleshooting

### Mismatch between decoded and encoded programs

- If you decoded the program into pretty tokens, encoding may not work correctly as there are duplicate pretty tokens.

  - To fix this, decode the program into accessible tokens and then encode it.

- If you decoded a program from a different source, and the encoded version does not match the original, try using a different encoding mode.
  - The `smart` encoding mode is used by default by this program but not all programs do.
  - List of other programs and their encoding modes:
    - SourceCoder: `max`
    - TokenIDE: `max`
    - TI Connect CE: `smart`
    - TI Planet Project Builder: `smart`
    - ti_vars_lib_cpp: `smart`
    - ti_vars_lib_py: `smart`
  - If you are still having issues, please open an issue with the decoded and encoded programs.

### Error loading program

- If there is any error loading the program, ensure that you are using a valid file type (`.8xp` or `.txt`).
- If there is an error encoding the program, ensure that the file matches the formatting detailed in [txt File Structure](txt-file-structure).
- If there is an error decoding the program, ensure that you properly specified the model of the calculator.

- If you are still having issues, please open an issue with the program that is causing the error.

### Syntax error when running on calculator

- If you are getting a syntax error when running the program on the calculator, ensure that you used the correct model when encoding the program.

- If you are still having issues, please open an issue with the program that is causing the error.

## Contributing

Contributions are welcome! Feel free to fork this repository and submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
