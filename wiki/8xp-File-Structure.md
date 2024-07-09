## Overview

Each 8xp file contains:

- 55 byte header
- 19 byte metadata
- Variable length body
- 2 byte checksum

## Header

### Signature

- Content: 8 byte signature that is always `**TI83F*`.
- Encoding: ASCII

### Signature Extra

- Content: 2 bytes always with these values: `0x1A`, `0x0A`.

### Mystery Byte

- Content: 1 byte always with the value `0x00`.

### Comment

- Content: 42 bytes of comment, usually: `Created by [Tools Name] [Tools Version]`. Padded with 0x00.
- Encoding: ASCII

### Metadata and Body Length

The number of bytes in the metadata and body, excluding checksum.

- Content: 2 bytes
- Encoding: little-endian 16-bit integer.

## Metadata

### Flag

- Content: 1 byte flag, usually `0x0D`.

### Unknown Byte

- Content: 1 byte always with the value `0x00`.

### Body and Checksum Length

The number of bytes in the body and checksum.

- Content: 2 bytes
- Encoding: little-endian 16-bit integer.

### File Type

- Content: 1 byte
- Encoding:
  - 0x05: Normal Program
  - 0x06: Locked Program
  - 0x17: Groups

### Name

- Content: 8 bytes of program name. Padded with 0x00.
- Encoding: ASCII

### Version

- Content: 1 byte, usually `0x00`.

### Is Archived

- Content: 1 byte
- Economics:
  - 0x00: Not archived
  - 0x80: Archived

### Body and Checksum Length

The number of bytes in the body and checksum. This is repeated again, for some reason.

- Content: 2 bytes
- Encoding: little-endian 16-bit integer.

### Body Length

The number of bytes in the body.

- Content: 2 bytes
- Encoding: little-endian 16-bit integer.

## Body

The body of the 8xp file is the actual program data. The length of the body is specified in the metadata.

- Content: Variable length
- Encoding: [Check out the 8xp token list](tokens.md)

## Checksum

- Content: 2 bytes
- Encoding: little-endian 16-bit integer.
