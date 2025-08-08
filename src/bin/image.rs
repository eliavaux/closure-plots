// #![feature(f16)]
// #![feature(f128)]

use image::Rgb;
use num_traits::Float;
use rayon::iter::ParallelIterator;
use half::f16;

fn main() {
	// compare_f16_with_f64();
	// compare_p16es1_with_f64();
	// compare_with_astrofloat();


	/* let res = 10;
	for i in 0..(1 << res) {
		let n = (1 << (16 - res)) * i;
		let n_adj = if n < (1 << 15) { !n } else { n - (1 << 15) };
		
		let n_float = f16::from_bits(n_adj);
		// println!("{}\t{}", n, n_float);
		println!("{}\t{}\t{}", n, n_adj, n_float);
	} */

	let from_bits_f16 = |n: u16| {
		// Floats start at 0, go up to maxReal, then continue with -0 and go down to minReal
		// This orders the floats by size
		let n = if n < (1 << 15) { !n } else { n - (1 << 15) };
		f16::from_bits(n)
	};

	let operation = |x, y| x + y;
	let operation_precise = |x: f64, y| x + y;

	closure_plot(
		"hi.png",
		8.5, 16.5,
		1024,
		from_bits_f16,
		operation,
		operation_precise
	);
}

enum FloatCase {
	Exact,
	Inexact,
	Overflow,
	Underflow,
	NotANumber
}

fn closure_plot<T, C, FB, OP, OPC>(
	filename: &str,
	min: f64,
	max: f64,
	resolution: u16,
	from_bits: FB,
	operation: OP,
	operation_precise: OPC
)
where
	T: Float, // Type we want to gerenate a closure plot for
	C: Float + From<T> + Into<f64>, // Second, more accurate type to compare T with
	FB: (Fn(u16) -> T) + Send + Sync,
	OP: (Fn(T, T) -> T) + Send + Sync,
	OPC: (Fn(C, C) -> C) + Send + Sync, // Same operation but with C
{
	let mut image_buf = image::ImageBuffer::new(resolution as u32, resolution as u32);

	image_buf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
		let x = x as u16;
		let y = y as u16;

		let x_t = from_bits((u16::MAX / resolution + 1) * x);
		let y_t = from_bits((u16::MAX / resolution + 1) * y);
		let x_c = <C as From<T>>::from(x_t);
		let y_c = <C as From<T>>::from(y_t);

		let result = operation(x_t, y_t);
		let result_precise = operation_precise(x_c, y_c);

		let precision = -((<C as From<T>>::from(result) / result_precise).abs().log10().abs().log10());
		let a: f64 = precision.into();
		let precision_adjusted = (max - a) / min;

		let r = 238.0 * precision_adjusted;
		let g = 225.0 * precision_adjusted;
		let b = 138.0 * precision_adjusted;

		// dbg!(r, g, b);

		*pixel = image::Rgb([r as u8, g as u8, b as u8]);
	});
	dbg!();

	image_buf.save(filename).unwrap();
}

