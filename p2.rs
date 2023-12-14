fn main() {
    solution1();
}

fn solution1(){
    let max_limit = 4_000_000;
    let result = fib(max_limit);
    assert!(result == 4613732);
    println!("result: {result}");
}

fn fib(max_limit: u64) -> u64{
    let mut a = 1;
    let mut b = 2;
    let mut answer = 0;

    while b <= max_limit{
        if b % 2 == 0{
            answer += b;
        }
        let c = a + b;
        a = b;
        b = c;
    }
    answer
}
