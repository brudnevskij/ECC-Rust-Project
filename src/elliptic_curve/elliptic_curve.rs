use super::finite_field::FiniteField;
use num_bigint::BigUint;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Point {
    Coordinates(BigUint, BigUint),
    Identity,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Point::Coordinates(x, y) => write!(f, "x: {}, y: {}", x, y),
            Point::Identity => write!(f, "Point at infinity"),
        }
    }
}

pub struct EllipticCurve {
    // y^2=x^2+a*x+b
    pub a: BigUint,
    pub b: BigUint,
    pub p: BigUint,
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
    pub fn add(&self, r: &Point, q: &Point) -> Point {
        assert!(self.is_on_curve(r), "Point {} is not on curve", r);
        assert!(self.is_on_curve(q), "Point {} is not on curve", q);
        assert_ne!(r, q, "Points should not be the same");

        match (r, q) {
            (Point::Identity, Point::Coordinates(x, y)) => Point::Coordinates(x.clone(), y.clone()),
            (Point::Coordinates(x, y), Point::Identity) => Point::Coordinates(x.clone(), y.clone()),
            (Point::Coordinates(x1, y1), Point::Coordinates(x2, y2)) => {
                let f = FiniteField { p: self.p.clone() };

                // logic for reflected points
                let y_sum = f.add(y1, y2);
                if x1 == x2 && y_sum == BigUint::from(0u32) {
                    return Point::Identity;
                }

                // lambda = (y2 - y1) / (x2 - x1)
                let d_y = f.sub(y2, y1);
                let d_x = f.sub(x2, x1);
                let lambda = f.div(&d_y, &d_x);

                let (x3, y3) = self.calculate_x3_y3(&lambda, x1, x2, y1);
                Point::Coordinates(x3, y3)
            }
            (Point::Identity, Point::Identity) => Point::Identity,
        }
    }

    pub fn double(&self, c: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point {} is not on curve", c);

        match c {
            Point::Identity => Point::Identity,
            Point::Coordinates(x, y) => {
                // if P = Q, y = y => 2P = e
                if y == &BigUint::from(0u32) {
                    return Point::Identity;
                }

                let f = FiniteField { p: self.p.clone() };

                // lambda = (3x^2 + a) / 2y
                let x_sq = f.mul(x, x);
                let numerator = f.add(&f.mul(&x_sq, &BigUint::from(3u32)), &self.a);
                let denominator = f.mul(&BigUint::from(2u32), y);
                let lambda = f.div(&numerator, &denominator);

                let (x2, y2) = self.calculate_x3_y3(&lambda, x, x, y);
                Point::Coordinates(x2, y2)
            }
        }
    }

    pub fn calculate_x3_y3(
        &self,
        lambda: &BigUint,
        x1: &BigUint,
        x2: &BigUint,
        y1: &BigUint,
    ) -> (BigUint, BigUint) {
        let f = FiniteField { p: self.p.clone() };

        let lambda_sq = f.mul(&lambda, &lambda);
        // x3 = lambda^2 - x1 -x2 (mod p)
        let x3 = f.sub(&f.sub(&lambda_sq, x1), x2);
        // y3 = lambda(x1 - x3) - y1 (mod p)
        let y3 = f.sub(&f.mul(&lambda, &f.sub(x1, &x3)), y1);
        (x3, y3)
    }

    pub fn scalar_mul(&self, c: &Point, d: &BigUint) -> Point {
        assert!(self.is_on_curve(c), "Point {} is not on curve", c);

        let mut t = (*c).clone();
        for i in (0..(d.bits() - 1)).rev() {
            t = self.double(&t);
            if d.bit(i) {
                t = self.add(&t, &c);
            }
        }
        t
    }

    pub fn is_on_curve(&self, c: &Point) -> bool {
        match c {
            Point::Coordinates(x, y) => {
                let y_sq = y.modpow(&BigUint::from(2u32), &self.p);
                let x_cb = x.modpow(&BigUint::from(3u32), &self.p);
                y_sq == (x_cb + &self.a * x + &self.b).modpow(&BigUint::from(1u32), &self.p)
            }
            Point::Identity => return true,
        }
    }
}

mod ec_test {
    use super::{BigUint, EllipticCurve, FiniteField, Point};

