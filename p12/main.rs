use num::pow;
use num::BigUint;

use num::integer;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let c1 = BigUint::from_bytes_be(&fs::read("c1.txt")?);
    let c2 = BigUint::from_bytes_be(&fs::read("c2.txt")?);
    let p1 = BigUint::from_bytes_be(&fs::read("p1.txt")?);
    let p2 = BigUint::from_bytes_be(&fs::read("p2.txt")?);

    let p1e: BigUint = pow::pow(p1, 65537) - c1;
    let p2e: BigUint = pow::pow(p2, 65537) - c2;

    println!("{}", integer::gcd(p1e, p2e));
    Ok(())
}
