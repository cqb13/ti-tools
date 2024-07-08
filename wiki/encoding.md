````markdown
# Encoding a Decoded Program

How to convert the body of a decoded program into a bytes.

## Parameters

- `decoded_program: &String`: A reference to the decoded program as a `String`.
- `tokens: &Map`: A reference to a map containing the token values. This map is used to find the byte sequences for each token.
- `perform_normalize: bool`: A boolean flag indicating whether to normalize the input text before encoding.
- `display_mode: DisplayMode`: An enum specifying the display mode for the text. The available modes are `Pretty`, `Accessible`, and `TiAscii`.
- `encode_mode: EncodeMode`: An enum specifying the encoding mode. The available modes are `Min`, `Max`, and `Smart`.

## Return Value

The function returns a `Vec<u8>` containing the encoded bytestream.

## Encoding Process

1. **Initialization**:

   - An empty `Vec<u8>` (`encoded_program`) is created to hold the final encoded byte sequence.

2. **Normalization**:

   - If `perform_normalize` is `true`, the function normalizes the input `decoded_program` using the `normalize` function.
   - If `perform_normalize` is `false`, the input `decoded_program` is used as-is.

3. **Line Processing**:

   - The function iterates through each line of the normalized (or original) `decoded_program`.
   - A boolean flag (`in_string`) is used to keep track of whether the current position is within a string.

4. **Token Matching**:

   - For each line, the function processes the text to match tokens based on the specified `encode_mode`:
     - `EncodeMode::Max`: Finds the longest matching token.
     - `EncodeMode::Min`: Finds the shortest matching token.
     - `EncodeMode::Smart`: Finds the shortest matching token if `in_string` is `true`, otherwise finds the longest matching token.
   - If a token is found:
     - The token's byte sequence is added to `encoded_program`.
     - The token's value is removed from the line being processed.
     - If the token is a double-quote (`"`), the `in_string` flag is toggled.
   - If no token is found, an error message is printed, and the program exits.

5. **New Line Handling**:
   - After processing each line, a new line byte (`0x3F`) is added to `encoded_program`.
   - The last new line byte is removed before returning the encoded program.

## Normalization

The `normalize` function replaces certain Unicode characters with their equivalent representations:

```rust
fn normalize(string: &str) -> String {
    let string = string
        .replace('\u{0398}', "θ")
        .replace('\u{03F4}', "θ")
        .replace('\u{1DBF}', "θ");
    string
}
```
````

## Key Conversion

The `convert_key_to_bytes` function converts a token key (in string format: $XX or $XX$XX) to a vector of bytes:

```rust
fn convert_key_to_bytes(key: &str) -> Vec<u8> {
    let key = key.replace(" en", "");
    let keys = key.split("$").collect::<Vec<&str>>();
    let keys = keys[1..].to_vec();
    let mut bytes = Vec::new();

    for key in keys {
        let byte = u8::from_str_radix(key, 16).unwrap();
        bytes.push(byte);
    }

    bytes
}
```

## Error Handling

The function handles errors in the following scenarios:

- If no matching token is found for a part of the line, an error message is printed, and the program exits.
