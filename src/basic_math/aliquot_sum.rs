/// The aliquot sum s(n) of a positive integer n is the sum of all proper divisors of n, that is, all divisors of n other than the number n itself.
///
/// For example, the aliquot sum of 15 is (1 + 3 + 5) = 9

pub fn aliquot_sum(n: u64) -> u64 {
    //loop through all numbers from 1..(n/2) and check if they divide n

    if n == 1 || n == 0 {
        return 0;
    }

    let mut sum: u64 = 0;

    for i in 1..(n / 2 + 1) {
        if n % i == 0 {
            sum += i;
        }
    }

    sum
}
