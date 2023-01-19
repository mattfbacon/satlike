use crate::ast::{BinaryOperator, Node, Proposition, UnaryOperator};

pub fn solve(mut premises: Vec<Node>, deduction: Proposition) -> bool {
	loop {
		if let Some(solution) = find_solution(&premises, deduction) {
			return solution;
		}
		let Some((idx, trivial)) = find_trivial(&premises) else { panic!("unsolvable!! {premises:#?}"); };
		premises.remove(idx);
		for premise in &mut premises {
			simplify_with(premise, trivial);
		}
	}
}

fn simplify_with(premise: &mut Node, trivial: Trivial) -> Option<bool> {
	// meaningless value that will never be read
	fn dummy() -> Node {
		Node::Proposition(Proposition('a'))
	}

	match premise {
		Node::Proposition(proposition) => {
			(*proposition == trivial.proposition).then_some(trivial.truth_value)
		}
		Node::UnaryOperation(child) => {
			simplify_with(&mut child.1, trivial).map(|value| match child.0 {
				UnaryOperator::Not => !value,
			})
		}
		Node::BinaryOperation(children) => {
			let (left, operator, right) = &mut **children;
			let left_value = simplify_with(left, trivial);
			let right_value = simplify_with(right, trivial);
			#[allow(clippy::unnested_or_patterns)] // clarity
			match (operator, left_value, right_value) {
				// simplified to F
				(BinaryOperator::And, Some(false), _) | (BinaryOperator::And, _, Some(false)) => {
					Some(false)
				}
				// simplified to T
				(BinaryOperator::Or, Some(true), _)
				| (BinaryOperator::Or, _, Some(true))
				| (BinaryOperator::Imply, Some(false), _)
				| (BinaryOperator::Imply, _, Some(true)) => Some(true),
				// simplified to the RHS
				(BinaryOperator::And, Some(true), _)
				| (BinaryOperator::Or, Some(false), _)
				| (BinaryOperator::Imply, Some(true), _) => {
					*premise = std::mem::replace(right, dummy());
					None
				}
				// simplified to the LHS
				(BinaryOperator::And, _, Some(true)) | (BinaryOperator::Or, _, Some(false)) => {
					*premise = std::mem::replace(left, dummy());
					None
				}
				// simplified to the negation of the LHS
				(BinaryOperator::Imply, _, Some(false)) => {
					*premise = Node::UnaryOperation(Box::new((
						UnaryOperator::Not,
						std::mem::replace(left, dummy()),
					)));
					None
				}
				// cannot be simplified
				(_, None, None) => None,
			}
		}
	}
}

#[derive(Debug, Clone, Copy)]
struct Trivial {
	proposition: Proposition,
	truth_value: bool,
}

fn as_trivial(premise: &Node) -> Option<Trivial> {
	match premise {
		Node::Proposition(proposition) => Some(Trivial {
			proposition: *proposition,
			truth_value: true,
		}),
		Node::UnaryOperation(child) => match child.0 {
			UnaryOperator::Not => as_trivial(&child.1).map(|trivial| Trivial {
				truth_value: !trivial.truth_value,
				..trivial
			}),
		},
		Node::BinaryOperation(..) => None,
	}
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