    #[test]
    fn test_ec_point_addition() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (6,3) + (5,1) = (10, 6)
        let p1 = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32));
        let r = Point::Coordinates(BigUint::from(10u32), BigUint::from(6u32));
        let sum = ec.add(&p1, &p2);
        assert_eq!(r, sum);

        // e + (6,3) = (6,3)
        let p1 = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Identity;
        let sum = ec.add(&p1, &p2);
        assert_eq!(p1, sum);

        // (6,3) + e = (6,3)
        let p1 = Point::Identity;
        let p2 = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));
        let sum = ec.add(&p1, &p2);
        assert_eq!(p2, sum);

        // Reflected points
        // (6,3) + (6,-3) = e
        let f = FiniteField {
            p: BigUint::from(17u32),
        };
        let p1 = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Coordinates(BigUint::from(6u32), f.inv_add(&BigUint::from(3u32)));
        let sum = ec.add(&p1, &p2);
        assert_eq!(sum, Point::Identity);
    }

    #[test]
    #[should_panic]
    fn test_ec_point_addition_same_points_assertion() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let p1 = Point::Identity;
        let p2 = Point::Identity;
        let _ = ec.add(&p1, &p2);
    }

    #[test]
    #[should_panic]
    fn test_ec_point_addition_p1_not_on_curve_assertion() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let p1 = Point::Coordinates(BigUint::from(63u32), BigUint::from(3u32));
        let p2 = Point::Identity;
        let _ = ec.add(&p1, &p2);
    }

    #[test]
    #[should_panic]
    fn test_ec_point_addition_p2_not_on_curve_assertion() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let p1 = Point::Identity;
        let p2 = Point::Coordinates(BigUint::from(63u32), BigUint::from(3u32));
        let _ = ec.add(&p1, &p2);
    }

    #[test]
    fn test_point_doubling() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (5,1) + (5,1) = (6,3)
        let p1 = Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32));
        let r = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));
        let sum = ec.double(&p1);
        assert_eq!(r, sum);

        let p1 = Point::Identity;
        let r = Point::Identity;
        let sum = ec.double(&p1);
        assert_eq!(r, sum);
    }

    #[test]
    #[should_panic]
    fn test_point_doubling_on_curve_assertion() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let p2 = Point::Coordinates(BigUint::from(63u32), BigUint::from(3u32));
        let _ = ec.double(&p2);
    }

    #[test]
    fn test_scalar_mul() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // 2 (5,1) = (6,3)
        let p1 = Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32));
        let r = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));
        let product = ec.scalar_mul(&p1, &BigUint::from(2u32));
        assert_eq!(r, product);

        // 10 (5,1) = (7,11)
        let p1 = Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32));
        let r = Point::Coordinates(BigUint::from(7u32), BigUint::from(11u32));
        let product = ec.scalar_mul(&p1, &BigUint::from(10u32));
        assert_eq!(r, product);

        // 19 (5,1) = e
        let p1 = Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32));
        let r = Point::Identity;
        let product = ec.scalar_mul(&p1, &BigUint::from(19u32));
        assert_eq!(r, product)
    }

    #[test]
    fn test_ec_secp256k1() {
        /*
            https://en.bitcoin.it/wiki/Secp256k1
            p = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F
            a = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
            b = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000007

            G = {
                    x = 79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798,
                    y = 483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8
                }
            n = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141
        */

        let p = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .expect("could not convert str to p");
        let a = BigUint::parse_bytes(
            b"0000000000000000000000000000000000000000000000000000000000000000",
            16,
        )
        .expect("could not convert str to a");
        let b = BigUint::parse_bytes(
            b"0000000000000000000000000000000000000000000000000000000000000007",
            16,
        )
        .expect("could not convert str to b");
        let n = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .expect("could not convert str to n");
        let gx = BigUint::parse_bytes(
            b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .expect("could not convert str to gx");
        let gy = BigUint::parse_bytes(
            b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        )
        .expect("could not convert str to gy");

        let ec = EllipticCurve { a, b, p };
        let g = Point::Coordinates(gx, gy);
        let res = ec.scalar_mul(&g, &n);
        // n * g = I, n is an order of the group
        assert_eq!(Point::Identity, res);
    }
}
