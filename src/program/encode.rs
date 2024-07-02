use crate::tokens::Trie;

use super::decode;
/**
Encodes a string of tokens represented as text into a byte stream and its minimum supported OS version

   Tokenization is performed using one of three procedures, dictated by ``mode``:
       - ``max``: Always munch maximally, i.e. consume the most input possible to produce a token
       - ``smart``: Munch maximally or minimally depending on context
       - ``string``: Always munch minimally (equivalent to ``smart`` string context)

   The ``smart`` tokenization mode uses the following contexts, munching maximally otherwise:
       - Strings: munch minimally, except when interpolating using ``Send(``
       - Program names: munch minimally up to 8 tokens
       - List names: munch minimally up to 5 tokens

   For reference, here are the tokenization modes utilized by popular IDEs and other software:
       - SourceCoder: ``max``
       - TokenIDE: ``max``
       - TI Connect CE: ``smart``
       - TI-Planet Project Builder: ``smart``
       - tivars_lib_cpp: ``smart``

   All tokenization modes respect token glyphs for substituting Unicode symbols.

   :param string: The text string to encode
   :param trie: The `TokenTrie` object to use for tokenization
   :param mode: The tokenization mode to use (defaults to ``smart``)
   :param normalize: Whether to apply NFC normalization to the input before encoding (defaults to ``true``)
   :return: A tuple of a stream of token bytes and a minimum `OsVersion`
*/

pub enum TokenizationMode {
    Max,
    Smart,
    String,
}

pub fn encode(
    decoded_program: String,
    trie: &Trie,
    mode: TokenizationMode,
    perform_normalize: bool,
) -> Vec<u8> {
    let mut encoded_program = Vec::new();

    let decoded_program = if perform_normalize {
        normalize(&decoded_program)
    } else {
        decoded_program
    };

    let mut index = 0;

    while decoded_program != "" {

    }

    encoded_program
}

// Applies NFC normalization to a given string to ensure recognition of certain Unicode characters used as token names
fn normalize(string: &str) -> String {
    let string = string
        .replace('\u{0398}', "θ")
        .replace('\u{03F4}', "θ")
        .replace('\u{1DBF}', "θ");
    string
}
