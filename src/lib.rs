use num_bigint::BigUint;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::ops::Sub;

#[derive(Debug)]
enum Point {
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
                let lambda_sq = lambda.modpow(&BigUint::from(2u32), &self.p);

                // x3 = lambda^2 - x1 -x2 (mod p)
                let x3 = f.sub(&f.sub(&lambda_sq, x1), x2);

                // y3 = lambda(x1 - x3) - y1 (mod p)
                let y3 = f.sub(&f.mul(&lambda, &f.sub(x1, &x3)), y1);
                Point::Coordinates(x3, y3)
            }
            (Point::Identity, Point::Identity) => Point::Identity,
        }
    }

    fn double(&self, c: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point {} is not on curve", c);

        match c {
            Point::Identity => Point::Identity,
            Point::Coordinates(x, y) => {
                let f = FiniteField { p: self.p.clone() };

                // lambda = (3x^2 + a) / 2y
                let x_sq = f.mul(x, x);
                let numerator = f.add(&f.mul(&x_sq, &BigUint::from(3u32)), &self.a);
                let denominator = f.mul(&BigUint::from(2u32), y);
                let lambda = f.div(&numerator, &denominator);
                let lambda_sq = f.mul(&lambda, &lambda);

                // x2 = lambda^2 - 2x1
                let x2 = f.sub(&lambda_sq, &f.mul(&BigUint::from(2u32), x));

                // y2 = lambda(x1 - x2) - y1
                let y2 = f.sub(&f.mul(&f.sub(x, &x2), &lambda), y);
                Point::Coordinates(x2, y2)
            }
        }
    }

    fn scalar_mul(&self, r: &Point, q: &Point) {
        todo!()
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

struct FiniteField {
    p: BigUint,
}

impl FiniteField {
    fn add(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let sum = a + b;
        sum.modpow(&BigUint::from(1u32), &self.p)
    }
    fn mul(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let product = a * b;
        product.modpow(&BigUint::from(1u32), &self.p)
    }

    fn sub(&self, a: &BigUint, b: &BigUint) -> BigUint {
        self.add(a, &self.inv_add(b))
    }

    fn div(&self, a: &BigUint, b: &BigUint) -> BigUint {
        self.mul(a, &self.inv_mul(b))
    }

    fn inv_add(&self, n: &BigUint) -> BigUint {
        assert!(
            n < &self.p,
            "number: {} is bigger or equal than modulus {}",
            n,
            &self.p
        );
        &self.p - n
    }

    fn inv_mul(&self, n: &BigUint) -> BigUint {
        n.modpow(&(&self.p - BigUint::from(2u32)), &self.p)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_add() {
        let f = FiniteField {
            p: BigUint::from(11u32),
        };
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let sum = f.add(&a, &b);

        assert_eq!(sum, BigUint::from(3u32));
    }

    #[test]
    fn test_add_2() {
        let f = FiniteField {
            p: BigUint::from(32u32),
        };
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let sum = f.add(&a, &b);

        assert_eq!(sum, BigUint::from(14u32));
    }

    #[test]
    fn test_mul() {
        let f = FiniteField {
            p: BigUint::from(11u32),
        };
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let prod = f.mul(&a, &b);

        assert_eq!(prod, BigUint::from(7u32));
    }

    #[test]
    fn test_inv_add() {
        let f = FiniteField {
            p: BigUint::from(51u32),
        };
        let a = BigUint::from(4u32);
        let prod = f.inv_add(&a);

        assert_eq!(prod, BigUint::from(47u32));
    }

    #[test]
    #[should_panic]
    fn test_inv_add_2() {
        let f = FiniteField {
            p: BigUint::from(51u32),
        };
        let a = BigUint::from(52u32);
        let _ = f.inv_add(&a);
    }

    #[test]
    fn test_inv_mul() {
        let f = FiniteField {
            p: BigUint::from(11u32),
        };
        let a = BigUint::from(4u32);
        let prod = f.inv_mul(&a);
        assert_eq!(prod, BigUint::from(3u32));
    }

    #[test]
    fn test_sub() {
        let f = FiniteField {
            p: BigUint::from(11u32),
        };
        let a = BigUint::from(10u32);
        let b = BigUint::from(4u32);
        let prod = f.sub(&a, &b);
        assert_eq!(prod, BigUint::from(6u32));
    }

    #[test]
    fn test_sub_1() {
        let f = FiniteField {
            p: BigUint::from(11u32),
        };
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let prod = f.sub(&a, &b);
        assert_eq!(prod, BigUint::from(5u32));
    }

    #[test]
    fn tes_div() {
        let f = FiniteField {
            p: BigUint::from(11u32),
        };
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let prod = f.div(&a, &b);
        assert_eq!(prod, BigUint::from(7u32));
    }

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
    fn test_point_doubling(){
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

}
