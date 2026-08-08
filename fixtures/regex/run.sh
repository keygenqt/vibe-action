#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# 1. Email matching with example
cargo run -- regex -q 'match an email address' -e 'user@example.com'

# 2. Extract domain from URL with example
cargo run -- regex -q 'extract domain name from a URL' -e 'https://www.example.com/path'

# 3. IPv4 address validation with example
cargo run -- regex -q 'match a valid IPv4 address' -e '192.168.1.1'

# 4. Date format without example
cargo run -- regex -q 'match a date in YYYY-MM-DD format'

# 5. Phone number with example
cargo run -- regex -q 'match a US phone number' -e '(123) 456-7890'
