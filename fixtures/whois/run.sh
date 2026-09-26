#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- vision whois './fixtures/whois/photo_1.jpg'
cargo run -- vision whois './fixtures/whois/photo_2.jpg'
