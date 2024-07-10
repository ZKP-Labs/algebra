use std::ops::Add;
use finite_field::FiniteField as FF;
use num_bigint::BigUint;
use num_traits::{Num, Zero};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Option<FF>,
    pub y: Option<FF>,
    pub a: FF,
    pub b: FF,
}

impl Point {
    pub fn new(x: Option<FF>, y: Option<FF>, a:FF, b:FF) -> Self {
        if x.is_none() && y.is_none() {
            return Self {x, y, a, b};
        } 
        if y.clone().unwrap().pow(2) != x.clone().unwrap().pow(3) 
                                            + a.clone()*x.clone().unwrap().clone() 
                                            + b.clone() {
            panic!("Not in curve");
        }   
        return Self {x , y, a, b};
    }

    pub fn scalar_mul(&self, mut n: u32) -> Self {
        let mut q = self.clone();
        let mut r = Point::new(None, None, q.a.clone(), q.b.clone());
        while n > 0 {
            if n % 2 == 1 {
                r = r + q.clone();
            }
            q = q.clone() + q.clone();
            n = n / 2;
        }
        r
    }

    pub fn infinity_point(a: FF, b: FF) -> Self {
        Self {
            x: None,
            y: None,
            a,
            b,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool { 
        self.x == other.x 
        && self.y == other.y 
        && self.a == other.a 
        && self.b == other.b
    }    
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("not in the same curve")
        }

        if self.x.is_none() && self.y.is_none() {
            return rhs;
        }

        if rhs.x.is_none() && rhs.y.is_none() {
            return self;
        }

        if self.x == rhs.x && self.y != rhs.y {
            return Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            };
        }

        let (x1, y1, x2, y2) = (
            self.x.clone().unwrap(), 
            self.y.clone().unwrap(), 
            rhs.x.clone().unwrap(), 
            rhs.y.clone().unwrap());

        if self == rhs {
            let prime = self.a.prime.clone();
            let big3 = FF::new(BigUint::from(3_u32), prime.clone());
            let big2 = FF::new(BigUint::from(2_u32), prime.clone());
            let doub_y1 = big2 * y1.clone();
            let sqrt_x1 = x1.clone().pow(2);
            let a = self.a.clone();
            let lamda = (big3*sqrt_x1 + a) / (doub_y1); 
            let x3 = lamda.clone().pow(2) - x1.clone() - x1.clone();    
            let y3 = lamda.clone() * (x1.clone() - x3.clone()) - y1.clone();

            return Self {
                x: Some(x3),
                y: Some(y3),
                a: self.a.clone(),
                b: self.b.clone()
            };  
        } 

        let t1 = y2.clone() - y1.clone();
        let t2 = x2.clone() - x1.clone();
        let lamda = t1 / t2;
        let x3 = lamda.clone().pow(2) - x1.clone() - x2.clone();    
        let y3 = lamda.clone() * (x1.clone() - x3.clone()) - y1.clone();
        return Self {
            x: Some(x3),
            y: Some(y3),
            a: self.a.clone(),
            b: self.b.clone()
            };
        }
}

pub struct Secp256k1 {
    pub a: FF,
    pub b: FF,
    pub p: BigUint,
    pub g: Point,
    pub n: BigUint,
}

