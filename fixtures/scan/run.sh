#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# Helper function to run scan and print the generated JSON
run_scan() {
    local project_path=$1
    local output

    # Run scan and capture stdout
    output=$(cargo run -- scan -p "$project_path")
    echo "$output"

    # Extract the saved file path from the output (e.g., "Saved to /path/to/file.json")
    local file_path
    file_path=$(echo "$output" | grep -oE 'Saved to [^ ]+' | awk '{print $3}')

    # If file path is found, print its content
    if [ -n "$file_path" ] && [ -f "$file_path" ]; then
        echo -e "--- JSON content ---"
        cat "$file_path"
        echo -e "\n----------------------"
        rm "$file_path"
    fi
}

run_scan "./fixtures/scan/kotlin"
run_scan "./fixtures/scan/rust"
run_scan "./fixtures/scan/java"
