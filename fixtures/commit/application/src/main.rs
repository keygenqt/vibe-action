fn main() {
    println!("Hello, world!");

    // New feature: print current time
    let now = chrono::Local::now();
    println!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S"));
}
