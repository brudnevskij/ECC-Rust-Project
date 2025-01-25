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
        todo!()
    }
    fn mul(a: &BigUint, b:&BigUint, p: &BigUint)-> BigUint{
        todo!()
    }
    fn inv_add(n: &BigUint, p: &BigUint)-> BigUint{
        todo!()
    }

    fn inv_mul(n: &BigUint, p: &BigUint)-> BigUint{
        todo!()
    }
}