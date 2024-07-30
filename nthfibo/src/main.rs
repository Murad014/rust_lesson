fn main() {
    let ans = fib(1);

    println!("{}", ans);
}


pub fn fib(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;

    if n == 0 {return 0}
    if n == 1 || n == 2{return 1}

    for i in 3..n+1 {
        let calc = a + b;
        a = b;
        b = calc;
    }

    b
}
