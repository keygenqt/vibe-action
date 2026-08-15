#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- whois './fixtures/whois/photo_1.jpg'
cargo run -- whois './fixtures/whois/photo_2.jpg'
