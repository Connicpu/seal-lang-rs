use ast::BinOp;
use ast::Expression;
use ast::Literal;
use num::Integer;

pub fn fold(expr: &mut Expression) {
    let new_value = match *expr {
        Expression::BinaryOp(ref mut lhs, op, ref mut rhs) => {
            fold(lhs);
            fold(rhs);
            simplify_binary(lhs, op, rhs)
        }
        Expression::Not(ref mut rhs) => {
            fold(rhs);
            apply_not(rhs)
        }
        Expression::Literal(ref mut lit) => {
            fold_literal(lit);
            None
        }
        _ => None,
    };

    if let Some(val) = new_value {
        *expr = val;
    }
}

fn fold_literal(lit: &mut Literal) {
    match *lit {
        Literal::Array(ref mut arr) => {
            for expr in arr.iter_mut() {
                fold(expr);
            }
        }
        Literal::Object(ref mut obj) => {
            for (_, expr) in obj.iter_mut() {
                fold(expr);
            }
        }
        Literal::Simd(ref mut arr, _) => {
            for expr in arr.iter_mut() {
                fold(expr);
            }
        }
        Literal::SimdSplat(ref mut expr, _) => {
            fold(&mut **expr);
        }
        _ => ()
    }
}

fn apply_not(rhs: &Expression) -> Option<Expression> {
    use ast::Literal::*;

    let lit = match *rhs {
        Expression::Literal(ref lit) => lit,
        _ => return None,
    };

    Some(Expression::Literal(match *lit {
        Bool(b) => Bool(!b),
        Integer(i) => Integer(!i),

        _ => return None,
    }))
}

fn simplify_binary(lhs: &Expression, op: BinOp, rhs: &Expression) -> Option<Expression> {
    use ast::BinOp::*;

    let (lhs, rhs) = match (lhs, rhs) {
        (&Expression::Literal(ref lhs), &Expression::Literal(ref rhs)) => (lhs, rhs),
        _ => return None,
    };

    match op {
        LogicalOr | LogicalAnd => simplify_logical(lhs, op, rhs),
        Equal | NotEqual | LessThan | LessOrEqual | GreaterThan | GreaterOrEqual => {
            simplify_equality(lhs, op, rhs)
        }
        BitOr | BitAnd | BitXor => simplify_bitwise(lhs, op, rhs),
        LShiftLeft | LShiftRight | AShiftRight => simplify_shift(lhs, op, rhs),

        Add | Sub | Mul | Div | Rem | Mod => simplify_arithmetic(lhs, op, rhs),

        // I can't constant-fold these
        Implements | RangeExclusive | RangeInclusive | DivRem => None,
    }
}

fn simplify_logical(lhs: &Literal, op: BinOp, rhs: &Literal) -> Option<Expression> {
    use ast::BinOp::*;
    use ast::Literal::*;

    Some(Expression::Literal(match (lhs, op, rhs) {
        (&Bool(l), LogicalOr, &Bool(r)) => Bool(l || r),
        (&Bool(l), LogicalAnd, &Bool(r)) => Bool(l && r),

        _ => return None,
    }))
}

fn simplify_equality(lhs: &Literal, op: BinOp, rhs: &Literal) -> Option<Expression> {
    use ast::BinOp::*;
    use ast::Literal::*;

    Some(Expression::Literal(match (lhs, op, rhs) {
        (&Bool(l), Equal, &Bool(r)) => Bool(l == r),
        (&Integer(l), Equal, &Integer(r)) => Bool(l == r),
        (&Integer(l), Equal, &Float(r)) => Bool((l as f64) == r),
        (&Float(l), Equal, &Integer(r)) => Bool(l == (r as f64)),
        (&Float(l), Equal, &Float(r)) => Bool(l == r),

        (&Bool(l), NotEqual, &Bool(r)) => Bool(l != r),
        (&Integer(l), NotEqual, &Integer(r)) => Bool(l != r),
        (&Integer(l), NotEqual, &Float(r)) => Bool((l as f64) != r),
        (&Float(l), NotEqual, &Integer(r)) => Bool(l != (r as f64)),
        (&Float(l), NotEqual, &Float(r)) => Bool(l != r),

        (&Integer(l), LessThan, &Integer(r)) => Bool(l < r),
        (&Integer(l), LessThan, &Float(r)) => Bool((l as f64) < r),
        (&Float(l), LessThan, &Integer(r)) => Bool(l < (r as f64)),
        (&Float(l), LessThan, &Float(r)) => Bool(l < r),

        (&Integer(l), LessOrEqual, &Integer(r)) => Bool(l <= r),
        (&Integer(l), LessOrEqual, &Float(r)) => Bool((l as f64) <= r),
        (&Float(l), LessOrEqual, &Integer(r)) => Bool(l <= (r as f64)),
        (&Float(l), LessOrEqual, &Float(r)) => Bool(l <= r),

        (&Integer(l), GreaterThan, &Integer(r)) => Bool(l > r),
        (&Integer(l), GreaterThan, &Float(r)) => Bool((l as f64) > r),
        (&Float(l), GreaterThan, &Integer(r)) => Bool(l > (r as f64)),
        (&Float(l), GreaterThan, &Float(r)) => Bool(l > r),

        (&Integer(l), GreaterOrEqual, &Integer(r)) => Bool(l >= r),
        (&Integer(l), GreaterOrEqual, &Float(r)) => Bool((l as f64) >= r),
        (&Float(l), GreaterOrEqual, &Integer(r)) => Bool(l >= (r as f64)),
        (&Float(l), GreaterOrEqual, &Float(r)) => Bool(l >= r),

        _ => return None,
    }))
}

