use num_bigint::BigUint;
use std::cmp::PartialEq;
use std::fmt::{Display, format, Formatter};
use std::ops::Sub;

enum Point {
    Coordinates(BigUint, BigUint),
    Identity,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Point::Coordinates(x, y) => write!(f,"x: {}, y: {}", x,y),
            Point::Identity => write!(f,"Point at infinity"),
        }
    }
}

struct EllipticCurve {
    // y^2=x^2+a*x+b
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Point::Identity, Point::Identity) => true,
            (Point::Coordinates(x1, y1), Point::Coordinates(x2, y2)) => x1 == x2 && y1 == y2,
            (_, _) => false,
        }
    }
}

impl EllipticCurve {
    fn add(&self, r: &Point, q: &Point) -> Point {
        assert!(self.is_on_curve(r), "Point {} is not on curve", r);
        assert!(self.is_on_curve(q), "Point {} is not on curve", q);
        assert!(r != q, "Points should not be the same");

        match (r, q) {
            (Point::Identity, _) => Point::Identity,
            (_, Point::Identity) => Point::Identity,
            (Point::Coordinates(x1, y1), Point::Coordinates(x2, y2)) => {
                let d_y = y2 + FiniteField::inv_add(y1, &self.p);
                let d_x = x2 + FiniteField::inv_add(x1, &self.p);
                let lambda = d_y * FiniteField::inv_mul(&d_x, &self.p);
                let lambda_sq = lambda.modpow(&BigUint::from(2u32), &self.p);
                let x3 = lambda_sq
                    + FiniteField::inv_add(x1, &self.p)
                    + FiniteField::inv_add(x2, &self.p);
                let y3 = lambda * (x1 + FiniteField::inv_add(&x3, &self.p));
                Point::Coordinates(x3, y3)
            }
        }
    }

    fn double(&self, r: &Point, q: &Point) {
        todo!()
    }

    fn scalar_mul(&self, r: &Point, q: &Point) {
        todo!()
    }

    pub fn is_on_curve(&self, c: &Point) -> bool {
        match c {
            Point::Coordinates(x, y) => {
                let y_sq = y.modpow(&BigUint::from(2u32), &self.p);
                let x_cb = x.modpow(&BigUint::from(3u32), &self.p);
                y_sq == x_cb + &self.a * x + &self.b
            }
            Point::Identity => return false,
        }
    }
}

struct FiniteField {}

impl FiniteField {
    fn add(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        let sum = a + b;
        sum.modpow(&BigUint::from(1u32), &p)
    }
    fn mul(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        let product = a * b;
        product.modpow(&BigUint::from(1u32), &p)
    }
    fn inv_add(n: &BigUint, p: &BigUint) -> BigUint {
        assert!(n < p, "number: {} is bigger or equal than modulus {}", n, p);
        p - n
    }

    fn inv_mul(n: &BigUint, p: &BigUint) -> BigUint {
        n.modpow(&(p - BigUint::from(2u32)), p)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let sum = FiniteField::add(&a, &b, &p);

        assert_eq!(sum, BigUint::from(3u32));
    }

    #[test]
    fn test_add_2() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(32u32);

        let sum = FiniteField::add(&a, &b, &p);

        assert_eq!(sum, BigUint::from(14u32));
    }

    #[test]
    fn test_mul() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let prod = FiniteField::mul(&a, &b, &p);

        assert_eq!(prod, BigUint::from(7u32));
    }

    #[test]
    fn test_inv_add() {
        let a = BigUint::from(4u32);
        let p = BigUint::from(51u32);

        let prod = FiniteField::inv_add(&a, &p);

        assert_eq!(prod, BigUint::from(47u32));
    }

    #[test]
    #[should_panic]
    fn test_inv_add_2() {
        let a = BigUint::from(52u32);
        let p = BigUint::from(51u32);

        let _ = FiniteField::inv_add(&a, &p);
    }

    #[test]
    fn test_inv_mul() {
        let a = BigUint::from(4u32);
        let p = BigUint::from(11u32);

        let prod = FiniteField::inv_mul(&a, &p);

        assert_eq!(prod, BigUint::from(3u32));
    }
}
