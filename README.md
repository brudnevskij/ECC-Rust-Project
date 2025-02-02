# **Elliptic Curve Implementation in Rust**

## 📌 Overview
This project implements an **elliptic curve** and **finite field** in Rust. It includes:
- A **custom struct** for elliptic curve operations.
- A **finite field element** struct that supports all standard field operations.
- An **ECDSA (Elliptic Curve Digital Signature Algorithm) implementation** that utilizes the above structures.

## 📂 Project Structure
- `elliptic_curve/`: Contains the `EllipticCurve`, `Point`, and `FiniteField` primitives.
- `lib.rs`: Implements the **ECDSA algorithm**.

## ✅ Tests
- The implementation is tested with multiple curves, including **secp256k1**.
- Run tests using:
  ```sh
  cargo test
## 🛠 Usage Example
```rust
use elliptic_curve::EllipticCurve;
use elliptic_curve::Point;

fn main() {
    let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
    let p1 = Point::Coordinates(BigUint::from(6u32), BigUint::from(3u32));
    let p2 = Point::Coordinates(BigUint::from(5u32), BigUint::from(1u32));
    
    let result = ec.add(&p1, &p2);
    println!("Point Addition Result: {:?}", result);
}
```