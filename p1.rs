fn main() {
    solution1();
}

fn solution1(){
    let sum: u32 = (0..1000)
        .filter(|i| i % 3 == 0 || i % 5 == 0)
        .sum();

    assert!(sum == 233168);
    println!("Sum: {}", sum);
}
