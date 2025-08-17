#![allow(unused)]

use std::fs::File;
use std::io::Write;
use std::iter::Iterator;

use closure_plots::{accuracy::*, closure_plot::*, decimal_accuracy_plot::*};

use image::ExtendedColorType;
use num_traits::Float;
use half::f16;
use softposit::P16E1;



fn from_bits_f16(n: u16) -> f16 {
	// Floats start at 0, go up to maxReal, then continue with -0 and go down to minReal
	// This orders the floats by size
	let n = if n < (1 << 15) { !n } else { n - (1 << 15) };
	f16::from_bits(n)
}

fn from_bits_p16(n: u16) -> P16E1 {
	// Posits start at 0, go up to maxReal, then continue with minReal and go up
	// This orders the posits by size
	let n = n.wrapping_add(1 << 15);
	P16E1::from_bits(n)
}


// Change these
fn operation_2d<T: Float>(x: T,) -> T {
	x * x
}

fn operation_3d<T: Float>(x: T, y: T) -> T {
	x * y
}



fn main() {
	let resolution = 12;
	let file_name = "multiplication";

	// generate_decimal_accuracy_plot(resolution, file_name);
	// generate_plots_2d(resolution, file_name);
	generate_plots_3d(resolution, file_name);
}



fn generate_plots_3d(resolution: u32, file_name: &str) {
	let image_res = 1 << resolution;

	let buf = closure_plot_3d(
		resolution,
		from_bits_f16,
		operation_3d,
		operation_3d::<f64>
	);

	let buf = parse_data_3d(buf);

	image::save_buffer(format!("target/{file_name}_f16.png"), &buf, image_res, image_res, ExtendedColorType::Rgb8)
		.expect("Couldn't save image");

	let buf = closure_plot_3d(
		resolution,
		from_bits_p16,
		operation_3d,
		operation_3d::<f64>
	);

	let buf = parse_data_3d(buf);

	image::save_buffer(format!("target/{file_name}_p16.png"), &buf, image_res, image_res, ExtendedColorType::Rgb8)
		.expect("Couldn't save image");
}

fn generate_plots_2d(resolution: u32, file_name: &str) {
	let buf_f16 = closure_plot_2d(
		resolution,
		from_bits_f16,
		operation_2d,
		operation_2d::<f64>
	);
	dbg!(&buf_f16.len());

	let buf_p16 = closure_plot_2d(
		resolution,
		from_bits_p16,
		operation_2d,
		operation_2d::<f64>
	);
	dbg!(&buf_p16.len());

	let mut f16 = parse_data_2d(&buf_f16);
	let mut p16 = parse_data_2d(&buf_p16);
	f16.sort_by(f64::total_cmp);
	p16.sort_by(f64::total_cmp);

	let mut file = File::create(format!("target/{file_name}.csv")).unwrap();

	assert_eq!(f16.len(), p16.len());
	let text: String = (0..buf_p16.len()).map(|i| {
		format!("{},{}\n", f16[i], p16[i])
	}).collect();

	file.write_all("floats,posits\n".as_bytes()).unwrap();
	file.write_all(text.as_bytes()).unwrap();
}

fn generate_decimal_accuracy_plot(file_name: &str) {
	let buf_f16 = decimal_accuracy_plot(from_bits_f16);
	let buf_p16 = decimal_accuracy_plot(from_bits_p16);

	let buf_f16 = parse_data_2d(&buf_f16);
	let buf_p16 = parse_data_2d(&buf_p16);

	let mut file_f16 = File::create(format!("target/{file_name}_f16.csv")).unwrap();
	let mut file_p16 = File::create(format!("target/{file_name}_p16.csv")).unwrap();

	let text_f16: String = (0..u16::MAX).map(|i| {
		format!("{},{}\n", from_bits_f16(i).log2(), buf_f16[i as usize])
	}).collect();

	let text_p16: String = (0..u16::MAX).map(|i| {
		format!("{},{}\n", from_bits_p16(i).log2(), buf_p16[i as usize])
	}).collect();

	file_f16.write_all("log2(x),floats\n".as_bytes()).unwrap();
	file_f16.write_all(text_f16.as_bytes()).unwrap();

	file_p16.write_all("log2(x),posits\n".as_bytes()).unwrap();
	file_p16.write_all(text_p16.as_bytes()).unwrap();
}



fn parse_data_3d(buf: Vec<Vec<Accuracy>>) -> Vec<u8> {
	let min_err = buf.iter().flatten().filter_map(|acc| {
		match acc {
			Accuracy::Inexact(loss) => Some(*loss),
			_ => None
		}
	}).min_by(f64::total_cmp).unwrap();

	dbg!(min_err);

	let max_err = buf.iter().flatten().filter_map(|acc| {
		match acc {
			Accuracy::Inexact(loss) => Some(*loss),
			_ => None
		}
	}).max_by(f64::total_cmp).unwrap();

	dbg!(max_err);

	buf.iter().rev().flatten().flat_map(|x| {
		color(x, min_err, max_err)
	}).collect()
}

fn parse_data_2d(buf_acc: &[Accuracy]) -> Vec<f64> {
	let mut res = Vec::new();
	for acc in buf_acc {
		use Accuracy::*;

		let acc = match *acc {
		    Exact => 0.0,
		    Inexact(loss) => loss,
		    Overflow => f64::INFINITY,
		    Underflow => f64::INFINITY,
		    NotANumber => f64::INFINITY,
		};
		res.push(acc);
	}
	res
}



const PALETTE: [[u8; 3]; 5] = [
	[  0,   0,   0], // Exact
	[ 54,  74, 255], // Inexact
	[255,  40,  40], // Overflow
	[ 75, 255,  61], // Underflow
	[255, 211,  54], // Not a number
];


fn color(case: &Accuracy, min_err: f64, max_err: f64) -> [u8; 3] {
	use Accuracy::*;
	match case {
		Exact => PALETTE[0],
		Inexact(error) => {
			let [r, g, b] = PALETTE[1];
			let error_adj = (max_err - error) / (max_err - min_err);
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
