use crate::commands::convert::print_bytes;
use crate::tokens::Tokens;
use std::path::PathBuf;

pub fn convert_8xp_to_txt(
    input_path: PathBuf,
    bytes: bool,
    display: bool,
    log_messages: bool,
) -> Vec<String> {
    let file = match std::fs::read(&input_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to read file: {}", err);
            std::process::exit(0);
        }
    };

    if bytes {
        println!("Bytes:");
        print_bytes(&file);
        println!();
    }

    let tokens = Tokens::new();

    if display {
        println!("Tokens:");
        println!();
    }
    let mut output_file = Vec::new();
    let mut skip_next = false;

    // 55 byte header
    let (header, file) = file.split_at(55);
    println!("header: {}", header.len());
    // 19 byte meta data
    let (met_data, file) = file.split_at(19);
    println!("meta: {}", met_data.len());

    // 2 byte checksum at end
    let (file, end_bytes) = file.split_at(file.len() - 2);
    println!("end bytes: {}", end_bytes.len());

    for (i, byte) in file.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        let token = tokens.single_byte_tokens.get(byte);

        match token {
            Some(token) => match token.as_str() {
                "[error: unused code point]" => {
                    if log_messages {
                        println!("Unused code point");
                    }
                }
                "[error: unknown 2-byte code]" => {
                    let next_byte = file[i + 1];
                    skip_next = true;
                    let token = match byte {
                        0x5C => tokens.matrix_tokens.get(&next_byte),
                        0x5D => tokens.list_tokens.get(&next_byte),
                        0x5E => tokens.equation_tokens.get(&next_byte),
                        0x60 => tokens.picture_tokens.get(&next_byte),
                        0x61 => tokens.gdb_tokens.get(&next_byte),
                        0xAA => tokens.string_tokens.get(&next_byte),
                        0x62 => tokens.statistic_variable_tokens.get(&next_byte),
                        0x63 => tokens.window_and_finance_tokens.get(&next_byte),
                        0x7E => tokens.graph_format_tokens.get(&next_byte),
                        0xBB => tokens.misc_tokens.get(&next_byte),
                        0xEF => tokens.ti_84_tokens.get(&next_byte),
                        _ => continue,
                    };

                    match token {
                        Some(token) => {
                            if display {
                                print!("{}", token)
                            }

                            output_file.push(token.to_string())
                        }
                        None => {
                            if log_messages {
                                println!(
                                    "Failed to translate 2-byte code: {:02X?} {:02X?}",
                                    byte, next_byte
                                )
                            }
                        }
                    }
                }
                _ => {
                    if display {
                        if token.to_string() == "\n" {
                            println!();
                        } else {
                            print!("{}", token);
                        }
                    }

                    output_file.push(token.to_string());
                }
            },
            None => {
                if log_messages {
                    println!("Unknown code point: {:02X?}", byte);
                }
            }
        }
    }
    if display {
        println!();
    }

    output_file
}
