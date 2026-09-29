// SPDX-FileCopyrightText: Copyright 2025 Fixture Author <fixture@example.com>
// SPDX-License-Identifier: MIT
fn main() {
    println!("Hello, world!");
    // TODO: remove debug output
    // New feature: print current time
    let now = chrono::Local::now();
    println!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S"));
}
