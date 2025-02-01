use num_bigint::BigUint;

pub struct FiniteField {
    pub p: BigUint,
}

impl FiniteField {
    pub fn add(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let sum = a + b;
        sum.modpow(&BigUint::from(1u32), &self.p)
    }
    pub fn mul(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let product = a * b;
        product.modpow(&BigUint::from(1u32), &self.p)
    }

    pub fn sub(&self, a: &BigUint, b: &BigUint) -> BigUint {
        self.add(a, &self.inv_add(b))
    }

    pub fn div(&self, a: &BigUint, b: &BigUint) -> BigUint {
        self.mul(a, &self.inv_mul(b))
    }

    pub fn inv_add(&self, n: &BigUint) -> BigUint {
        assert!(
            n < &self.p,
            "number: {} is bigger or equal than modulus {}",
            n,
            &self.p
        );
        &self.p - n
    }

    pub fn inv_mul(&self, n: &BigUint) -> BigUint {
        n.modpow(&(&self.p - BigUint::from(2u32)), &self.p)
    }
}

mod ff_test {
    use super::{FiniteField, BigUint};

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

}
