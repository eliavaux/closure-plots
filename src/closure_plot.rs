use std::fmt::Debug;

use num_traits::Float;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::accuracy::{Accuracy::*, *};


pub fn closure_plot_2d<T, C, FB, OP, OPC>(
	resolution: u32,
	from_bits: FB,
	operation: OP,
	operation_precise: OPC,
) -> Vec<Accuracy>
where
	T: Float + Into<C> + Send + Sync, // Type we want to generate a closure plot for
	C: Float + Into<f64> + Send + Sync + Debug, // Second, more accurate type to compare T with
	FB: (Fn(u16) -> T) + Send + Sync,
	OP: (Fn(T) -> T) + Send + Sync,
	OPC: (Fn(C) -> C) + Send + Sync, // Same operation but with C
{
	(0..=1u16.unbounded_shl(resolution).wrapping_sub(1)).into_par_iter().map(|x| {
		let x = (1 << (16 - resolution)) * x;

		let x_t = from_bits(x);
		let x_c = x_t.into();

		let result = operation(x_t);
		let result_precise = operation_precise(x_c);

		let result: C = result.into();
		accuracy(result, result_precise)
	}).collect()
}


pub fn closure_plot_3d<T, C, FB, OP, OPC>(
	resolution: u32,
	from_bits: FB,
	operation: OP,
	operation_precise: OPC
) -> Vec<Vec<Accuracy>>
where
	T: Float + Into<C> + Send + Sync, // Type we want to generate a closure plot for
	C: Float + Into<f64> + Send + Sync + Debug, // Second, more accurate type to compare T with
	FB: (Fn(u16) -> T) + Send + Sync,
	OP: (Fn(T, T) -> T) + Send + Sync,
	OPC: (Fn(C, C) -> C) + Send + Sync, // Same operation but with C
{
	(0..=1u16.unbounded_shl(resolution).wrapping_sub(1)).into_par_iter().map(|x| {
		let x = (1 << (16 - resolution)) * x;
		(0..=1u16.unbounded_shl(resolution).wrapping_sub(1)).map(|y| {
			let y = (1 << (16 - resolution)) * y;

			let x_t = from_bits(x);
			let y_t = from_bits(y);
			let x_c = x_t.into();
			let y_c = y_t.into();

			let result = operation(x_t, y_t);
			let result_precise = operation_precise(x_c, y_c);

			let result: C = result.into();
			let accuracy = accuracy(result, result_precise);

			match accuracy {
				// Convert decimal loss into decimal accuracy
				Inexact(loss) => Inexact(-loss.log10()),
				_ => accuracy
			}
		}).collect::<Vec<Accuracy>>()
	}).collect()
}

