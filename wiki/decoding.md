# Decoding a Bytestream

How to decode bytes from an 8xp file into a human-readable program.

## Parameters

- `bytestream: &[u8]`: A slice of bytes representing the encoded data.
- `map: &Map`: A reference to a map that holds the token values. This map is used to look up the textual representation of each byte sequence.
- `lang: &str`: A string specifying the language in which the text should be decoded.
- `mode: &DisplayMode`: An enum that defines the display mode for the output text. The available modes are `Pretty`, `Accessible`, and `TiAscii`.

## Return Value

The function returns a `Result` type:

- `Ok(String)`: The decoded program as a `String` if the decoding is successful.
- `Err(String)`: An error message as a `String` if the decoding fails.

## Decoding Process

1. **Initialization**:

   - An empty `String` (`decoded_program`) is created to hold the final decoded text.
   - An index (`index`) is initialized to 0 to keep track of the current position in the bytestream.
   - An empty `Vec<u8>` (`current_bytes`) is created to hold the current byte sequence being processed.

2. **Iteration**:

   - The function iterates through the bytestream one byte at a time.
   - Each byte is added to the `current_bytes` vector.

3. **Token Key Construction**:

   - Depending on the length of `current_bytes`, a key is constructed in the format `"$XX"` for single-byte sequences or `"$XX$XX"` for two-byte sequences, where `XX` represents the hexadecimal value of the bytes.

4. **Token Lookup**:

   - The function attempts to find a corresponding token in the map using the constructed key and the specified language.
   - If a token is found:
     - The textual representation of the token is appended to `decoded_program` based on the selected display mode (`Pretty`, `Accessible`, or `TiAscii`).
     - `current_bytes` is cleared to process the next byte sequence.
   - If no token is found, the function continues to the next byte.

5. **Completion**:
   - After processing all bytes in the bytestream, the function checks if `current_bytes` is empty.
   - If `current_bytes` is empty, the function returns the `decoded_program` as `Ok(String)`.
   - If `current_bytes` is not empty, indicating that there was an incomplete byte sequence at the end of the bytestream, the function returns an error with a message indicating the unmatched byte sequence.

## Error Handling

The function handles errors in two main scenarios:

- If an invalid byte length is encountered (i.e., more than two bytes in `current_bytes`), it returns an error with a message indicating the invalid byte length.
- If the function reaches the end of the bytestream and there are still unmatched bytes in `current_bytes`, it returns an error with a message indicating the unmatched byte sequence.
