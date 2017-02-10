use std::iter::Peekable;
use std::str::CharIndices;

pub mod dfa;
pub mod emoji;
pub mod keywords;
pub mod seal_dfa;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
pub type Tok<'input> = (TokenType, &'input str);

pub struct Lexer<'input> {
    source: &'input str,
    chars: Peekable<CharIndices<'input>>,
    loc: Location,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            source: input,
            chars: input.char_indices().peekable(),
            loc: Location {
                line: 1,
                column: 1,
                index: 0,
            },
        }
    }
}

lazy_static!{
    static ref SEAL_DFA: dfa::Dfa<TokenType, char> = seal_dfa::create_dfa();
}

impl<'input> Lexer<'input> {
    fn do_next(&mut self) -> Option<<Self as Iterator>::Item> {
        let dfa: &dfa::Dfa<TokenType, char> = &*SEAL_DFA;

        let mut initial_iter = self.chars.clone();

        let start = self.loc;
        let mut node = dfa.root();
        let mut last_accepting = None;
        let &(_, first) = match self.chars.peek() {
            Some(c) => c,
            None => return None,
        };

        loop {
            let (i, c) = match self.chars.next() {
                None => return None,
                Some(i) => i,
            };

            self.loc.index = i;
            self.loc.column += 1;

            if let Some(next) = dfa.next(node, &c) {
                if let Some(&state) = dfa.state(next) {
                    let mut end = self.loc;
                    end.index = i + c.len_utf8();

                    let span = &self.source[start.index..end.index];
                    let tok = Token {
                        kind: state,
                        span: span,
                    };
                    last_accepting = Some((tok, end, self.chars.clone()));
                }

                node = next;
            } else {
                break;
            }

            if c == '\n' {
                self.loc.line += 1;
                self.loc.column = 1;
            }
        }

        Some(match last_accepting {
            Some((mut tok, loc, iter)) => {
                self.loc = loc;
                self.chars = iter;
                if tok.kind == TokenType::Identifier {
                    if let Some(tt) = keywords::match_keyword(tok.span) {
                        tok.kind = tt;
                    }
                }
                Ok((start, (tok.kind, tok.span), loc))
            }
            None => {
                let (i, _) = initial_iter.next().unwrap();
                self.loc = start;
                self.loc.column += 1;
                self.loc.index = i;
                self.chars = initial_iter;
                Err(LexicalError::Unexpected(first, start))
            }
        })
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, Location, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.do_next() {
                Some(Ok((_, (TokenType::Whitespace, _), _))) => continue,
                Some(Ok((_, (TokenType::Comment, _), _))) => continue,
                t => return t,
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

#[derive(Debug)]
pub enum LexicalError {
    Unexpected(char, Location),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Token<'input> {
    pub kind: TokenType,
    pub span: &'input str,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenType {
    Invalid,

    Identifier,
    Whitespace,

    Comment,
    DocComment,
    ModuleDocComment,

    IntLiteral,
    HexLiteral,
    OctLiteral,
    BinLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    Label,

    Break,
    Continue,
    Else,
    Enum,
    Extern,
    Function, // `fn`
    For,
    If,
    Impl,
    Impls,
    In,
    Let,
    Mod, // `mod`
    NewObject, // `new_object`. I like using `new` as a function name
    Nil,
    Return,
    Throw,
    Trait,
    Type,
    Use,

    OpenCurly,
    CloseCurly,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Colon,
    Comma,
    Question,
    Dot,

    RangeExclusive,
    RangeInclusive,

    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,

    Not,

    LogicalAnd,
    LogicalOr,

    LogicalAndAssign,
    LogicalOrAssign,

    Add,
    Sub,
    Mul,
    Div,
    Rem,
    DivRem,

    BitAnd,
    BitOr,
    BitXor,

    Shl,
    Shr,
    LShr,

    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    ModAssign,

    BitAndAssign,
    BitOrAssign,
    BitXorAssign,

    ShlAssign,
    ShrAssign,
    LShrAssign,
}