impl Secp256k1 {
    pub fn new() -> Self {
        let p = BigUint::from_str_radix("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap();
        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from(7_u32), p.clone());

        let x = BigUint::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
        let y = BigUint::from_str_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();
        let x = FF::new(x, p.clone());
        let y = FF::new(y, p.clone());
        let g = Point::new(Some(x), Some(y), a.clone(), b.clone());

        let n = BigUint::from_str_radix("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();
        Self {a, b, p, g, n}
    }

    pub fn g(&self) -> &Point {
        &self.g
    }

    pub fn n(&self) -> &BigUint {
        &self.n
    }

    pub fn point(&self, x: BigUint, y: BigUint) -> Point {
        Point::new(Some(FF::new(x, self.p.clone())), Some(FF::new(y, self.p.clone())), self.a.clone(), self.b.clone())
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use num_bigint::BigUint;
    use num_traits::{FromPrimitive, Num, Zero};
    use super::*;

    #[test]
    fn test_create_point() {
        let x = "8837213008c5d8a75b57fa57caf34709bd5ce952e803a14f801c4d0cd3e7a688";
        let y = "16a99921859b01771522bd63d463508217b9e0b3cd2d85a657202e644f55e5f3";
    
        let x = BigUint::from_str_radix(x, 16).unwrap();
        let y = BigUint::from_str_radix(y, 16).unwrap();
    
        let p = BigUint::from_str_radix("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap();
        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from(7_u32), p.clone());

        let gx = FF::new(x, p.clone());
        let gy = FF::new(y, p.clone());
        

        let k = Point::new(Some(gx.clone()),Some(gy.clone()),a.clone(),b.clone());
        let k1 = Point::new(Some(gx.clone()),Some(gy.clone()),a.clone(),b.clone());
        assert_eq!(k.clone().y.unwrap().pow(2),k.clone().x.unwrap().pow(3) + k.b.clone());
        assert_eq!(k.clone(), k1);
    }


    #[test]
    fn test_addition() {
        let p = BigUint::from_u8(2).unwrap().pow(256_u32)
            - BigUint::from_u8(2).unwrap().pow(32_u32)
            - BigUint::from_u32(977).unwrap();
        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from_u32(7).unwrap(), p.clone());

        let x1 = BigUint::from_str("73286588022960150853141494243436869925915710116679081821764683029732695813278").unwrap();
        let y1 = BigUint::from_str("24078932672756686264184803023496970251662289758831078626496324175980264267340").unwrap();

        let x1 = FF::new(x1, p.clone());
        let y1 = FF::new(y1, p.clone());
        let pp = Point::new(Some(x1.clone()), Some(y1.clone()), a.clone(), b.clone());

        let x2 = BigUint::from_str("11836935774968833838816391111149481227030000373644152368864624284734962045370").unwrap();
        let y2 = BigUint::from_str("95059496343748871885283713597473300370257361075022743150847197727772278759251").unwrap();

        let x2 = FF::new(x2, p.clone());
        let y2 = FF::new(y2, p.clone());
        let q = Point::new(Some(x2.clone()), Some(y2.clone()), a.clone(), b.clone());


        let x = BigUint::from_str("65091106880770579434025914072026411322435085827839064564163275832463685721657").unwrap();
        let y = BigUint::from_str("30305360825903491312731261345618076499154842379458777456881820539931512892321").unwrap();

        let x = FF::new(x, p.clone());
        let y = FF::new(y, p.clone());
        let rs = Point::new(Some(x.clone()), Some(y.clone()), a.clone(), b.clone());

        let infinite = Point::infinity_point(a.clone(), b.clone());
        assert_eq!(pp.clone() + infinite, pp.clone());
        assert_eq!(pp.clone() + pp.clone() + q, rs);
    }

    #[test]
    pub fn test_scalar() {
        let p = BigUint::from_u8(2).unwrap().pow(256_u32)
            - BigUint::from_u8(2).unwrap().pow(32_u32)
            - BigUint::from_u32(977).unwrap();
        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from_u32(7).unwrap(), p.clone());

        let x1 = BigUint::from_str("73286588022960150853141494243436869925915710116679081821764683029732695813278").unwrap();
        let y1 = BigUint::from_str("24078932672756686264184803023496970251662289758831078626496324175980264267340").unwrap();

        let x1 = FF::new(x1, p.clone());
        let y1 = FF::new(y1, p.clone());
        let pp = Point::new(Some(x1.clone()), Some(y1.clone()), a.clone(), b.clone());
        let n = 1073741824;
        
        let x = BigUint::from_str("61611953048517811650664679398419437441390089727362306165129633387128161805960").unwrap();
        let y = BigUint::from_str("10250536693719006916088998792381667957869597354334095501142440685140205495795").unwrap();

        let x = FF::new(x, p.clone());
        let y = FF::new(y, p.clone());
        let rs = Point::new(Some(x.clone()), Some(y.clone()), a.clone(), b.clone());

        assert_eq!(pp.scalar_mul(n), rs);
    }

    #[test]
    pub fn test_secp256k1() {
        let secp256k1 = Secp256k1::new();
        let x = BigUint::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
        let y = BigUint::from_str_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();
        let g = secp256k1.point(x, y);
        assert_eq!(secp256k1.g(), &g);
        assert_eq!(secp256k1.g.scalar_mul(100) + g.clone(), g.clone().scalar_mul(101));
    }
    
}