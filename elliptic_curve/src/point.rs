use finite_field::ff::FiniteField;
use finite_field::prime_field::PrimeField as FF;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::Add;

pub type PointData = (Option<FF>, Option<FF>, FF, FF);

/// Trait for point in elliptic curve
pub trait Point:
    Sized + PartialEq + Add<Self, Output = Self> + for<'a> Add<&'a Self, Output = Self>
{
    /// create a new point
    fn new(point: &PointData) -> Self;

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
    fn new(point: &PointData) -> Self {
        match point {
            (Some(x), Some(y), a, b) => {
                if y.pow(2) != x.pow(3) + (a.clone() * x) + b {
                    panic!("Not in curve");
                }
                Self {
                    x: Some(x.clone()),
                    y: Some(y.clone()),
                    a: a.clone(),
                    b: b.clone(),
                }
            }
            (None, None, a, b) => Self {
                x: None,
                y: None,
                a: a.clone(),
                b: b.clone(),
            },
            _ => panic!("Invalid point"),
        }
    }

    /// return x
    fn x(&self) -> FF {
        if self.x.is_none() {
            return self.a.to_zero();
        }
        self.x.clone().unwrap()
    }

    /// return y
    fn y(&self) -> FF {
        if self.y.is_none() {
            return self.a.to_zero();
        }
        self.y.clone().unwrap()
    }

    /// return (x, y)
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

        let mut r = ECCPoint::new(&(None, None, q.a.clone(), q.b.clone()));
        while n > BigUint::zero() {
            if n.clone() % BigUint::from(2_u32) == BigUint::one() {
                r = r + q.clone();
            }
            q = q.clone() + q.clone();
            n = &n / BigUint::from(2_u32);
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
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

impl Add for ECCPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl<'a> Add<&'a Self> for ECCPoint {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("not in the same curve")
        }

        match (&self.x, &self.y, rhs.x.as_ref(), rhs.y.as_ref()) {
            (None, None, _, _) => rhs.clone(),
            (_, _, None, None) => self,
            (Some(x1), Some(y1), Some(x2), Some(y2)) if x1 == x2 && y1 != y2 => Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            },
            (Some(x1), Some(y1), Some(_), Some(_)) if &self == rhs => {
                let prime = &self.a.prime;
                let big3 = FF::new(BigUint::from(3_u32), prime.clone());
                let big2 = FF::new(BigUint::from(2_u32), prime.clone());

                let doub_y1 = big2 * y1;
                let sqrt_x1 = x1.pow(2);

                let a = &self.a;
                let lamda = (big3 * sqrt_x1 + a) / (doub_y1);

                let x3 = lamda.pow(2) - x1 - x1;
                let y3 = lamda * (x1.clone() - &x3) - y1;

                Self {
                    x: Some(x3),
                    y: Some(y3),
                    a: self.a.clone(),
                    b: self.b.clone(),
                }
            }
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                let t1 = y2.clone() - y1;
                let t2 = x2.clone() - x1;

                let lamda = t1 / t2;
                let x3 = lamda.pow(2) - x1 - x2;
                let y3 = lamda * (x1.clone() - &x3) - y1;
                Self {
                    x: Some(x3),
                    y: Some(y3),
                    a: self.a.clone(),
                    b: self.b.clone(),
                }
            }
            _ => panic!("Invalid point"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use num_bigint::BigUint;
    use num_traits::{FromPrimitive, Num, Zero};
    use std::str::FromStr;

    #[test]
    fn test_create_point() {
        let x = "8837213008c5d8a75b57fa57caf34709bd5ce952e803a14f801c4d0cd3e7a688";
        let y = "16a99921859b01771522bd63d463508217b9e0b3cd2d85a657202e644f55e5f3";

        let x = BigUint::from_str_radix(x, 16).unwrap();
        let y = BigUint::from_str_radix(y, 16).unwrap();

        let p = BigUint::from_str_radix(
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
            16,
        )
        .unwrap();
        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from(7_u32), p.clone());

        let gx = FF::new(x, p.clone());
        let gy = FF::new(y, p.clone());

        let k = ECCPoint::new(&(Some(gx.clone()), Some(gy.clone()), a.clone(), b.clone()));
        let k1 = ECCPoint::new(&(Some(gx), Some(gy), a, b));
        assert_eq!(
            k.clone().y.unwrap().pow(2),
            k.clone().x.unwrap().pow(3) + &k.b
        );

        assert_eq!(k.clone(), k1);
    }

    #[test]
    fn test_addition() {
        let p = BigUint::from_str_radix(
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
            16,
        )
        .unwrap();
        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from_u32(7).unwrap(), p.clone());

        let x1 = BigUint::from_str(
            "73286588022960150853141494243436869925915710116679081821764683029732695813278",
        )
        .unwrap();
        let y1 = BigUint::from_str(
            "24078932672756686264184803023496970251662289758831078626496324175980264267340",
        )
        .unwrap();

        let x1 = FF::new(x1, p.clone());
        let y1 = FF::new(y1, p.clone());
        let point: PointData = (Some(x1), Some(y1), a.clone(), b.clone());
        let pp = ECCPoint::new(&point);

        let x2 = BigUint::from_str(
            "11836935774968833838816391111149481227030000373644152368864624284734962045370",
        )
        .unwrap();
        let y2 = BigUint::from_str(
            "95059496343748871885283713597473300370257361075022743150847197727772278759251",
        )
        .unwrap();

        let x2 = FF::new(x2, p.clone());
        let y2 = FF::new(y2, p.clone());
        let q_point: PointData = (Some(x2), Some(y2), a.clone(), b.clone());
        let q = ECCPoint::new(&q_point);

        let x = BigUint::from_str(
            "65091106880770579434025914072026411322435085827839064564163275832463685721657",
        )
        .unwrap();
        let y = BigUint::from_str(
            "30305360825903491312731261345618076499154842379458777456881820539931512892321",
        )
        .unwrap();

        let x = FF::new(x, p.clone());
        let y = FF::new(y, p.clone());

        let rs: PointData = (Some(x), Some(y), a.clone(), b.clone());
        let rs = ECCPoint::new(&rs);

        let infinite = ECCPoint::new_infinity_point(a.clone(), b.clone());
        assert_eq!(pp.clone() + &infinite, pp.clone());
        assert_eq!(pp.clone() + &pp.clone() + q.clone(), rs);
        assert_eq!(pp.clone() + infinite, pp.clone());
        assert_eq!(pp.clone() + pp + q, rs);
    }

    #[test]
    pub fn test_scalar() {
        let p = BigUint::from_str_radix(
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
            16,
        )
        .unwrap();

        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from_u32(7).unwrap(), p.clone());

        let x1 = BigUint::from_str(
            "73286588022960150853141494243436869925915710116679081821764683029732695813278",
        )
        .unwrap();
        let y1 = BigUint::from_str(
            "24078932672756686264184803023496970251662289758831078626496324175980264267340",
        )
        .unwrap();

        let x1 = FF::new(x1, p.clone());
        let y1 = FF::new(y1, p.clone());

        let point: PointData = (Some(x1), Some(y1), a.clone(), b.clone());
        let pp = ECCPoint::new(&point);

        let n = BigUint::from(1073741824_u32);

        let x = BigUint::from_str(
            "61611953048517811650664679398419437441390089727362306165129633387128161805960",
        )
        .unwrap();
        let y = BigUint::from_str(
            "10250536693719006916088998792381667957869597354334095501142440685140205495795",
        )
        .unwrap();

        let x = FF::new(x, p.clone());
        let y = FF::new(y, p);
        let rs: PointData = (Some(x), Some(y), a, b);
        let rs = ECCPoint::new(&rs);

        assert_eq!(pp.scalar_mul(n), rs);
    }
}
