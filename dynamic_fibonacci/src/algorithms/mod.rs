pub fn fibonacci_on_the_fly(values: &mut Vec<i64>, n: i64) -> i64 {
    if (n as usize) < values.len() {
        return values[n as usize];
    }
    let val = fibonacci_on_the_fly(values, n - 2) + fibonacci_on_the_fly(values, n - 1);
    values.push(val);
    return val;
}
