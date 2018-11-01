mod constants;
extern crate chrono;

mod lib {

	use constants::*;
	use chrono::prelude::*;

	
	#[allow(dead_code)]
	#[derive(PartialEq, Debug)]
	pub enum Is<T> {
		Cons(T),
		Nil,
	}

	pub trait IsThirteen<OpType> {
		fn thirteen(self) -> bool; 
		fn roughly(self) -> bool;
		fn returning(self) -> bool;
		fn not(self) -> bool;
		fn divisible(self) -> bool;
		fn square(self) -> bool;
		fn greater(self) -> bool;
		fn less(self) -> bool;
		fn within(self, OpType) -> bool;
		fn year_of_birth(self) -> Is<OpType>;
		fn plus(self,OpType) -> Is<OpType>;
		fn minus(self,OpType) -> Is<OpType>;
		fn times(self,OpType) -> Is<OpType>;
		fn divided_by(self,OpType) -> Is<OpType>;
		fn can_spell(self) -> bool;
		fn anagram_of(self) -> bool;
		fn backwards(self) -> bool;
		fn base(self) -> bool;
	}

	impl IsThirteen<i32> for Is<i32> {
		fn thirteen(self) -> bool {
			match self {
				Is::Cons(literal) => THIRTEEN == literal,
				_ => false
			}
		}
		fn roughly(self) -> bool {
			match self {
				Is::Cons(approx) => {
					let f = approx as f64;
					f >= ((THIRTEEN as f64) - THIRTEEN_FUZZ) && f < ((THIRTEEN as f64) + THIRTEEN_FUZZ)
				},
				_ => false
			}
		}
		fn returning(self) -> bool {
			unreachable!();
		}
		fn not(self) -> bool {
			match self {
				Is::Cons(literal) => {
					literal != THIRTEEN
				},
				_ => true
			}
		}
		fn divisible(self) -> bool {
			match self {
				Is::Cons(literal) => {
					literal % THIRTEEN == 0
				},
				_ => true
			}
		}
		fn square(self) -> bool {
			match self {
				Is::Cons(square) => (THIRTEEN * THIRTEEN) == square,
				_ => false
			}
		}
		fn greater(self) -> bool {
			match self {
				Is::Cons(x) => x > THIRTEEN,
				_ => false
			}
		}
		fn less(self) -> bool {
			match self {
				Is::Cons(x) => x < THIRTEEN,
				_ => false
			}
		}
		fn within(self, other: i32) -> bool {
			match self {
				Is::Cons(x) => x > (THIRTEEN - other) && x  < (THIRTEEN + other),
				_ => false
			}
		}
		fn year_of_birth(self) -> Is<i32> {
			match self {
				Is::Cons(literal) => {
					let local: DateTime<Local> = Local::now();
					Is::Cons(local.year() - literal)
				},
				_ => Is::Cons(0)
			}
		}
		fn plus(self, addend: i32) -> Is<i32> {
			match self {
				Is::Cons(literal) => Is::Cons(literal + addend),
				_ => Is::Cons(0)
			}
		}
		fn minus(self, minuend: i32) -> Is<i32> {
			match self {
				Is::Cons(literal) => Is::Cons(literal - minuend),
				_ => Is::Cons(0)
			}
		}
		fn times(self, multiplier: i32) -> Is<i32> {
			match self {
				Is::Cons(literal) => Is::Cons(literal*multiplier),
				_ => Is::Cons(0)
			}
		}
		fn divided_by(self, divisor: i32) -> Is<i32> {
			match self {
				Is::Cons(literal) => Is::Cons(literal/divisor),
				_ => Is::Cons(0)
			}
		}
		fn can_spell(self) -> bool {
			unimplemented!();
		}
		fn anagram_of(self) -> bool {
			unimplemented!();
		}
		fn backwards(self) -> bool {
			unimplemented!();
		}
		fn base(self) -> bool {
			unimplemented!();
		}
	}
}

#[cfg(test)]
mod tests {
	use lib::*;
	#[test]
	fn is_literally_thirteen() {
		assert_eq!(Is::Cons(13).thirteen(), true);
	}
	#[test]
	fn is_year_of_birth_thirteen() {
		assert_eq!(Is::Cons(2005).year_of_birth().thirteen(), true);
	}
	#[test]
	fn is_square_thirteen() {
		assert_eq!(Is::Cons(169).square(), true);
	}
	#[test]
	fn is_plus_thirteen() {
		assert_eq!(Is::Cons(5).plus(8).thirteen(), true);
	}
	#[test]
	fn is_minus_thirteen() {
		assert_eq!(Is::Cons(20).minus(7).thirteen(), true);
	}
	#[test]
	fn is_times_thirteen() {
		assert_eq!(Is::Cons(13).times(1).thirteen(), true);
	}
	#[test]
	fn is_divided_thirteen() {
		assert_eq!(Is::Cons(26).divided_by(2).thirteen(), true);
	}
	#[test]
	fn is_within_thirteen() {
		assert_eq!(Is::Cons(11).within(20), true);
	}
	#[test]
	fn is_greater_thirteen() {
		assert_eq!(Is::Cons(14).greater(), true);
	}
	#[test]
	fn is_less_thirteen() {
		assert_eq!(Is::Cons(12).less(), true);
	}
	#[test]
	fn is_not_thirteen() {
		assert_eq!(Is::Cons(12).not(), true);
	}
	#[test]
	fn is_divisible_thirteen() {
		assert_eq!(Is::Cons(169).divisible(), true);
	}
	#[test]
	fn is_roughly_thirteen() {
		assert_eq!(Is::Cons(12).roughly(), true);
	}
}
