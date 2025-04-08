use std::ops;
use std::cmp::Ordering;

/// # Integer structure
/// This structure represents numbers like
/// `Vec<u64>` array of digits.
///
/// # Operations
/// ## Done
/// - Ordering
/// - Addition
/// - Subtraction
/// - Multiplication
/// - Division
/// - Remainder
/// ## In plan
/// - Sqrt
/// - Greatest Common Divisor
/// - Logarithm
/// - Modular power
#[derive(Debug, Clone)]
pub struct Int {
	/// Number of digits. First digits must be not zero.
	pub size: usize,
	/// Sign of number:
	///
	/// `false` for positive;
	///
	/// `true` for negative;
	pub sign: bool,
	/// The digits of number. Digits must be `u64`.
	pub value: Vec<u64>
}

/// BASE: limit number, base of my number system.
const BASE: u128 = 2u128.pow(64);
/// 2 ** 64 - 1: useful for subtractions!
const BASE_64: u64 = (BASE - 1) as u64;


fn swap(ord: Ordering) -> Ordering {
	//! Change ordering to opposite.
	match ord {
		Ordering::Less => Ordering::Greater,
		Ordering::Greater => Ordering::Less,
		Ordering::Equal => Ordering::Equal
	}
}
impl ops::Index<usize> for Int {
	// Let use syntax like `self[i]`
	// instead of `self.value[i]`
	type Output = u64;
	fn index(&self, ind: usize) -> &u64 {
		self.value.get(ind).expect("index out of range")
	}
}

impl PartialEq for Int {
	fn eq(&self, other: &Int) -> bool {
		if (self.size != other.size) || (self.sign != other.sign) {
			false
		} else {
			for i in 0..self.size {
				if self[i] != other[i] {
					return false;
				}
			}
			true
		}
	}
}
impl Eq for Int { }

