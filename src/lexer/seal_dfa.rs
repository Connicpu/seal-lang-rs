use lexer::TokenType as TT;
use lexer::dfa::Dfa;
use lexer::emoji::EmojiChar;
use unicode_xid::UnicodeXID;

pub fn create_dfa() -> Dfa<TT, char> {
    let mut dfa = Dfa::<TT, char>::new();
    let root = dfa.root();

    // Whitespace
    let whitespace = dfa.create(TT::Whitespace);
    dfa.transition_complex(root, whitespace, |c| c.is_whitespace());
    dfa.transition_complex(whitespace, whitespace, |c| c.is_whitespace());

    // Single-line comments
    let single_comment_begin = dfa.create(None);
    let single_comment = dfa.create(TT::Comment);
    let doc_comment = dfa.create(TT::DocComment);
    let mod_doc_comment = dfa.create(TT::ModuleDocComment);

    dfa.transition(root, single_comment_begin, '/');
    dfa.transition(single_comment_begin, single_comment, '/');
    dfa.transition(single_comment, doc_comment, '/');
    dfa.transition(single_comment, mod_doc_comment, '!');

    dfa.transition_default(single_comment, single_comment);
    dfa.transition(single_comment, None, '\r');
    dfa.transition(single_comment, None, '\n');

    dfa.transition_default(doc_comment, doc_comment);
    dfa.transition(doc_comment, None, '\r');
    dfa.transition(doc_comment, None, '\n');

    dfa.transition_default(mod_doc_comment, mod_doc_comment);
    dfa.transition(mod_doc_comment, None, '\r');
    dfa.transition(mod_doc_comment, None, '\n');

    // Multi-line comments
    let multi_comment = dfa.create(TT::Comment);
    let multi_body = dfa.create(TT::Comment);
    let multi_maybe_end = dfa.create(TT::Comment);

    dfa.transition_default(multi_body, multi_body);
    dfa.transition_default(multi_maybe_end, multi_body);

    dfa.transition(single_comment_begin, multi_body, '*');
    dfa.transition(multi_body, multi_maybe_end, '*');
    dfa.transition(multi_maybe_end, multi_comment, '\0');
    dfa.transition(multi_body, multi_comment, '\0');

    // Identifiers
    let identifier = dfa.create(TT::Identifier);
    let identifier_emb = dfa.create(TT::Identifier);
    dfa.transition(root, identifier, '_');
    dfa.transition_complex(root, identifier, |c| UnicodeXID::is_xid_start(*c));
    dfa.transition_complex(root, identifier_emb, |c| c.is_emoji_modifier_base());
    dfa.transition_complex(root, identifier, |c| c.is_emoji());

    let ident_edge = |dfa: &mut Dfa<TT, char>, from| {
        dfa.transition(from, identifier, '_');
        dfa.transition_complex(from, identifier, |c| UnicodeXID::is_xid_continue(*c));
        dfa.transition_complex(from, identifier_emb, |c| c.is_emoji_modifier_base());
        dfa.transition_complex(from, identifier, |c| c.is_emoji());
    };

    ident_edge(&mut dfa, identifier);
    ident_edge(&mut dfa, identifier_emb);
    dfa.transition_complex(identifier_emb, identifier, |c| c.is_emoji_modifier());

    // mod= special case
    let modeq_m = dfa.create(None);
    let modeq_mo = dfa.create(None);
    let modeq_mod = dfa.create(TT::Mod);
    let modeq = dfa.create(TT::ModAssign);
    dfa.transition(root, modeq_m, 'm');
    dfa.transition(modeq_m, modeq_mo, 'o');
    dfa.transition(modeq_mo, modeq_mod, 'd');
    dfa.transition(modeq_mod, modeq, '=');
    ident_edge(&mut dfa, modeq_m);
    ident_edge(&mut dfa, modeq_mo);
    ident_edge(&mut dfa, modeq_mod);

    // Integers
    let int_literal = dfa.create(TT::IntLiteral);
    let int_separator = dfa.create(None);
    let zero_prefix = dfa.create(TT::IntLiteral);

    dfa.transition(root, zero_prefix, '0');
    dfa.transition_complex(root, int_literal, |c| c.is_digit(10));
    dfa.transition_complex(int_literal, int_literal, |c| c.is_digit(10));
    dfa.transition_complex(int_separator, int_literal, |c| c.is_digit(10));
    dfa.transition_complex(zero_prefix, int_literal, |c| c.is_digit(10));
    dfa.transition(int_literal, int_separator, '_');

    let hex_start = dfa.create(None);
    let oct_start = dfa.create(None);
    let bin_start = dfa.create(None);

    dfa.transition(zero_prefix, hex_start, 'x');
    dfa.transition(zero_prefix, oct_start, 'o');
    dfa.transition(zero_prefix, bin_start, 'b');

    let hex_literal = dfa.create(TT::HexLiteral);
    let hex_separator = dfa.create(None);

    dfa.transition_complex(hex_start, hex_literal, |c| c.is_digit(16));
    dfa.transition_complex(hex_separator, hex_literal, |c| c.is_digit(16));
    dfa.transition(hex_literal, hex_separator, '_');

    let oct_literal = dfa.create(TT::OctLiteral);
    let oct_separator = dfa.create(None);

    dfa.transition_complex(oct_start, oct_literal, |c| c.is_digit(8));
    dfa.transition_complex(oct_separator, oct_literal, |c| c.is_digit(8));
    dfa.transition(oct_literal, oct_separator, '_');

    let bin_literal = dfa.create(TT::BinLiteral);
    let bin_separator = dfa.create(None);

    dfa.transition_complex(bin_start, bin_literal, |c| c.is_digit(2));
    dfa.transition_complex(bin_separator, bin_literal, |c| c.is_digit(2));
    dfa.transition(bin_literal, bin_separator, '_');

    // Floats
    let dotted_float = dfa.create(None);
    let float_literal = dfa.create(TT::FloatLiteral);
    let float_separator = dfa.create(None);

    dfa.transition(zero_prefix, dotted_float, '.');
    dfa.transition(int_literal, dotted_float, '.');
    dfa.transition_complex(dotted_float, float_literal, |c| c.is_digit(10));
    dfa.transition_complex(float_literal, float_literal, |c| c.is_digit(10));
    dfa.transition_complex(float_separator, float_literal, |c| c.is_digit(10));
    dfa.transition(float_literal, float_separator, '_');

    // Scientific floats
    let scientific_start = dfa.create(None);
    let signed_scientific = dfa.create(None);
    let scientific_literal = dfa.create(TT::FloatLiteral);

    dfa.transition(float_literal, scientific_start, 'e');
    dfa.transition(scientific_start, signed_scientific, '+');
    dfa.transition(scientific_start, signed_scientific, '-');
    dfa.transition_complex(scientific_start, scientific_literal, |c| c.is_digit(10));
    dfa.transition_complex(signed_scientific, scientific_literal, |c| c.is_digit(10));

    // Strings
    let string_literal = dfa.create(TT::StringLiteral);
    let string_body = dfa.create(None);
    let string_escape = dfa.create(None);

    dfa.transition(root, string_body, '"');
    dfa.transition_default(string_body, string_body);
    dfa.transition(string_body, string_literal, '"');
    dfa.transition(string_body, string_escape, '\\');
    dfa.transition_default(string_escape, string_body);

    // Char literals
    let char_literal = dfa.create(TT::CharLiteral);
    let label1 = dfa.create(TT::Label);
    let label = dfa.create(TT::Label);
    let label_begin = dfa.create(None);
    let char_escape = dfa.create(None);
    let char_end = dfa.create(None);
    let char_u_begin = dfa.create(None);
    let char_u_value = dfa.create(None);

    dfa.transition(root, label_begin, '\'');

    dfa.transition_default(label_begin, char_end);
    dfa.transition_complex(label_begin, label1, |c| UnicodeXID::is_xid_start(*c));
    dfa.transition(label_begin, char_escape, '\\');

    dfa.transition(label1, char_literal, '\'');
    dfa.transition_complex(label1, label, |c| UnicodeXID::is_xid_continue(*c));
    dfa.transition_complex(label, label, |c| UnicodeXID::is_xid_continue(*c));

    dfa.transition_default(char_escape, char_end);
    dfa.transition(char_escape, char_u_begin, 'u');
    dfa.transition(char_end, char_literal, '\\');

    dfa.transition(char_u_begin, char_u_value, '{');
    dfa.transition(char_u_value, char_end, '}');
    dfa.transition_complex(char_u_value, char_u_value, |c| c.is_digit(16));

    // Operators
    dfa.insert_string(root, "{".chars(), TT::OpenCurly);
    dfa.insert_string(root, "}".chars(), TT::CloseCurly);
    dfa.insert_string(root, "(".chars(), TT::OpenParen);
    dfa.insert_string(root, ")".chars(), TT::CloseParen);
    dfa.insert_string(root, "[".chars(), TT::OpenBracket);
    dfa.insert_string(root, "]".chars(), TT::CloseBracket);
    dfa.insert_string(root, ";".chars(), TT::Semicolon);
    dfa.insert_string(root, ":".chars(), TT::Colon);
    dfa.insert_string(root, ",".chars(), TT::Comma);
    dfa.insert_string(root, "?".chars(), TT::Question);
    dfa.insert_string(root, ".".chars(), TT::Dot);

    dfa.insert_string(root, "..".chars(), TT::RangeExclusive);
    dfa.insert_string(root, "...".chars(), TT::RangeInclusive);

    dfa.insert_string(root, "==".chars(), TT::Equal);
    dfa.insert_string(root, "!=".chars(), TT::NotEqual);
    dfa.insert_string(root, "<".chars(), TT::LessThan);
    dfa.insert_string(root, "<=".chars(), TT::LessOrEqual);
    dfa.insert_string(root, ">".chars(), TT::GreaterThan);
    dfa.insert_string(root, ">=".chars(), TT::GreaterOrEqual);

    dfa.insert_string(root, "!".chars(), TT::Not);

    dfa.insert_string(root, "&&".chars(), TT::LogicalAnd);
    dfa.insert_string(root, "||".chars(), TT::LogicalOr);
    dfa.insert_string(root, "&&=".chars(), TT::LogicalAndAssign);
    dfa.insert_string(root, "||=".chars(), TT::LogicalOrAssign);

    dfa.insert_string(root, "+".chars(), TT::Add);
    dfa.insert_string(root, "-".chars(), TT::Sub);
    dfa.insert_string(root, "*".chars(), TT::Mul);
    dfa.insert_string(root, "/".chars(), TT::Div);
    dfa.insert_string(root, "%".chars(), TT::Rem);
    dfa.insert_string(root, "/%".chars(), TT::DivRem);
    dfa.insert_string(root, "&".chars(), TT::BitAnd);
    dfa.insert_string(root, "|".chars(), TT::BitOr);
    dfa.insert_string(root, "^".chars(), TT::BitXor);
    dfa.insert_string(root, "<<".chars(), TT::Shl);
    dfa.insert_string(root, ">>".chars(), TT::Shr);
    dfa.insert_string(root, ">>>".chars(), TT::LShr);

    dfa.insert_string(root, "=".chars(), TT::Assign);
    dfa.insert_string(root, "+=".chars(), TT::AddAssign);
    dfa.insert_string(root, "-=".chars(), TT::SubAssign);
    dfa.insert_string(root, "*=".chars(), TT::MulAssign);
    dfa.insert_string(root, "/=".chars(), TT::DivAssign);
    dfa.insert_string(root, "%=".chars(), TT::RemAssign);
    dfa.insert_string(root, "&=".chars(), TT::BitAndAssign);
    dfa.insert_string(root, "|=".chars(), TT::BitOrAssign);
    dfa.insert_string(root, "^=".chars(), TT::BitXorAssign);
    dfa.insert_string(root, "<<=".chars(), TT::ShlAssign);
    dfa.insert_string(root, ">>=".chars(), TT::ShrAssign);
    dfa.insert_string(root, ">>>=".chars(), TT::LShrAssign);

    dfa
}
