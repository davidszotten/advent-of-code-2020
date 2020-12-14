fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    // (b, x, y)

    if a == 0 {
        return (b, 0, 1);
    }

    let (g, x, y) = gcd_extended(b % a, a);

    return (g, y - (b / a) * x, x);
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (g, x, _) = gcd_extended(a, m);
    if g != 1 {
        unimplemented!("inverse doesn't exist");
    }
    // m is added to handle negative x
    (x % m + m) % m
}

pub fn mod_div(a: i64, b: i64, m: i64) -> i64 {
    mod_mul(a, mod_inverse(b, m), m)
}

pub fn mod_mul(a: i64, b: i64, m: i64) -> i64 {
    let mut res = 0;
    let mut a = a % m;
    let mut b = b;
    while b > 0 {
        // If b is odd, add 'a' to result
        if b % 2 == 1 {
            res = (res + a) % m;
        }

        // Multiply 'a' with 2
        a = (a * 2) % m;
        // Divide b by 2
        b /= 2;
    }

    res % m
}
