/// Calculates the greatest common divisor using Euclid's algorithm
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    return a;
}

/// Calculates the least common multiple of a and b
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn gcd() {
        struct TestCase {
            a: usize,
            b: usize,
            expected: usize,
        }

        let cases = [
            TestCase {
                a: 5,
                b: 1,
                expected: 1,
            },
            TestCase {
                a: 10,
                b: 4,
                expected: 2,
            },
            TestCase {
                a: 33,
                b: 27,
                expected: 3,
            },
            TestCase {
                a: 27,
                b: 33,
                expected: 3,
            },
        ];

        for tc in &cases {
            let result = super::gcd(tc.a, tc.b);
            assert_eq!(tc.expected, result);
        }
    }
}
