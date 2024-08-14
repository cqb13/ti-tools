import json

with open("./tokens.json") as file:
    json_data = json.load(file)

    tokens = {}
    for key, value in json_data.items():
        about = []
        syntaxes = value["syntaxes"]

        if syntaxes is None:
            print(f"Skipping {key} because it has no syntaxes")
            continue

        for syntax in syntaxes:
            if syntax["syntax"] is None or syntax["syntax"] == "":
                print(f"Warning {key} because it has no syntax")

            if syntax["description"] is None or syntax["description"] == "":
                print(f"Skipping {key} because it has no description")
                about.append({"syntax": syntax["syntax"], "description": ""})
                continue

            syntax_info = {
                "syntax": syntax["syntax"],
                "description": syntax["description"],
            }
            about.append(syntax_info)

        tokens[key] = about

    with open("./token_definitions.json", "w") as file:
        json.dump(tokens, file, indent=2)
