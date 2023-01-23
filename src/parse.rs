use logos::Logos as _;
use nom::branch::alt;
use nom::combinator::{all_consuming, map, opt};
use nom::multi::many0_count;
use nom::sequence::{delimited, tuple};

use crate::ast::{BinaryOperator, Node, NodeInner, Proposition};
use crate::lex::{Token, TokenKind};

pub fn parse(input: &str) -> Node {
	let tokens: Vec<_> = Token::lexer(input).collect();
	let mut parser = all_consuming(node);
	parser(&tokens).unwrap().1
}

impl<'a> nom::Parser<&'a [Token], Token, nom::error::Error<&'a [Token]>> for TokenKind {
	fn parse(&mut self, input: &'a [Token]) -> ParseResult<'a, Token> {
		if let Some(token) = input
			.first()
			.copied()
			.filter(|token| *self == TokenKind::from(token))
		{
			Ok((&input[1..], token))
		} else {
			Err(nom::Err::Error(nom::error::Error::new(
				input,
				nom::error::ErrorKind::OneOf,
			)))
		}
	}
}

type ParseResult<'a, T> = nom::IResult<&'a [Token], T>;

fn proposition(input: &[Token]) -> ParseResult<'_, Node> {
	if let Some(Token::Proposition(ch)) = input.first() {
		Ok((&input[1..], Proposition(*ch).into()))
	} else {
		Err(nom::Err::Error(nom::error::Error::new(
			input,
			nom::error::ErrorKind::Char,
		)))
	}
}

fn node(input: &[Token]) -> ParseResult<'_, Node> {
	fn layer2(input: &[Token]) -> ParseResult<'_, Node> {
		alt((
			proposition,
			delimited(TokenKind::OpenParen, node, TokenKind::CloseParen),
		))(input)
	}

	fn layer1(input: &[Token]) -> ParseResult<'_, Node> {
		map(
			tuple((many0_count(TokenKind::Not), layer2)),
			|(nots, inner)| {
				let negated = nots % 2 == 1;
				inner.negate_if(negated)
			},
		)(input)
	}

	map(
		tuple((layer1, opt(tuple((binary_operator, layer1))))),
		|(left, opt_binary)| {
			if let Some((binary, right)) = opt_binary {
				NodeInner::BinaryOperation(Box::new((left, binary, right))).into()
			} else {
				left
			}
		},
	)(input)
}

fn binary_operator(input: &[Token]) -> ParseResult<'_, BinaryOperator> {
	input
		.first()
		.copied()
		.and_then(|token| match token {
			Token::And => Some(BinaryOperator::And),
			Token::Or => Some(BinaryOperator::Or),
			Token::Imply => Some(BinaryOperator::Imply),
			_ => None,
		})
		.map_or_else(
			|| {
				Err(nom::Err::Error(nom::error::Error::new(
					input,
					nom::error::ErrorKind::OneOf,
				)))
			},
			|ret| Ok((&input[1..], ret)),
		)
}

#[cfg(test)]
mod tests {
	use super::parse;
	use crate::ast::{BinaryOperator, Node, NodeInner, Proposition};

	fn make_prop(proposition: char) -> Node {
		Proposition(proposition).into()
	}

	fn make_binop(left: impl Into<Node>, op: BinaryOperator, right: impl Into<Node>) -> Node {
		NodeInner::BinaryOperation(Box::new((left.into(), op, right.into()))).into()
	}

	#[test]
	fn proposition() {
		assert_eq!(parse("a"), Node::from(Proposition('a')));
	}

	#[test]
	fn binary_operators() {
		assert_eq!(
			parse("a & b"),
			make_binop(make_prop('a'), BinaryOperator::And, make_prop('b')),
		);
		assert_eq!(
			parse("a | b"),
			make_binop(make_prop('a'), BinaryOperator::Or, make_prop('b')),
		);
		assert_eq!(
			parse("a → b"),
			make_binop(make_prop('a'), BinaryOperator::Imply, make_prop('b')),
		);
	}

	#[test]
	fn unary_operators() {
		assert_eq!(parse("!a"), make_prop('a').negate());
	}

	#[test]
	fn mixed() {
		assert_eq!(
			parse("(a & (!b | c)) -> !d"),
			make_binop(
				make_binop(
					make_prop('a'),
					BinaryOperator::And,
					make_binop(make_prop('b').negate(), BinaryOperator::Or, make_prop('c')),
				),
				BinaryOperator::Imply,
				make_prop('d').negate(),
			),
		);
	}
}
