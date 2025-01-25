use std::fmt::format;
use std::ops::Sub;
use num_bigint::{BigUint};

struct Point{
    x:BigUint,
    y:BigUint
}

struct EllipticCurve{
    // y^2=x^2+a*x+b
    a: BigUint,
    b: BigUint,
    p: BigUint
}

impl EllipticCurve{
    fn add(&self, r:&Point, q: &Point){
        todo!()
    }

    fn double(&self, r:&Point, q: &Point){
        todo!()
    }

    fn scalar_mul(&self, r:&Point, q: &Point){
        todo!()
    }
}

struct FiniteField {}

impl FiniteField {
    fn add(a: &BigUint, b:&BigUint, p: &BigUint)-> BigUint{
        let sum = a+b;
        sum.modpow(&BigUint::from(1u32), &p)
    }
    fn mul(a: &BigUint, b:&BigUint, p: &BigUint)-> BigUint{
        let product = a * b;
        product.modpow(&BigUint::from(1u32), &p)
    }
    fn inv_add(n: &BigUint, p: &BigUint)-> BigUint{
        assert!(n<p, "number: {} is bigger or equal than modulus {}", n, p);
        p - n
    }

    fn inv_mul(n: &BigUint, p: &BigUint)-> BigUint{
        n.modpow(&(p-BigUint::from(2u32)), p)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_add(){
        let a= BigUint::from(4u32);
        let b= BigUint::from(10u32);
        let p= BigUint::from(11u32);

        let sum = FiniteField::add(&a,&b,&p);

        assert_eq!(sum, BigUint::from(3u32));
    }

    #[test]
    fn test_add_2(){
        let a= BigUint::from(4u32);
        let b= BigUint::from(10u32);
        let p= BigUint::from(32u32);

        let sum = FiniteField::add(&a,&b,&p);

        assert_eq!(sum, BigUint::from(14u32));
    }

    #[test]
    fn test_mul(){
        let a= BigUint::from(4u32);
        let b= BigUint::from(10u32);
        let p= BigUint::from(11u32);

        let prod = FiniteField::mul(&a,&b,&p);

        assert_eq!(prod, BigUint::from(7u32));
    }

    #[test]
    fn test_inv_add(){
        let a= BigUint::from(4u32);
        let p= BigUint::from(51u32);

        let prod = FiniteField::inv_add(&a,&p);

        assert_eq!(prod, BigUint::from(47u32));
    }

    #[test]
    #[should_panic]
    fn test_inv_add_2(){
        let a= BigUint::from(52u32);
        let p= BigUint::from(51u32);

        let _ = FiniteField::inv_add(&a,&p);
    }

    #[test]
    fn test_inv_mul(){
        let a= BigUint::from(4u32);
        let p= BigUint::from(11u32);

        let prod = FiniteField::inv_mul(&a,&p);

        assert_eq!(prod, BigUint::from(3u32));
    }
}