use crate::commands::convert::print_bytes;
use crate::tokens::Tokens;
use std::path::PathBuf;

pub struct Program {
    pub header: Header,
    pub meta_data: MetaData,
    pub body: Vec<String>,
    pub checksum: [u8; 2],
}

impl Program {
    pub fn new(file: Vec<u8>, log_messages: bool) -> Program {
        let tokens = Tokens::new();

        let mut skip_next = false;

        // 55 byte header
        let (header_bytes, file) = file.split_at(55);
        let header = Header::new(header_bytes);

        // 19 byte meta data
        let (meta_data_bytes, file) = file.split_at(19);
        let meta_data = MetaData::new(meta_data_bytes);

        // 2 byte checksum at end
        let (file, checksum) = file.split_at(file.len() - 2);

        let mut body = Vec::new();

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
                            Some(token) => body.push(token.to_string()),
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
                        body.push(token.to_string());
                    }
                },
                None => {
                    if log_messages {
                        println!("Unknown code point: {:02X?}", byte);
                    }
                }
            }
        }

        Program {
            header,
            meta_data,
            body,
            checksum: [checksum[0], checksum[1]],
        }
    }

    pub fn display(&self) {
        println!("Program:");
        println!("----------------------------");
        println!("Header:");
        self.header.display();
        println!();
        println!("Meta Data:");
        self.meta_data.display();
        println!();
        println!("Checksum:");
        println!("{}", u16::from_le_bytes(self.checksum));
        println!();

        for token in &self.body {
            if token.to_string() == "\n".to_string() {
                println!()
            } else {
                print!("{}", token)
            }
        }
    }

    pub fn to_output(&self) -> Vec<String> {
        let mut output_array: Vec<String> = Vec::new();

        output_array.push("Program:".to_string());
        output_array.extend(vec!["\n".to_string()]);
        output_array.extend(vec!["----------------------------".to_string()]);
        output_array.extend(vec!["\n".to_string()]);

        output_array.push("Header:".to_string());
        output_array.extend(vec!["\n".to_string()]);
        for line in self.header.to_array() {
            output_array.push(line);
            output_array.extend(vec!["\n".to_string()]);
        }
        output_array.extend(vec!["\n".to_string()]);

        output_array.push("Meta Data:".to_string());
        output_array.extend(vec!["\n".to_string()]);
        for line in self.meta_data.to_array() {
            output_array.push(line);
            output_array.extend(vec!["\n".to_string()]);
        }
        output_array.extend(vec!["\n".to_string()]);

        output_array.push(format!("Checksum: {}", u16::from_le_bytes(self.checksum)));
        output_array.extend(vec!["\n".to_string()]);

        output_array.push("Program Start:".to_string());
        output_array.extend(vec!["\n".to_string()]);
        output_array.extend(vec!["----------------------------".to_string()]);
        output_array.extend(vec!["\n".to_string()]);

        output_array.extend(self.body.clone());

        output_array
    }
}

pub struct Header {
    pub signature: String,             // signature, always "**TI83F*" in ASCII (8 bytes)
    pub signature_part_2: [u8; 2],     // signature part 2, two bytes, always with these values: 0x1A, 0x0A
    pub mystery_byte: u8,              // Either 0x00 or 0x0A?
    pub comment: String,               // 42 bytes, typical compile app version then padding of 0x00 [ex: "Created by TI Connect CE 5.3.0.384"]
    pub meta_and_body_length: [u8; 2], //Number of bytes from here on, excluding the checksum (little endian)
}

impl Header {
    pub fn new(header_bytes: &[u8]) -> Header {
        let signature = header_bytes[0..8]
            .iter()
            .map(|byte| *byte as char)
            .collect::<String>();

        let signature_part_2 = [header_bytes[8], header_bytes[9]];

        let mystery_byte = header_bytes[10];

        let comment = header_bytes[11..53]
            .iter()
            .map(|byte| *byte as char)
            .collect::<String>();

        let meta_and_body_length = [header_bytes[53], header_bytes[54]];

        Header {
            signature,
            signature_part_2,
            mystery_byte,
            comment,
            meta_and_body_length,
        }
    }

    pub fn to_array(&self) -> Vec<String> {
        vec![
            format!("Signature: {}", self.signature),
            format!(
                "Signature Part 2: {:02X?} {:02X?}",
                self.signature_part_2[0], self.signature_part_2[1]
            ),
            format!("Mystery Byte: {:02X?}", self.mystery_byte),
            format!("Comment: {}", self.comment),
            format!(
                "Meta and Body Length: {:02X?} {:02X?} ({})",
                self.meta_and_body_length[0],
                self.meta_and_body_length[1],
                u16::from_le_bytes(self.meta_and_body_length)
            ),
        ]
    }

