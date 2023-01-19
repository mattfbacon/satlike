#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Proposition(pub char);

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Node {
	Proposition(Proposition),
	BinaryOperation(Box<(Self, BinaryOperator, Self)>),
	UnaryOperation(Box<(UnaryOperator, Self)>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
	And,
	Or,
	Imply,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
	Not,
}
