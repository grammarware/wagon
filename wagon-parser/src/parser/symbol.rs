use crate::parser::Span;
use std::fmt::Display;
use std::write;

use super::helpers::between_sep;
use super::{Parse, LexerBridge, ParseResult, Tokens, SpannableNode, Peek, ResultPeek};
use wagon_lexer::{math::Math, productions::Productions};

use super::terminal::Terminal;
use super::assignment::Assignment;
use super::Ident;

#[cfg(test)]
use wagon_macros::new_unspanned;

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
#[cfg_attr(test, new_unspanned)]
/// A symbol in a [`Chunk`][super::chunk::Chunk].
///
/// A symbol is any individual element of a `Chunk`.
pub enum Symbol {
    /// A non-terminal with optional parameters.
	NonTerminal(SpannableNode<Ident>, Vec<SpannableNode<Ident>>),
    /// A list of [`Assignment`] enclosed by `{}`.
	Assignment(Vec<SpannableNode<Assignment>>),
    /// A Terminal.
	Terminal(SpannableNode<Terminal>),
    /// Nothing.
    Epsilon
}

impl Parse for Symbol {
    fn parse(lexer: &mut LexerBridge) -> ParseResult<Self> where Self: Sized {
        match lexer.peek_result()? {
        	Tokens::ProductionToken(Productions::Identifier(_)) => {
                let ident = SpannableNode::parse(lexer)?;
                let args = if lexer.peek() == Some(&Ok(Tokens::ProductionToken(Productions::Lt))) {
                    between_sep(lexer, &Tokens::ProductionToken(Productions::Lt), &Tokens::ProductionToken(Productions::Gt), Tokens::ProductionToken(Productions::Comma))?
                } else {
                    Vec::new()
                };
    			Ok(Self::NonTerminal(ident, args))
        	},
        	Tokens::ProductionToken(Productions::LCur) => {
                lexer.next();
                Ok(Self::Assignment(SpannableNode::parse_sep_end(lexer, Tokens::MathToken(Math::Semi), Tokens::MathToken(Math::RCur))?))
        	},
        	_ => Ok(Self::Terminal(SpannableNode::parse(lexer)?))
        }
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Self::Epsilon
    }
}

impl Symbol {

    /// Check if this symbol is not a non-terminal.
    pub(crate) const fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal(..) | Self::Assignment(..) | Self::Epsilon)
    }

    /// Check if this symbol is an [`Assignment`].
    #[must_use] 
    pub const fn is_assignment(&self) -> bool {
        matches!(self, Self::Assignment(..))
    }

    #[cfg(test)]
    /// Create a symbol which is just a [`Terminal::LitString`] representing the input parameter.
    pub(crate) fn simple_terminal(ident: &str) -> Self {
        Self::Terminal(SpannableNode::new(Terminal::LitString(ident.to_string()), 0..ident.len()))
    }

    #[cfg(test)]
    /// Create a symbol which is just a non-terminal [`Ident::Unknown`] with no arguments, representing the input parameter.
    pub (crate) fn simple_ident(ident: &str) -> Self {
        Self::NonTerminal(SpannableNode::new(Ident::Unknown(ident.to_string()), 0..ident.len()), Vec::new())
    }

    /// Create a symbol which is just a spanned non-terminal [`Ident::Unknown`].
    pub(crate) fn simple_ident_spanned(ident: &str, span: Span) -> SpannableNode<Self> {
        SpannableNode::new(Self::NonTerminal(SpannableNode::new(Ident::Unknown(ident.to_string()), span.clone()), Vec::new()), span)
    }
}

use itertools::Itertools;
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonTerminal(i, args) => {
                if args.is_empty() {
                    write!(f, "{i}")
                } else {
                    write!(f, "{i}({})", args.iter().join(", "))
                }
            }
            Self::Assignment(i) => write!(f, "{{{}}}", i.iter().join("; ")),
            Self::Terminal(i) => write!(f, "{i}"),
            Self::Epsilon => write!(f, "ε"),
        }
    }
}