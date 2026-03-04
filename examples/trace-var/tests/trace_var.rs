use trace_var::trace_var;

#[trace_var(p, n)]
fn factorial(mut n: u64) -> u64 {
    let mut p = 1u64;
    while n > 1 {
        p *= n;
        n -= 1;
    }
    p
}

#[test]
fn factorial_correctness() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
}

#[trace_var(x)]
fn double(x: i32) -> i32 {
    let x = x * 2;
    x
}

#[test]
fn double_correctness() {
    assert_eq!(double(3), 6);
    assert_eq!(double(0), 0);
}
