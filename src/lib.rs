use num_bigint::{BigUint, RandBigInt};
use rand::{thread_rng, Rng};

mod elliptic_curve;
use elliptic_curve::{EllipticCurve, FiniteField, Point};

struct ECDSA {
    ec: EllipticCurve,
    // group generator
    gen: Point,
    // group order
    order: BigUint,
}

impl ECDSA {
    pub fn generate_key_pair(&self) -> (BigUint, Point) {
        let private_key = self.gen_private_key();
        let public_key = self.generate_public_key(&private_key);
        (private_key, public_key)
    }

    fn gen_private_key(&self) -> BigUint {
        self.gen_random_n(&self.order)
    }

    fn gen_random_n(&self, max: &BigUint) -> BigUint {
        let mut rng = thread_rng();
        rng.gen_biguint_range(&BigUint::from(0u32), max)
    }

    fn generate_public_key(&self, pk: &BigUint) -> Point {
        self.ec.scalar_mul(&self.gen, pk)
    }

    pub fn generate_hash_less_than(data: &str, max: &BigUint) -> BigUint {
        let digest = sha256::digest(data);
        let hash_bytes = hex::decode(&digest).expect("Could not convert hash to Vec<u8>");
        let hash = BigUint::from_bytes_be(&hash_bytes)
            .modpow(&BigUint::from(1u32), &(max - BigUint::from(1u32)));
        let hash = hash + BigUint::from(1u32);
        hash
    }

    // R = k * G, r, _ = R(x , y)
    // s = (hash(m) + private_key * r) * k^(-1) mod q
    pub fn sign(&self, hash: &BigUint, private_key: &BigUint, k: &BigUint) -> (BigUint, BigUint) {
        assert!(hash < &self.order, "Hash is bigger than the order");
        assert!(private_key < &self.order, "Hash is bigger than the order");
        assert!(k < &self.order, "Hash is bigger than the order");

        if let Point::Coordinates(r, _) = self.ec.scalar_mul(&self.gen, k) {
            let ff = FiniteField {
                p: self.order.clone(),
            };
            let s = ff.mul(&r, private_key);
            let s = ff.add(&s, hash);
            let k_inv = ff.inv_mul(k);
            let s = ff.mul(&s, &k_inv);
            return (r, s);
        }

        panic!("The random point R is Identity element")
    }

    // u1 = s^(-1) * hash(message) mod q
    // u2 = s^(-1) * r mod q
    // P = u1 G + u2 public_key = (x, y)
    // if r == x then verified!
    pub fn verify(
        &self,
        hash: &BigUint,
        public_key: &Point,
        signature: &(BigUint, BigUint),
    ) -> bool {
        let ff = FiniteField {
            p: self.order.clone(),
        };
        let (r, s) = signature;
        let s_inv = ff.inv_mul(s);
        let u1 = ff.mul(&s_inv, hash);
        let u2 = ff.mul(&s_inv, r);
        let u1_point = self.ec.scalar_mul(&self.gen, &u1);
        let u2_point = self.ec.scalar_mul(public_key, &u2);
        if let Point::Coordinates(x, _) = self.ec.add(&u1_point, &u2_point) {
            return x == *r;
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::ECDSA;
    use crate::elliptic_curve::{EllipticCurve, Point};
    use num_bigint::BigUint;

    fn get_test_ecdsa() -> ECDSA {
        ECDSA {
            ec: EllipticCurve {
                a: BigUint::from(2u32),
                b: BigUint::from(2u32),
                p: BigUint::from(17u32),
            },
            gen: Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32)),
            order: BigUint::from(19u32),
        }
    }

    fn get_secp256k1_ec() -> ECDSA {
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

        ECDSA {
            ec: EllipticCurve { a, b, p },
            gen: Point::Coordinates(gx, gy),
            order: n,
        }
    }
    #[test]
    fn test_sign_verify() {
        let ecdsa = get_test_ecdsa();

        let private_key = BigUint::from(7u32);
        let public_key = ecdsa.generate_public_key(&private_key);

        let msg = "Bob transferring 1 coin to Alice";
        let hash = ECDSA::generate_hash_less_than(msg, &ecdsa.order);
        let k = BigUint::from(18u32);
        let signature = ecdsa.sign(&hash, &private_key, &k);

        let verify_result = ecdsa.verify(&hash, &public_key, &signature);
        assert!(verify_result, "Verification is false")
    }

