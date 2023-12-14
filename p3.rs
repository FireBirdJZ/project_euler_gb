fn main() {
    let n = 600_851_475_143;
    let factors = prime_factors(n);
    if let Some(&largest) = factors.last() {
        assert!(largest == 6857);
        println!("Largest prime factor is: {}", largest);
    } else {
        println!("No prime factors found.");
    }
}

// fn sieve_of_eratosthenes(n: u64) -> (Vec<u64>, usize){
//     let mut primes = vec![0; n as usize / 2];
//     let mut count = 0;

//     if n < 2: return {(primes, count)}

//     let sieve_size = n as usize + 1;
//     let mut sieve = vec![true; sieve_size];

//     sieve[0] = false;
//     sieve[1] = false;

//     for num in 2..sieve_size {
//         if sieve[num]{
//             primes[count] = num as u64;
//             count += 1;
//         }

//         let mut multiple = num * num;
//         while multiple < sieve_size {
//             sieve[multiple] = false;
//             multiple += num;
//         }
//     }
//     (primes, count)
// }

fn prime_factors(n: u64) -> Vec<u64>{
    let mut factors = Vec::new();
    let mut remainder = n;

    // Factor out 2s; Don't need this because n is odd
    // while remainder % 2 == 0 {
    //     factors.push(2);
    //     remainder /= 2;
    // }

    // Factor out odd numbers starting from 3
    let mut i = 3;
    while i * i <= remainder {
        while remainder % i == 0 {
            factors.push(i);
            remainder /= i;
        }
        i += 2;
    }

    // If remainder is a prime number greater than 2
    if remainder > 2 {
        factors.push(remainder);
    }

    factors
}