impl Ord for Int {
	fn cmp(&self, other: &Int) -> Ordering {
		if self.sign && other.sign {
			swap(self.abs().cmp(&other.abs())) // -a <=> -b implies a >=< b
		} else if self.sign && !other.sign { Ordering::Less }
		else if !self.sign && other.sign { Ordering::Greater }
		else {
			if self.size != other.size {
				self.size.cmp(&other.size)
			} else {
				for i in 0..self.size {
					if self[i] != other[i] {
						return self[i].cmp(&other[i]);
					}
				}
				Ordering::Equal
			}
		}
	}
}
impl PartialOrd for Int {
	fn partial_cmp(&self, other: &Int) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl ops::Neg for &Int {
	type Output = Int;
	fn neg(self) -> Int {
		Int{ size: self.size, sign: !self.sign, value: self.value.clone() }
	}
}
impl ops::Neg for Int {
	type Output = Int;
	fn neg(self) -> Int {
		-&self
	}
}

impl ops::Shl<u64> for &Int {
	type Output = Int;
	fn shl(self, num: u64) -> Int {
		let mut temp0: u64 = 0;
		let mut arr: Vec<u64> = Vec::new();
		for i in (0..self.size).rev() {
			let mut temp1 = 0;
			let mut a = (self.value[i] as u128) << num;
			if a >= BASE {
				temp1 = (a / BASE) as u64;
				a %= BASE;
			}
			arr.push(a as u64 + temp0);
			temp0 = temp1;
		} if temp0 > 0 {
			arr.push(temp0);
		}
		arr.reverse();
		Int{ size: arr.len(), sign: self.sign, value: arr }
	}
}
impl ops::Shl<u64> for Int {
	type Output = Int;
	fn shl(self, num: u64) -> Int {
		&self << num
	}
}

impl ops::Add for &Int {
	type Output = Int;
	fn add(self, other: &Int) -> Int {
		if self == &Int::zero() {return other.clone();}
		if other == &Int::zero() {return self.clone();}
		if (self.sign == other.sign) & self.sign {
			-((-self) + (-other))
		} else if self.sign != other.sign {
			if self.sign { other - (-self) } else { self - (-other) }
		} else {
			if self.size > other.size {
				let a = &mut other.value.clone();
				let mut b = vec![0u64; self.size - other.size];
				b.append(a);
				let c = Int{ size: self.size, sign: other.sign, value: b };
				return self + c;
			}
			if self.size == other.size {
				let mut temp0: u128 = 0;
				let mut arr: Vec<u64> = Vec::new();
				for i in (1..self.size).rev() {
					let a = self[i] as u128;
					let b = other[i] as u128;
					let mut c = a + b + temp0;
					if c >= BASE {
						c -= BASE;
						temp0 = 1;
					} else { temp0 = 0 }
					arr.push(c as u64);
				}
				let x = self[0] as u128 + other[0] as u128 + temp0;
				if x >= BASE {
					arr.push((x - BASE) as u64);
					arr.push(1)
				} else { arr.push(x as u64); }
				arr.reverse();
				return Int { size: arr.len(), sign: false, value: arr };
			}
			other + self
		}
	}
}
impl ops::Add<&Int> for Int {
	type Output = Int;
	fn add(self, other: &Int) -> Int {
		&self + other
	}
}
impl ops::Add<Int> for &Int {
	type Output = Int;
	fn add(self, other: Int) -> Int {
		self + &other
	}
}
impl ops::Add for Int {
	type Output = Int;
	fn add(self, other: Int) -> Int {
		&self + &other
	}
}

impl ops::Sub for &Int {
	type Output = Int;
	fn sub(self, other: &Int) -> Int {
		if (self.sign == other.sign) & self.sign {
			-((-self) - (-other))
		} else if self.sign != other.sign {
			if self.sign { -(-self + other) } else { self + (-other) }
		} else {
			if self < other {
				return -(other - self);
			} else {
				let arr = self + other.complement() + Int::one();
				let mut new = arr.value;
				new[0] -= 1;
				for i in 0..new.len() {
					if new[i] != 0 {
						return Int { size: new.len()-i, sign: false, value: new[i..new.len()].to_vec() };
					}
				}
				Int::zero()
			}
		}
	}
}
impl ops::Sub<&Int> for Int {
	type Output = Int;
	fn sub(self, other: &Int) -> Int {
		&self - other
	}
}
impl ops::Sub<Int> for &Int {
	type Output = Int;
	fn sub(self, other: Int) -> Int {
		self - &other
	}
}
impl ops::Sub for Int {
	type Output = Int;
	fn sub(self, other: Int) -> Int {
		&self - &other
	}
}

impl ops::Mul<u64> for &Int {
	type Output = Int;
	fn mul(self, num: u64) -> Int {
		let num = num as u128;
		let mut temp0: u64 = 0;
		let mut arr: Vec<u64> = Vec::new();
		for i in (0..self.size).rev() {
			let mut temp1 = 0;
			let mut a = (self.value[i] as u128) * num + temp0 as u128;
			if a >= BASE {
				temp1 = (a / BASE) as u64;
				a %= BASE;
			}
			arr.push(a as u64);
			temp0 = temp1;
		} if temp0 > 0 {
			arr.push(temp0);
		}
		arr.reverse();
		Int{ size: arr.len(), sign: self.sign, value: arr }
	}
}
impl ops::Mul<u64> for Int {
	type Output = Int;
	fn mul(self, num: u64) -> Int {
		&self * num
	}
}

impl ops::Mul for &Int {
	type Output = Int;
	fn mul(self, other: &Int) -> Int {
		if self.sign != other.sign {
			-(self.abs() * other.abs())
		} else if self.sign { self.abs() * other.abs() } else {
			let mut ans = Int{ size: 1, sign: false, value: vec![0]};
			for i in 0..self.size {
				let j = self.size - i - 1;
				ans = ans + (other * self[i]).zeroes_at_right(j);
			} ans.without_leading_zeroes();
			ans
		}
	}
}
impl ops::Mul<&Int> for Int {
	type Output = Int;
	fn mul(self, other: &Int) -> Int {
		&self * other
	}
}
impl ops::Mul<Int> for &Int {
	type Output = Int;
	fn mul(self, other: Int) -> Int {
		self * &other
	}
}
impl ops::Mul for Int {
	type Output = Int;
	fn mul(self, other: Int) -> Int {
		&self * &other
	}
}

impl ops::Rem<u64> for &Int {
	type Output = u64;
	fn rem(self, other: u64) -> u64 {
		if self.size == 1 {
			self.value[0] % other
		} else if self.size == 2 {
			let num = (self.value[0] as u128 * BASE + self.value[1] as u128) % other as u128;
			num as u64
		} else {
			let num1 = Int{size: self.size-1, sign: self.sign, value: self.value[0..self.size-1].to_vec()};
			let num2 = Int{size: 1, sign: self.sign, value: vec![self[self.size-1]]};
			&(num1 * pow(2, 64, other) + num2) % other
		}
	}
}
impl ops::Rem<u64> for Int {
	type Output = u64;
	fn rem(self, other: u64) -> u64 {
		&self % other
	}
}

impl ops::Div<u64> for &Int {
	type Output = Int;
	fn div(self, other: u64) -> Int {
		let num = Int { size: 1, sign: false, value: vec![other] };
		if self < &num { Int::zero() } else {
			if self.size == 1 {
				Int { size: 1, sign: self.sign, value: vec![self[0] / other] }
			} else if self.size == 2 {
				let a = self[0] as u128 * BASE + self[1] as u128;
				let a = a / other as u128;
				if a >= BASE {
					Int { size: 2, sign: self.sign, value: vec![(a / BASE) as u64, (a % BASE) as u64] }
				} else { Int { size: 1, sign: self.sign, value: vec![a as u64] } }
			} else {
				let mut rem = 0u128;
				let mut ans = Vec::new();
				for i in 0..self.size {
					let a = rem * BASE + self[i] as u128;
					let b = a / other as u128;
					rem = a % other as u128;
					ans.push(b as u64);
				}
				if ans[0] > 0 { Int { size: self.size, sign: self.sign, value: ans } } else { Int { size: self.size - 1, sign: self.sign, value: ans[1..].to_vec() } }
			}
		}
	}
}
impl ops::Div<u64> for Int {
	type Output = Int;
	fn div(self, other: u64) -> Int {
		&self / other
	}
}

impl ops::Div for &Int {
	type Output = Int;
	fn div(self, other: &Int) -> Int {
		if other == &Int::zero() {panic!("Division by zero")}
		if self < other { return Int::zero(); }
		let mut left = Int::one();
		let mut right = Int::one().zeroes_at_right(self.size - other.size+1);
		let mut mid;
		while left < right {
			mid = (&left + &right) / 2;
			let result = &(&mid * other);
			if result <= self && self < &(result + other) { return mid; }
			if result > self { right = mid.clone(); }
			else { left = mid.clone(); }
		} left
	}
}
impl ops::Div<&Int> for Int {
	type Output = Int;
	fn div(self, other: &Int) -> Int {
		&self / other
	}
}
impl ops::Div<Int> for &Int {
	type Output = Int;
	fn div(self, other: Int) -> Int {
		self / &other
	}
}
impl ops::Div for Int {
	type Output = Int;
	fn div(self, other: Int) -> Int {
		&self / &other
	}
}

impl ops::Rem for &Int {
	type Output = Int;
	fn rem(self, other: &Int) -> Int {
		let divisor = self / other;
		self - divisor * other
	}
}
impl ops::Rem<&Int> for Int {
	type Output = Int;
	fn rem(self, other: &Int) -> Int {
		&self % other
	}
}
impl ops::Rem<Int> for &Int {
	type Output = Int;
	fn rem(self, other: Int) -> Int {
		self % &other
	}
}
impl ops::Rem for Int {
	type Output = Int;
	fn rem(self, other: Int) -> Int {
		&self % &other
	}
}

impl Int {
	pub fn zero() -> Int {
		//! Return `0` as `Int`.
		Int{ size: 0, sign: false, value: Vec::new()}
	}
	pub fn one() -> Int {
		//! Return `1` as `Int`.
		Int{ size: 1, sign: false, value: vec![1]}
	}
	pub fn base() -> Int {
		//! Return `BASE` in `Int` form.
		Int{ size: 2, sign: false, value: vec![1, 0]}
	}
	fn without_leading_zeroes(&mut self) {
		//! Make sure that number don't start with zero's.
		let mut i = 0;
		while i < self.size && self[i] == 0 { i += 1 }
		self.size -= i;
		self.value = self.value[i..].to_vec();
	}
	fn complement(&self) -> Int {
		//! Make digit-by-digit subtraction.
		//!
		//! Quite useful for subtraction.
		if self == &Int::zero() { return Int::base() - Int::one(); }
		let mut arr: Vec<u64> = Vec::new();
		for i in 0..self.size {
			arr.push(BASE_64 - self.value[i])
		}
		Int { size: self.size, sign: self.sign, value: arr }
	}
	fn zeroes_at_right(&self, num: usize) -> Int {
		//! Write `num` zero's at right sight of number.
		let mut arr = vec![0u64; num];
		let mut ans = self.value.clone();
		ans.append(&mut arr);
		Int { size: self.size + num, sign: self.sign, value: ans }
	}
	pub fn abs(&self) -> Int {
		//! Return positive copy of `Int`.
		Int{ size: self.size, sign: false, value: self.value.clone() }
	}
	pub fn isqrt(&self) -> Int {
		if self == &Int::zero() { return Int::zero(); }
		let mut left = Int::one();
		let mut right = self.clone();
		while left < right {
			let mid = (&left + &right) / 2;
			if &(&mid * &mid) == self { return mid; }
			if &(&mid * &mid) > self { right = mid }
			else { left = mid}
		} left
	}
	pub fn pow(self: Int, exp: Int) -> Int {
		let mut exp = exp.clone();
		let mut result = Int::one();
		let mut n = self.clone();
		while exp > Int::zero() {
			if &exp % 2 == 1 {
				result = result * &n;
			}
			exp = exp / 2;
			n = &n * &n;
		}
		result
	}
	pub fn mod_pow(self: Int, exp: Int, base: &Int) -> Int {
		let mut exp = exp.clone();
		if base == &Int::one() { return Int::zero(); }
		let mut result = Int::one();
		let mut n = &self % base;
		while exp > Int::zero() {
			if &exp % 2 == 1 {
				result = result * &n % base;
			}
			exp = exp / 2;
			n = &n * &n % base;
		}
		result
	}
	pub fn from_str(string: &str) -> Int {
		//! Return an `Int` from to a given string.
		//! # Examples
		//! ```
		//! # use integer::Int;
		//!
		//! let a = Int::from_str("175");
		//! let b = Int{ size: 1, sign: false, value: vec![175] };
		//! assert_eq!(a, b);
		//!
		//! let a = Int::from_str("263130836933693530167218012160000000"); // factorial of 32
		//! let arr = vec![14264351252571976, 12400865694432886784];
		//! let b = Int{ size: 2, sign: false, value: arr };
		//! assert_eq!(a, b);
		//!
		//! let a = Int::from_str("-175");
		//! let b = Int{ size: 1, sign: true, value: vec![175] };
		//! assert_eq!(a, b);
		//! ```
		let n = string.len();
		if n == 0 { return Int::zero() }
		if string.chars().nth(0).unwrap() == '-' { return -Int::from_str(&string[1..]) }
		let mut ans = Int{ size: 1, sign: false, value: vec![0] };
		let mut base = Int{ size: 1, sign: false, value: vec![1] };
		let ten = Int{ size: 1, sign: false, value: vec![10] };
		for i in (0..n).rev() {
			let digit = string.chars().nth(i).unwrap().to_digit(10).unwrap();
			ans = ans + &base * digit as u64;
			base = base * &ten;
		} ans.without_leading_zeroes();
		ans
	}
	pub fn to_str(&self) -> String {
		//! Return string from `Int` - result written in decimal.
		//! ```
		//! # use integer::Int;
		//!
		//! let num = Int{ size: 1, sign: false, value: vec![175] };
		//! assert_eq!(num.to_str(), "175");
		//!
		//! let arr = vec![14264351252571976, 12400865694432886784];
		//! let num = Int{ size: 2, sign: false, value: arr }; // factorial of 32
		//! assert_eq!(num.to_str(), "263130836933693530167218012160000000");
		//!
		//! let num = Int{ size: 1, sign: true, value: vec![175] };
		//! assert_eq!(num.to_str(), "-175");
		//! ```
		if self < &Int::zero() { return "-".to_owned() + &(-self).to_str() }
		if self == &Int::zero() { return String::from("0") }
		let mut arr = Vec::new();
		let mut num = self.clone();
		while num > Int::zero() {
			let digit = &num % 10;
			arr.push(digit.to_string());
			num = num / 10;
		} arr.reverse();
		let mut ans = String::new();
		for elem in arr {
			ans.push_str(elem.to_string().as_str());
		} ans
	}
}

fn pow(mut n: u64, mut power: u64, base: u64) -> u64 {
	if base == 1 { return 0 }
	let mut result = 1;
	n %= base;
	while power > 0 {
		if power % 2 == 1 {
			result = result * n % base;
		}
		power = power >> 1;
		n = (n as u128 * n as u128 % base as u128) as u64;
	}
	result
}
