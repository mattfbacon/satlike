use crate::ast::{Node, NodeInner, Proposition};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unsolvable;

pub fn solve(mut premises: Vec<Node>, deduction: Proposition) -> Result<bool, Unsolvable> {
	loop {
		if let Some(solution) = find_solution(&premises, deduction) {
			return Ok(solution);
		}

		let Some((idx, trivial)) = find_trivial(&premises) else { return Err(Unsolvable); };

		premises.remove(idx);
		for premise in &mut premises {
			simplify_with(premise, trivial);
		}
	}
}

fn simplify_with(premise: &mut Node, trivial: Trivial) -> Option<bool> {
	// meaningless value that will never be read
	fn dummy() -> Node {
		Proposition('\0').into()
	}

	match &mut premise.inner {
		NodeInner::Proposition(proposition) => {
			(*proposition == trivial.proposition).then_some(trivial.truth_value)
		}
		NodeInner::Nand(children) => {
			enum Action {
				ReplaceWithLeft,
				ReplaceWithRight,
				DoNothing,
			}

			let [left, right] = &mut **children;
			let left_value = simplify_with(left, trivial);
			let right_value = simplify_with(right, trivial);
			let value_or_action = match (left_value, right_value) {
				(Some(left), Some(right)) => Ok(!(left && right)),
				(Some(false), _) | (_, Some(false)) => Ok(true),
				(Some(true), _) => Err(Action::ReplaceWithRight),
				(_, Some(true)) => Err(Action::ReplaceWithLeft),
				(None, None) => Err(Action::DoNothing),
			};
			match value_or_action {
				Ok(value) => Some(value),
				Err(action) => {
					let replace_with = match action {
						Action::ReplaceWithLeft => Some(std::mem::replace(left, dummy())),
						Action::ReplaceWithRight => Some(std::mem::replace(right, dummy())),
						Action::DoNothing => None,
					};
					if let Some(replace_with) = replace_with {
						// negated due to NAND
						let outer_negated = !premise.negated;
						*premise = replace_with.negate_if(outer_negated);
					}
					None
				}
			}
		}
	}
	.map(|truth_value| truth_value ^ premise.negated)
}

#[derive(Debug, Clone, Copy)]
struct Trivial {
	proposition: Proposition,
	truth_value: bool,
}

fn as_trivial(premise: &Node) -> Option<Trivial> {
	match &premise.inner {
		NodeInner::Proposition(proposition) => Some(Trivial {
			proposition: *proposition,
			truth_value: true,
		}),
		NodeInner::Nand(..) => None,
	}
	.map(|trivial| Trivial {
		truth_value: trivial.truth_value ^ premise.negated,
		..trivial
	})
}

fn find_solution(premises: &[Node], deduction: Proposition) -> Option<bool> {
	premises.iter().find_map(|premise| {
		as_trivial(premise)
			.filter(|trivial| trivial.proposition == deduction)
			.map(|trivial| trivial.truth_value)
	})
}

fn find_trivial(premises: &[Node]) -> Option<(usize, Trivial)> {
	premises
		.iter()
		.enumerate()
		.find_map(|(idx, premise)| as_trivial(premise).map(|trivial| (idx, trivial)))
}

#[cfg(test)]
mod tests {
	use super::*;

	fn do_test<'a>(
		premises: impl AsRef<[&'a str]>,
		deduction: char,
		expected_value: Result<bool, Unsolvable>,
	) {
		let premises: Vec<Node> = premises
			.as_ref()
			.iter()
			.map(|premise| crate::parse::parse(premise))
			.collect();
		let deduction = Proposition(deduction);
		assert_eq!(solve(premises, deduction), expected_value);
	}

	#[test]
	fn trivial() {
		do_test(["a"], 'a', Ok(true));
	}

	#[test]
	fn trivial_negative() {
		do_test(["!a"], 'a', Ok(false));
	}

	#[test]
	fn unrelated() {
		do_test(["a"], 'b', Err(Unsolvable));
	}

	#[test]
	fn imply() {
		do_test(["a", "a -> b"], 'b', Ok(true));
	}

	#[test]
	fn imply_negative() {
		do_test(["a", "a -> !b"], 'b', Ok(false));
	}

	#[test]
	fn imply_negative_2() {
		do_test(["!a -> b", "!a"], 'b', Ok(true));
	}

	#[test]
	fn given_test_case() {
		do_test(
			["(m ∧ ¬b) → j", "(f ∨ s) → m", "b → t", "f → ¬t", "f"],
			'j',
			Ok(true),
		);
	}
}
