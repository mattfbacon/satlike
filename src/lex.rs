#[derive(Debug, logos::Logos, strum::EnumDiscriminants, Clone, Copy)]
#[strum_discriminants(name(TokenKind))]
pub enum Token {
	#[token("(")]
	OpenParen,
	#[token(")")]
	CloseParen,
	#[regex(r"[a-zA-Z]", |lex| lex.slice().chars().next().unwrap())]
	Proposition(char),
	#[token("&")]
	#[token("∧")]
	And,
	#[token("|")]
	#[token("∨")]
	Or,
	#[token("->")]
	#[token("→")]
	Imply,
	#[token("!")]
	#[token("¬")]
	Not,

	#[error]
	#[regex(r"\s+", logos::skip)]
	Error,
}
