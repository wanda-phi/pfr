#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct Bcd {
    pub digits: [u8; Self::DIGITS],
}

impl Bcd {
    pub const DIGITS: usize = 12;
    pub const ZERO: Self = Self {
        digits: [0; Self::DIGITS],
    };

    pub fn from_bytes(digits: [u8; Self::DIGITS]) -> Self {
        for x in digits {
            assert!(x < 10);
        }
        Self { digits }
    }

    pub fn from_digit(digit: u8) -> Self {
        Self::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, digit])
    }

    pub fn to_ascii(self) -> [u8; Self::DIGITS] {
        let mut got_digit = false;
        let mut res = self.digits.map(|x| {
            got_digit |= x != 0;
            if got_digit {
                b'0' + x
            } else {
                b' '
            }
        });
        if res[11] == b' ' {
            res[11] = b'0';
        }
        res
    }

    pub fn from_ascii(digits: &[u8]) -> Self {
        assert!(digits.len() <= Self::DIGITS);
        assert!(digits.iter().copied().all(|x| x.is_ascii_digit()));
        Self {
            digits: core::array::from_fn(|i| {
                if i >= Self::DIGITS - digits.len() {
                    digits[i - (Self::DIGITS - digits.len())] - b'0'
                } else {
                    0
                }
            }),
        }
    }

    pub fn leading_zeros(self) -> usize {
        let mut res = 0;
        while self.digits[res] == 0 && res < 12 {
            res += 1;
        }
        res
    }
}

impl core::ops::Add<Bcd> for Bcd {
    type Output = Bcd;

    fn add(self, rhs: Bcd) -> Self::Output {
        let mut res = self;
        let mut carry = 0;
        for i in (0..Bcd::DIGITS).rev() {
            res.digits[i] += rhs.digits[i] + carry;
            carry = if res.digits[i] >= 10 {
                res.digits[i] -= 10;
                1
            } else {
                0
            };
        }
        res
    }
}

impl core::ops::AddAssign<Bcd> for Bcd {
    fn add_assign(&mut self, rhs: Bcd) {
        *self = *self + rhs;
    }
}

impl core::ops::Mul<u8> for Bcd {
    type Output = Bcd;

    fn mul(self, rhs: u8) -> Self::Output {
        assert!(rhs < 10);
        let mut res = Self::ZERO;
        let mut carry = 0;
        for i in (0..Bcd::DIGITS).rev() {
            carry += self.digits[i] * rhs;
            res.digits[i] = carry % 10;
            carry /= 10;
        }
        res
    }
}