/* fn compare_f16_with_f64() {
	let width = 1024 * 4;
	let height = width;

	let mut precision_min = f64::MAX; 
	let mut precision_max = f64::MIN_POSITIVE;

	let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

	for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
		let x = x as u16;
		let y = y as u16;

		// if x == width/2 {
		// 	let x = if x < width / 2 { width - x } else { x };
		// 	dbg!(x);
		// 	let x = f32::from_bits((u32::MAX / width) * x);
		// 	dbg!(x);
		// }

		// println!("x:\t{x}");
		// println!("y:\t{y}");

		// Floats start at 0, go up to maxReal, then continue with -0 and go down to minReal
		// This orders the floats by size
		let x = if x < width / 2 { width - x - 1} else { x };
		let y = if y < width / 2 { width - y - 1} else { y };

		let x = f16::from_bits((u16::MAX / width + 1) * x);
		let y = f16::from_bits((u16::MAX / height + 1) * y);

		// println!("x:\t{x}");
		// println!("y:\t{y}");

		if x.is_nan() || y.is_nan() { // NaN
			*pixel = Rgb([132, 142, 232]);
		} else {
			let res = x / y;
			let res_prec = x as f64 / y as f64;

			// if false {
			if res.is_infinite() { // Overflow
				*pixel = Rgb([229, 122, 119]);
			} else {
				if res == 0.0 && x != 0.0 && y != 0.0 { // Underflow
					*pixel = Rgb([165, 241, 154]);
				} else if res as f64 == res_prec { // Exact
					*pixel = Rgb([0, 0, 0]);
				} else {
					let precision = -((res as f64/res_prec).abs().log10().abs().log10());

					// if precision.is_nan() {
					/* if precision.is_infinite() || precision.is_nan() {
						println!("vvvvvvvvvvv");
						dbg!(x, y);
						dbg!(res as f64);
						dbg!(res_prec);
						dbg!(res as f64/res_prec);
						dbg!((res as f64/res_prec).log10());
						println!("^^^^^^^^^^^");
					} */
					// println!("{}", precision as f64);
					// println!("{}", x as f64);
					precision_min = precision_min.min(precision);
					precision_max = precision_max.max(precision);

					let precision_adjusted = (16.5 - precision) / 8.5;
					let r = 238.0 * precision_adjusted;
					let g = 225.0 * precision_adjusted;
					let b = 138.0 * precision_adjusted;

					// dbg!(r, g, b);

					*pixel = image::Rgb([r as u8, g as u8, b as u8]);
				}
			}
		}
	}
	dbg!(precision_min, precision_max);

	imgbuf.save("closure_plot_f16_multiplication.png").unwrap();
} */

/* fn compare_p16es1_with_f64() {
	let width = 1024 * 4;
	let height = width;

	let mut precision_min = f64::MAX; 
	let mut precision_max = f64::MIN_POSITIVE;

	let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

	for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
		let x = x as u16;
		let y = y as u16;

		// Floats start at 0, go up to maxReal, then continue with -0 and go down to minReal
		// This orders the floats by size
		let x = if x < width / 2 { width - x - 1} else { x };
		let y = if y < width / 2 { width - y - 1} else { y };

		let x = P16E1::from_bits((u16::MAX / width + 1) * x);
		let y = P16E1::from_bits((u16::MAX / height + 1) * y);

		// println!("x:\t{x}");
		// println!("y:\t{y}");

		if x.is_nan() || y.is_nan() { // NaN
			*pixel = Rgb([132, 142, 232]);
		} else {
			let res = x * y;
			let res_prec = f64::from(x) * f64::from(y);

			// if false {
			if res.is_infinite() { // Overflow
				*pixel = Rgb([229, 122, 119]);
			} else {
				if res == P16E1::ZERO && (x != P16E1::ZERO && y != P16E1::ZERO) { // Underflow
					*pixel = Rgb([165, 241, 154]);
				} else if f64::from(res) == res_prec { // Exact
					*pixel = Rgb([0, 0, 0]);
				} else {
					let precision = -((f64::from(res)/res_prec).abs().log10().abs().log10());

					// if precision.is_nan() {
					/* if precision.is_infinite() || precision.is_nan() {
						println!("vvvvvvvvvvv");
						dbg!(x, y);
						dbg!(res as f64);
						dbg!(res_prec);
						dbg!(res as f64/res_prec);
						dbg!((res as f64/res_prec).log10());
						println!("^^^^^^^^^^^");
					} */
					// println!("{}", precision as f64);
					// println!("{}", x as f64);
					precision_min = precision_min.min(precision);
					precision_max = precision_max.max(precision);

					let precision_adjusted = (16.5 - precision) / 8.5;
					let r = 238.0 * precision_adjusted;
					let g = 225.0 * precision_adjusted;
					let b = 138.0 * precision_adjusted;

					// dbg!(r, g, b);

					*pixel = image::Rgb([r as u8, g as u8, b as u8]);
				}
			}
		}
	}
	dbg!(precision_min, precision_max);

	imgbuf.save("closure_plot_p16es1_multiplication.png").unwrap();
} */
