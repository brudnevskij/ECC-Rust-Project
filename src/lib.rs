use num_bigint::BigUint;
mod elliptic_curve;
use elliptic_curve::{EllipticCurve, Point, FiniteField};

struct ECDSA {
    ec: EllipticCurve,
    // group generator
    gen: Point,
    // group order
    order: BigUint
}

impl ECDSA {
    pub fn generate_key_pair(&self)-> (BigUint,Point){
        let private_key = self.gen_private_key();
        let public_key = self.generate_public_key(&private_key);
        (private_key,public_key)

    }

    fn gen_private_key(&self)-> BigUint{
        self.gen_random_n(&self.order)
    }

    fn gen_random_n(&self, max: &BigUint)-> BigUint {
        todo!()
    }

    fn generate_public_key(&self, pk: &BigUint)-> Point{
        todo!()
    }

    pub fn sign(&self, hash: &BigUint, k: &BigUint)-> (BigUint, BigUint){
        todo!()
    }

    pub fn verify(&self, hash: &BigUint, public_key: &Point, signature: &BigUint )-> (BigUint, BigUint){
        todo!()
    }
}