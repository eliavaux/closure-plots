// #![feature(f16)]
// #![feature(f128)]

use image::ExtendedColorType;
use num_traits::Float;
use half::f16;
use std::{fmt::Debug, iter::Iterator};
use softposit::P16E1;

fn main() {
	let resolution = 12;
	let image_res = 1 << resolution;

	// 16-Bit floating points
	let from_bits_f16 = |n: u16| {
		// Floats start at 0, go up to maxReal, then continue with -0 and go down to minReal
		// This orders the floats by size
		let n = if n < (1 << 15) { !n } else { n - (1 << 15) };
		f16::from_bits(n)
	};

	// Using addition
	let operation = |x, y| x + y;
	let operation_precise = |x: f64, y| x + y;

	let buf = closure_plot(
		resolution,
		from_bits_f16,
		operation,
		operation_precise
	);

	let buf = image_buffer(buf);

	let image_res = 1 << resolution;

	image::save_buffer("hi.png", &buf, image_res, image_res, ExtendedColorType::Rgb8)
		.expect("Couldn't save image");
}

enum Accuracy {
	Exact,
	Inexact(f64), // Exact to how many decimals?
	Overflow,
	Underflow,
	NotANumber
}

fn closure_plot<T, C, FB, OP, OPC>(
	resolution: u8,
	from_bits: FB,
	operation: OP,
	operation_precise: OPC
) -> Vec<Accuracy>
where
	T: Float + Send + Sync, // Type we want to gerenate a closure plot for
	C: Float + Send + Sync + From<T> + Into<f64> + Debug, // Second, more accurate type to compare T with
	FB: (Fn(u16) -> T) + Send + Sync,
	OP: (Fn(T, T) -> T) + Send + Sync,
	OPC: (Fn(C, C) -> C) + Send + Sync, // Same operation but with C
{
	use Accuracy::*;

	(0u16..1 << resolution).flat_map(|x| {
		let x = (1 << (16 - resolution)) * x;
		(0u16..1 << resolution).map(|y| {
			let y = (1 << (16 - resolution)) * y;

			let x_t = from_bits(x);
			let y_t = from_bits(y);
			let x_c = <C as From<T>>::from(x_t);
			let y_c = <C as From<T>>::from(y_t);

			let result = operation(x_t, y_t);
			let result = <C as From<T>>::from(result);
			let result_precise = operation_precise(x_c, y_c);

			let precision = -((result / result_precise).abs().log10().abs().log10());
			// dbg!(precision);

			// Special cases
			if result.is_nan() {
				NotANumber
			} else if result.is_infinite() {
				Overflow
			} else if result == result_precise {
				Exact
			} else if result.is_zero() && !result_precise.is_zero() {
				Underflow
			} else {
				Inexact(precision.into())
			}
		}).collect::<Vec<Accuracy>>()
	}).collect()
}

const PALETTE: [[u8; 3]; 5] = [
	[  0,   0,   0], // Exact
	[255,   0, 255], // Inexact
	[255,   0,   0], // Overflow
	[  0,   0, 255], // Underflow
	[255, 255,   0], // Not a number
];


fn image_buffer(buf: Vec<Accuracy>) -> Vec<u8> {
	buf.iter().flat_map(|x| {
		color(x)
	}).collect()
}

fn color(case: &Accuracy) -> [u8; 3] {
	use Accuracy::*;
	match case {
		Exact => PALETTE[0],
		Inexact(error) => {
			let [r, g, b] = PALETTE[1];
			let error_adj = (16.5 - error) / 8.5;
			// dbg!(error, error_adj);

			let r = r as f64 * error_adj;
			let g = g as f64 * error_adj;
			let b = b as f64 * error_adj;

			[r as u8, g as u8, b as u8]
		},
		Overflow => PALETTE[2],
		Underflow => PALETTE[3],
		NotANumber => PALETTE[4],
	}
}