    #[test]
    fn test_sign_tempered_message() {
        let ecdsa = get_test_ecdsa();

        let private_key = BigUint::from(7u32);
        let public_key = ecdsa.generate_public_key(&private_key);

        let msg = "Bob transferring 1 coin to Alice";
        let hash = ECDSA::generate_hash_less_than(msg, &ecdsa.order);
        let k = BigUint::from(18u32);
        let signature = ecdsa.sign(&hash, &private_key, &k);

        let msg = "Bob transferring 100 coin to Alice";
        let hash = ECDSA::generate_hash_less_than(msg, &ecdsa.order);
        let verify_result = ecdsa.verify(&hash, &public_key, &signature);
        assert!(!verify_result, "Verification is true")
    }

    #[test]
    fn test_sign_tempered_signature() {
        let ecdsa = get_test_ecdsa();

        let private_key = BigUint::from(7u32);
        let public_key = ecdsa.generate_public_key(&private_key);

        let msg = "Bob transferring 1 coin to Alice";
        let hash = ECDSA::generate_hash_less_than(msg, &ecdsa.order);
        let k = BigUint::from(18u32);
        let signature = ecdsa.sign(&hash, &private_key, &k);

        let (r, s) = signature;
        let tempered_signature = (r + BigUint::from(1u32), s);
        let verify_result = ecdsa.verify(&hash, &public_key, &tempered_signature);
        assert!(!verify_result, "Verification is true")
    }
    #[test]
    fn test_secp256k1_sign_verify() {
        let ecdsa = get_secp256k1_ec();
        let private_key = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364140",
            16,
        )
        .expect("could not convert str to private_key");
        let public_key = ecdsa.generate_public_key(&private_key);

        let msg = "Bob transferring 1 coin to Alice";
        let hash = ECDSA::generate_hash_less_than(msg, &ecdsa.order);
        let k = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFAAAEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .expect("could not convert str to k");
        let signature = ecdsa.sign(&hash, &private_key, &k);

        let verify_result = ecdsa.verify(&hash, &public_key, &signature);
        assert!(verify_result, "Verification is false")
    }

    #[test]
    fn test_secp256k1_tempered_message() {
        let ecdsa = get_secp256k1_ec();
        let private_key = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364140",
            16,
        )
        .expect("could not convert str to private_key");
        let public_key = ecdsa.generate_public_key(&private_key);

        let msg = "Bob transferring 1 coin to Alice";
        let hash = ECDSA::generate_hash_less_than(msg, &ecdsa.order);
        let k = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFAAAEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .expect("could not convert str to k");
        let signature = ecdsa.sign(&hash, &private_key, &k);

        let msg = "Bob transferring 100 coin to Alice";
        let hash = ECDSA::generate_hash_less_than(msg, &ecdsa.order);
        let verify_result = ecdsa.verify(&hash, &public_key, &signature);
        assert!(!verify_result, "Verification is true")
    }

    #[test]
    fn test_secp256k1_tempered_signature() {
        let ecdsa = get_secp256k1_ec();
        let private_key = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364140",
            16,
        )
        .expect("could not convert str to private_key");
        let public_key = ecdsa.generate_public_key(&private_key);

        let msg = "Bob transferring 1 coin to Alice";
        let hash = ECDSA::generate_hash_less_than(msg, &ecdsa.order);
        let k = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFAAAEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .expect("could not convert str to k");
        let signature = ecdsa.sign(&hash, &private_key, &k);
        let (r, s) = signature;
        let tempered_signature = (r + BigUint::from(1u32), s);
        let verify_result = ecdsa.verify(&hash, &public_key, &tempered_signature);
        assert!(!verify_result, "Verification is true")
    }
}
