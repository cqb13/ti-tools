import json
from typing import Dict, List, Optional, Union


class OsVersion:
    def __init__(self, model: str, version: str):
        self.model = model
        self.version = version

    def __lt__(self, other):
        return (self.model, self.version) < (other.model, other.version)

    def __eq__(self, other):
        return (self.model, self.version) == (other.model, other.version)


class Translation:
    def __init__(self, ti_ascii: str, display: str, accessible: str):
        self.ti_ascii = ti_ascii
        self.display = display
        self.accessible = accessible


class Token:
    def __init__(
        self,
        since: OsVersion,
        until: Optional[OsVersion],
        langs: Dict[str, Translation],
    ):
        self.since = since
        self.until = until
        self.langs = langs


class TokenData:
    def __init__(self, data: Union[List[Token], Dict[str, List[Token]]]):
        self.data = data


with open("../src/tokens/standard_tokens/8X.json", "r") as file:
    json_data = json.load(file)


def parse_os_version(data: Dict) -> OsVersion:
    return OsVersion(model=data["model"], version=data["os-version"])


def parse_translation(data: Dict) -> Translation:
    return Translation(
        ti_ascii=data["ti-ascii"],
        display=data["display"],
        accessible=data["accessible"],
    )


def parse_token(data: Dict) -> Token:
    since = parse_os_version(data["since"])
    until = parse_os_version(data.get("until")) if data.get("until") else None
    langs = {key: parse_translation(value) for key, value in data["langs"].items()}
    return Token(since=since, until=until, langs=langs)


tokens = {}
for key, value in json_data.items():
    if isinstance(value, list):
        tokens[key] = TokenData(data=[parse_token(token) for token in value])
    else:
        nested_tokens = {
            sub_key: [parse_token(token) for token in sub_value]
            for sub_key, sub_value in value.items()
        }
        tokens[key] = TokenData(data=nested_tokens)

with open("tokens.md", "w") as file:
    for key, token_data in tokens.items():
        file.write(f"## {key.replace("$", "")}\n")
        if isinstance(token_data.data, list):
            for token in token_data.data:
                file.write(f"### Since: {token.since.model} {token.since.version}\n")
                if token.until:
                    file.write(
                        f"#### Until: {token.until.model} {token.until.version}\n"
                    )
                for lang, translation in token.langs.items():
                    file.write(
                        f"- **{lang}**:\n - ti ascii: {translation.ti_ascii} \n - display: {translation.display} \n - accessible: {translation.accessible}\n"
                    )
        else:
            for sub_key, token_list in token_data.data.items():
                file.write(f"### {sub_key.replace("$", "")}\n")
                for token in token_list:
                    file.write(
                        f"#### Since: {token.since.model} {token.since.version}\n"
                    )
                    if token.until:
                        file.write(
                            f"##### Until: {token.until.model} {token.until.version}\n"
                        )
                    for lang, translation in token.langs.items():
                        file.write(
                            f"- **{lang}**:\n - ti ascii: {translation.ti_ascii} \n - display: {translation.display} \n - accessible: {translation.accessible}\n"
                        )
        file.write("\n")
