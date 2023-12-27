def generate_rust_match(input_filename, output_filename):
    try:
        with open(input_filename, 'r') as file:
            lines = file.readlines()

        match_cases = []

        for line in lines:
            # Split the line into the index and the company name
            parts = line.split("\t")
            # Check if the line is valid (contains at least an index and a name)
            if len(parts) >= 2:
                index = parts[0].strip()
                name = parts[1].strip().replace('\"', '\\\"')  # Escape double quotes
                # Create the match case string for this company code
                match_cases.append(f"        {index} => \"{name}\",\n")

        # Open the output Rust file
        with open(output_filename, 'w') as output_file:
            # Write the function signature and the start of the match block
            output_file.write(
                "    match code {\n"
            )

            # Write all the match cases
            output_file.writelines(match_cases)

            # Write the fallback case and close the match block and function
            output_file.write(
                "        _ => \"Unknown company code\",\n"
                "    }\n"
            )
        print(f"Match statement successfully generated in {output_filename}")

    except FileNotFoundError:
        print(f"Error: The file '{input_filename}' was not found.")
    except Exception as e:
        print(f"An error occurred: {e}")

# Call the function with the appropriate filenames
generate_rust_match('company_ids.txt', 'match_statement.rs')
