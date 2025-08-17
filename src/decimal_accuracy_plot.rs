use num_traits::Float;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::accuracy::*;


pub fn decimal_accuracy_plot<T, FB>(from_bits: FB) -> Vec<Accuracy>
where
	T: Float + Into<f64> + Send + Sync,
	FB: (Fn(u16) -> T) + Send + Sync
{
	use Accuracy::*;

	(0..u16::MAX).into_par_iter().map(|x| {
		let succ = x + 1;

		let x_t = from_bits(x);
		let succ_t = from_bits(succ);
		let x_f64: f64 = x_t.into();
		let succ_f64: f64 = succ_t.into();
		
		let accuracy = accuracy(x_f64, succ_f64);

		match accuracy {
			// Convert decimal loss into decimal accuracy
			Inexact(loss) => Inexact(-loss.log10()),
			_ => accuracy
		}
	}).collect()
}

