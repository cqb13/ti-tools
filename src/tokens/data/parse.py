# data from here: https://github.com/TI-Toolkit/tokens
# used for creating the token sheets
# TODO: Make it write the files instead of printing them
import json


def open_json(file):
    with open(file) as f:
        data = json.load(f)

    return data


def display_two_byte(data):
    display_two_byte_token_set(data, "Matrix", "$5C")
    display_two_byte_token_set(data, "List", "$5D")
    display_two_byte_token_set(data, "Equation", "$5E")
    display_two_byte_token_set(data, "Picture", "$60")
    display_two_byte_token_set(data, "GDB", "$61")
    display_two_byte_token_set(data, "String", "$AA")
    display_two_byte_token_set(data, "Stat Var", "$62")
    display_two_byte_token_set(data, "Window & Finance", "$63")
    display_two_byte_token_set(data, "Graph", "$7E")
    display_two_byte_token_set(data, "Misc", "$BB")
    display_two_byte_token_set(data, "TI-84", "$EF")


def display_two_byte_token_set(token_list, name, byte):
    tokens = token_list[byte]
    print(f"{name} Tokens")
    print()
    # using len to get the latest version of the token
    for token in tokens:
        data = tokens[token][len(tokens[token]) - 1]["langs"]["en"]
        clean_token = token.replace("$", "0x")
        print(f'({clean_token}, "{data["accessible"]}".to_string()),')
    print()
    found = False
    for token in tokens:
        data = tokens[token][len(tokens[token]) - 1]["langs"]["en"]
        if data["accessible"] != data["display"]:
            found = True
            print(
                f'("{data["accessible"]}".to_string(), "{data["display"]}".to_string()),'
            )
    if found:
        print()


def display_one_byte(data):
    print("Single Byte Tokens")
    print()
    for token in data:
        clean_token = token.replace("$", "0x")
        if (
            token == "$5C"
            or token == "$5D"
            or token == "$5E"
            or token == "$60"
            or token == "$61"
            or token == "$AA"
            or token == "$62"
            or token == "$63"
            or token == "$7E"
            or token == "$BB"
            or token == "$EF"
        ):
            print(f'({clean_token}, "[error: unknown 2-byte code]".to_string()),')
        else:
            try:
                data_langs = data[token][-1]["langs"]["en"]
                clean_token = token.replace("$", "0x")
                print(f'({clean_token}, "{data_langs["accessible"]}".to_string()),')
            except KeyError as e:
                print(f"KeyError for token {token}: {e}")
    print()
    found = False
    for token in data:
        if (
            token == "$5C"
            or token == "$5D"
            or token == "$5E"
            or token == "$60"
            or token == "$61"
            or token == "$AA"
            or token == "$62"
            or token == "$63"
            or token == "$7E"
            or token == "$BB"
            or token == "$EF"
        ):
            continue
        else:
            try:
                data_langs = data[token][-1]["langs"]["en"]
                if data_langs["accessible"] != data_langs["display"]:
                    found = True
                    print(
                        f'("{data_langs["accessible"]}".to_string(), "{data_langs["display"]}".to_string()),'
                    )
            except KeyError as e:
                print(f"KeyError for token {token}: {e}")

    if found:
        print()


def main():
    data = open_json("data.json")
    display_one_byte(data)
    display_two_byte(data)


if __name__ == "__main__":
    main()
