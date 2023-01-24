use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Proposition(pub char);

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NodeInner {
	Proposition(Proposition),
	Nand(Box<[Node; 2]>),
}

impl Display for NodeInner {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Proposition(proposition) => proposition.0.fmt(formatter)?,
			Self::Nand(children) => {
				let [left, right] = &**children;

				let left_needs_parens = !left.negated && matches!(left.inner, Self::Nand(..));
				if left_needs_parens {
					formatter.write_str("(")?;
				}
				left.fmt(formatter)?;
				if left_needs_parens {
					formatter.write_str(")")?;
				}

				formatter.write_str(" ↑ ")?;

				let right_needs_parens = !right.negated && matches!(right.inner, Self::Nand(..));
				if right_needs_parens {
					formatter.write_str("(")?;
				}
				right.fmt(formatter)?;
				if right_needs_parens {
					formatter.write_str(")")?;
				}
			}
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Node {
	pub negated: bool,
	pub inner: NodeInner,
}

impl Node {
	pub fn negate(self) -> Self {
		Self {
			negated: !self.negated,
			..self
		}
	}

	pub fn negate_if(self, cond: bool) -> Self {
		Self {
			negated: self.negated ^ cond,
			..self
		}
	}
}

impl From<NodeInner> for Node {
	fn from(inner: NodeInner) -> Self {
		Self {
			negated: false,
			inner,
		}
	}
}

impl From<Proposition> for Node {
	fn from(proposition: Proposition) -> Self {
		Self {
			negated: false,
			inner: NodeInner::Proposition(proposition),
		}
	}
}

impl Display for Node {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		let needs_parens = self.negated && matches!(self.inner, NodeInner::Nand(..));
		if self.negated {
			formatter.write_str("!")?;
		}
		if needs_parens {
			formatter.write_str("(")?;
		}
		self.inner.fmt(formatter)?;
		if needs_parens {
			formatter.write_str(")")?;
		}
		Ok(())
	}
}
