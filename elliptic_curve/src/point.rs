use std::ops::Add;
use finite_field::ff::PrimeField as FF;
use finite_field::ff::FiniteField;
use num_bigint::BigUint;
use num_traits::{Zero, One};

pub trait Point:
    Sized
    + PartialEq
    + Add<Self, Output=Self>
{
    /// create a new point
    fn new(x: Option<FF>, y: Option<FF>, a: FF, b: FF) -> Self;

    /// return x
    fn x(&self) -> FF;

    /// return y
    fn y(&self) -> FF;

    /// return (x, y)
    fn xy(&self) -> (FF, FF);
}

#[derive(Debug, Clone)]
pub struct ECCPoint {
    pub x: Option<FF>,
    pub y: Option<FF>,
    pub a: FF,
    pub b: FF,
}

impl Point for ECCPoint {
    fn new(x: Option<FF>, y: Option<FF>, a:FF, b:FF) -> Self {

        match (x, y) {
            (Some(x), Some(y)) => {
                if y.clone().pow(2) != x.clone().pow(3) + a.clone()*x.clone() + b.clone() {
                    panic!("Not in curve");
                }
                return Self {x: Some(x), y: Some(y), a, b};
            },
            (None, None) => {
                return Self {x: None, y: None, a, b};
            },
            _ => panic!("Invalid point")
            
        }
    }

    fn x(&self) -> FF {
        if self.x.is_none() {
            return self.a.to_zero();
        } 
        self.x.clone().unwrap()
    }

    fn y(&self) -> FF {
        if self.y.is_none() {
            return self.a.to_zero();
        } 
        self.y.clone().unwrap()
    }

    fn xy(&self) -> (FF, FF) {
        (self.x(), self.y())
    }
}

impl ECCPoint {

    /// return true if the point is infinity
    pub fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    /// implement scalar multiplication in elliptic curve
    /// 
    /// input: n
    /// 
    /// output: n*P which P is point in elliptic curve
    pub fn scalar_mul(&self, mut n: BigUint) -> Self {
        let mut q = self.clone();
        let mut r = ECCPoint::new(None, None, q.a.clone(), q.b.clone());
        while n > BigUint::zero() {
            if n.clone() % BigUint::from(2_u32) == BigUint::one() {
                r = r + q.clone();
            }
            q = q.clone() + q.clone();
            n = n.clone() / BigUint::from(2_u32);
        }
        r
    }


    /// return infinity point
    pub fn new_infinity_point(a: FF, b: FF) -> Self {
        Self {
            x: None,
            y: None,
            a,
            b,
        }
    }
}

impl PartialEq for ECCPoint {
    fn eq(&self, other: &ECCPoint) -> bool { 
        self.x == other.x 
        && self.y == other.y 
        && self.a == other.a 
        && self.b == other.b
    }    
}

impl Add for ECCPoint {
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


#[cfg(test)]
mod tests {
    use crate::point::Point;
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
        
        let k = ECCPoint::new(Some(gx.clone()),Some(gy.clone()),a.clone(),b.clone());
        let k1 = ECCPoint::new(Some(gx.clone()),Some(gy.clone()),a.clone(),b.clone());
        assert_eq!(k.clone().y.unwrap().pow(2),k.clone().x.unwrap().pow(3) + k.b.clone());
        assert_eq!(k.clone(), k1);
    }

    #[test]
    fn test_addition() {
        let p = BigUint::from_str_radix("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap();
        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from_u32(7).unwrap(), p.clone());

        let x1 = BigUint::from_str("73286588022960150853141494243436869925915710116679081821764683029732695813278").unwrap();
        let y1 = BigUint::from_str("24078932672756686264184803023496970251662289758831078626496324175980264267340").unwrap();

        let x1 = FF::new(x1, p.clone());
        let y1 = FF::new(y1, p.clone());
        let pp = ECCPoint::new(Some(x1.clone()), Some(y1.clone()), a.clone(), b.clone());

        let x2 = BigUint::from_str("11836935774968833838816391111149481227030000373644152368864624284734962045370").unwrap();
        let y2 = BigUint::from_str("95059496343748871885283713597473300370257361075022743150847197727772278759251").unwrap();

        let x2 = FF::new(x2, p.clone());
        let y2 = FF::new(y2, p.clone());
        let q = Point::new(Some(x2.clone()), Some(y2.clone()), a.clone(), b.clone());


        let x = BigUint::from_str("65091106880770579434025914072026411322435085827839064564163275832463685721657").unwrap();
        let y = BigUint::from_str("30305360825903491312731261345618076499154842379458777456881820539931512892321").unwrap();

        let x = FF::new(x, p.clone());
        let y = FF::new(y, p.clone());
        let rs = ECCPoint::new(Some(x.clone()), Some(y.clone()), a.clone(), b.clone());

        let infinite = ECCPoint::new_infinity_point(a.clone(), b.clone());
        assert_eq!(pp.clone() + infinite, pp.clone());
        assert_eq!(pp.clone() + pp.clone() + q, rs);
    }

    #[test]
    pub fn test_scalar() {
        let p = BigUint::from_str_radix("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap();

        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from_u32(7).unwrap(), p.clone());

        let x1 = BigUint::from_str("73286588022960150853141494243436869925915710116679081821764683029732695813278").unwrap();
        let y1 = BigUint::from_str("24078932672756686264184803023496970251662289758831078626496324175980264267340").unwrap();

        let x1 = FF::new(x1, p.clone());
        let y1 = FF::new(y1, p.clone());
        let pp = ECCPoint::new(Some(x1.clone()), Some(y1.clone()), a.clone(), b.clone());
        let n = BigUint::from(1073741824_u32);
        
        let x = BigUint::from_str("61611953048517811650664679398419437441390089727362306165129633387128161805960").unwrap();
        let y = BigUint::from_str("10250536693719006916088998792381667957869597354334095501142440685140205495795").unwrap();

        let x = FF::new(x, p.clone());
        let y = FF::new(y, p.clone());
        let rs = ECCPoint::new(Some(x.clone()), Some(y.clone()), a.clone(), b.clone());

        assert_eq!(pp.scalar_mul(n), rs);
    }
}