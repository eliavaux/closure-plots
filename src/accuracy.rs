use num_traits::Float;

pub enum Accuracy {
	Exact,
	Inexact(f64), // Decimal loss
	Overflow,
	Underflow,
	NotANumber
}

pub fn accuracy<C>(x: C, y: C) -> Accuracy
where
	C: Float + Into<f64>,
{
	use Accuracy::*;

	// Special cases
	if x.is_nan() {
		NotANumber
	} else if x.is_infinite() {
		Overflow
	} else if x == y {
		Exact
	} else if x.is_zero() && !y.is_zero() {
		Underflow
	} else {
		let decimal_loss = (x / y).log10().abs();
		// dbg!(precision);
		Inexact(decimal_loss.into())
	}	
}