fn simplify_bitwise(lhs: &Literal, op: BinOp, rhs: &Literal) -> Option<Expression> {
    use ast::BinOp::*;
    use ast::Literal::*;

    Some(Expression::Literal(match (lhs, op, rhs) {
        (&Bool(l), BitOr, &Bool(r)) => Bool(l | r),
        (&Bool(l), BitAnd, &Bool(r)) => Bool(l & r),
        (&Bool(l), BitXor, &Bool(r)) => Bool(l ^ r),

        (&Integer(l), BitOr, &Integer(r)) => Integer(l | r),
        (&Integer(l), BitAnd, &Integer(r)) => Integer(l & r),
        (&Integer(l), BitXor, &Integer(r)) => Integer(l ^ r),

        _ => return None,
    }))
}

fn simplify_shift(lhs: &Literal, op: BinOp, rhs: &Literal) -> Option<Expression> {
    use ast::BinOp::*;
    use ast::Literal::*;

    Some(Expression::Literal(match (lhs, op, rhs) {
        (&Integer(l), LShiftLeft, &Integer(r)) => Integer(l << r),
        (&Integer(l), AShiftRight, &Integer(r)) => Integer(l >> r),
        (&Integer(l), LShiftRight, &Integer(r)) => Integer(((l as u64) >> r) as i64),

        _ => return None,
    }))
}

fn simplify_arithmetic(lhs: &Literal, op: BinOp, rhs: &Literal) -> Option<Expression> {
    use ast::BinOp::*;
    use ast::Literal::*;

    Some(Expression::Literal(match (lhs, op, rhs) {
        // Add
        (&Integer(l), Add, &Integer(r)) => Integer(l + r),
        (&Integer(l), Add, &Float(r)) => Float((l as f64) + r),
        (&Float(l), Add, &Integer(r)) => Float(l + (r as f64)),
        (&Float(l), Add, &Float(r)) => Float(l + r),

        // Sub
        (&Integer(l), Sub, &Integer(r)) => Integer(l - r),
        (&Integer(l), Sub, &Float(r)) => Float((l as f64) - r),
        (&Float(l), Sub, &Integer(r)) => Float(l - (r as f64)),
        (&Float(l), Sub, &Float(r)) => Float(l - r),

        // Mul
        (&Integer(l), Mul, &Integer(r)) => Integer(l * r),
        (&Integer(l), Mul, &Float(r)) => Float((l as f64) * r),
        (&Float(l), Mul, &Integer(r)) => Float(l * (r as f64)),
        (&Float(l), Mul, &Float(r)) => Float(l * r),

        // Div
        (&Integer(l), Div, &Integer(r)) => Integer(l / r),
        (&Integer(l), Div, &Float(r)) => Float((l as f64) / r),
        (&Float(l), Div, &Integer(r)) => Float(l / (r as f64)),
        (&Float(l), Div, &Float(r)) => Float(l / r),

        // Rem
        (&Integer(l), Rem, &Integer(r)) => Integer(l % r),
        (&Integer(l), Rem, &Float(r)) => Float(frem((l as f64), r)),
        (&Float(l), Rem, &Integer(r)) => Float(frem(l, (r as f64))),
        (&Float(l), Rem, &Float(r)) => Float(frem(l, r)),

        // Mod
        (&Integer(l), Mod, &Integer(r)) => Integer(l.mod_floor(&r)),
        (&Integer(l), Mod, &Float(r)) => Float(frem((l as f64), r)),
        (&Float(l), Mod, &Integer(r)) => Float(frem(l, (r as f64))),
        (&Float(l), Mod, &Float(r)) => Float(frem(l, r)),

        _ => return None,
    }))
}

fn fmod(x: f64, y: f64) -> f64 {
    x - y * (x / y).trunc()
}

fn frem(x: f64, y: f64) -> f64 {
    x - y * (x / y).round()
}
