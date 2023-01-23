#![deny(
	absolute_paths_not_starting_with_crate,
	future_incompatible,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	non_ascii_idents,
	nonstandard_style,
	noop_method_call,
	pointer_structural_match,
	private_in_public,
	rust_2018_idioms,
	unused_qualifications
)]
#![warn(clippy::pedantic)]
#![allow(clippy::let_underscore_drop)]
#![forbid(unsafe_code)]

mod ast;
mod lex;
mod parse;
mod solve;

fn main() {
	let mut premises = Vec::new();
	let mut deduction = None;
	for line in std::io::stdin().lines() {
		let line = line.unwrap();
		let line = line.trim();

		if let Some(deduction_raw) = line.strip_prefix("∴ ") {
			deduction = Some(ast::Proposition(deduction_raw.chars().next().unwrap()));
			break;
		}

		premises.push(parse::parse(line));
	}

	let deduction = deduction.expect("no deduction before end of stdin");
	let valid = solve::solve(premises, deduction);
	println!(
		"the deduction is {}.",
		match valid {
			Ok(true) => "valid",
			Ok(false) => "invalid",
			Err(self::solve::Unsolvable) => "indeterminate",
		}
	);
}
