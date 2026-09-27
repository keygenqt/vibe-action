fn memoized_fib() -> impl Fn(u32) -> u64 {
    let mut cache = vec![0u64, 1u64];
    move |n: u32| -> u64 {
        if (n as usize) < cache.len() {
            cache[n as usize]
        } else {
            let val = memoized_fib()(n - 1) + memoized_fib()(n - 2);
            cache.push(val);
            val
        }
    }
}