    pub fn display(&self) {
        for line in self.to_array() {
            println!("{}", line);
        }
    }
}

pub struct MetaData {
    pub flag: u8,                                    // either 0x0B or 0x0D (11 or 13)
    pub unknown: u8,                                 // in most cases 0x00
    pub body_and_checksum_length: [u8; 2],           // Length of the body/code section and the 2 byte checksum, in bytes
    pub file_type: FileType,                         // Type of file
    pub name: String,                                // Maximum 8 chars. Unused characters are filled with 0x00 ASCII
    pub version: u8,                                 // Version of the file
    pub archived: Archived,                          // Archived or not
    pub body_and_checksum_length_duplicate: [u8; 2], // Same thing as before not sure why???
    pub body_length: [u8; 2],                        // Length of the body/code section, in bytes not including the checksum (little endian)
}

impl MetaData {
    pub fn new(meta_data_bytes: &[u8]) -> MetaData {
        let flag = meta_data_bytes[0];
        let unknown = meta_data_bytes[1];
        let body_and_checksum_length = [meta_data_bytes[2], meta_data_bytes[3]];
        let file_type = FileType::from_byte(meta_data_bytes[4]);
        let name = meta_data_bytes[5..13]
            .iter()
            .map(|byte| *byte as char)
            .collect::<String>();
        let version = meta_data_bytes[13];
        let archived = Archived::from_byte(meta_data_bytes[14]);
        let body_and_checksum_length_duplicate = [meta_data_bytes[15], meta_data_bytes[16]];
        let body_length = [meta_data_bytes[17], meta_data_bytes[18]];

        MetaData {
            flag,
            unknown,
            body_and_checksum_length,
            file_type,
            name,
            version,
            archived,
            body_and_checksum_length_duplicate,
            body_length,
        }
    }

    pub fn to_array(&self) -> Vec<String> {
        vec![
            format!("Flag: {:02X?}", self.flag),
            format!("Unknown: {:02X?}", self.unknown),
            format!(
                "Body and Checksum Length: {:02X?} {:02X?} ({})",
                self.body_and_checksum_length[0],
                self.body_and_checksum_length[1],
                u16::from_le_bytes(self.body_and_checksum_length)
            ),
            format!("File Type: {}", self.file_type.to_string()),
            format!("Name: {}", self.name),
            format!("Version: {:02X?}", self.version),
            format!("Archived: {}", self.archived.to_string()),
            format!(
                "Body and Checksum Length Duplicate: {:02X?} {:02X?} ({})",
                self.body_and_checksum_length_duplicate[0],
                self.body_and_checksum_length_duplicate[1],
                u16::from_le_bytes(self.body_and_checksum_length_duplicate)
            ),
            format!(
                "Body Length: {:02X?} {:02X?} ({})",
                self.body_length[0],
                self.body_length[1],
                u16::from_le_bytes(self.body_length)
            ),
        ]
    }

    pub fn display(&self) {
        for line in self.to_array() {
            println!("{}", line);
        }
    }
}

pub enum FileType {
    Program,
    EditLockedProgram,
    Group,
    FlashApplication,
}

impl FileType {
    pub fn from_byte(byte: u8) -> FileType {
        match byte {
            0x05 => FileType::Program,
            0x06 => FileType::EditLockedProgram,
            0x17 => FileType::Group,
            0x24 => FileType::FlashApplication,
            _ => panic!("Unknown file type: {:02X?}", byte),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FileType::Program => "Program".to_string(),
            FileType::EditLockedProgram => "Edit Locked Program".to_string(),
            FileType::Group => "Group".to_string(),
            FileType::FlashApplication => "Flash Application".to_string(),
        }
    }
}

pub enum Archived {
    Unarchived,
    Archived,
}

impl Archived {
    pub fn from_byte(byte: u8) -> Archived {
        match byte {
            0x00 => Archived::Unarchived,
            0x80 => Archived::Archived,
            _ => panic!("Unknown archived byte: {:02X?}", byte),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Archived::Unarchived => "Unarchived".to_string(),
            Archived::Archived => "Archived".to_string(),
        }
    }
}

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
        println!("\n");
    }

    let program = Program::new(file, log_messages);

    if display {
        program.display();
        println!();
    }

    program.to_output()
}
