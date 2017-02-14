use ast;
use ast::ArrayLiteral;
use ast::BinOp;
use ast::Expression;
use ast::Literal;
use num::Integer;

pub fn fold_module(root: &mut ast::Module) {
    match *root {
        ast::Module::Root { ref mut items } => {
            for item in items.iter_mut() {
                fold_item(item);
            }
        }
        ast::Module::Inline { ref mut items, .. } => {
            for item in items.iter_mut() {
                fold_item(item);
            }
        }
        _ => (),
    }
}

fn fold_item(item: &mut ast::Item) {
    match *item {
        ast::Item::Module(ref mut module) => fold_module(module),
        ast::Item::TypeImpl(_, ref mut imp) => fold_impl(imp),
        ast::Item::Function(_, ref mut func) => fold_func(func),
        _ => (),
    }
}

fn fold_impl(imp: &mut ast::TypeImpl) {
    for func in imp.methods.iter_mut() {
        fold_func(func);
    }
}

fn fold_func(func: &mut ast::Function) {
    fold_block(&mut func.body);
}

fn fold_block(block: &mut ast::Block) {
    for stmnt in block.statements.iter_mut() {
        fold_statement(stmnt);
    }
}

fn fold_statement(stmnt: &mut ast::Statement) {
    use ast::Statement::*;
    match *stmnt {
        Expression(ref mut expr) => fold(expr),
        Declaration(_, _, Some(ref mut expr)) => fold(expr),
        Assignment(ref mut lhs, ref mut extra, _, ref mut rhs) => {
            fold(lhs);
            fold_all(extra);
            fold(rhs);
        }
        IfElse(ast::IfElse { ref mut if_block, ref mut else_block, .. }) => {
            fold_block(if_block);
            if let Some(ref mut else_block) = *else_block {
                fold_block(else_block);
            }
        }
        Loop(ast::Loop { ref mut block, .. }) => fold_block(block),
        ForLoop(ast::ForLoop { ref mut iterator, ref mut block, .. }) => {
            fold(iterator);
            fold_block(block);
        }
        WhileLoop(ast::WhileLoop { ref mut condition, ref mut block, .. }) => {
            fold(condition);
            fold_block(block);
        }
        Return(ref mut exprs) => fold_all(exprs),
        Throw(ref mut expr) => fold(expr),

        _ => (),
    }
}

fn fold_all(exprs: &mut [Expression]) {
    for expr in exprs.iter_mut() {
        fold(expr);
    }
}

pub fn fold(expr: &mut Expression) {
    let new_value = match *expr {
        Expression::Literal(ref mut lit) => {
            fold_literal(lit);
            None
        }
        Expression::MemberAccess(ref mut lhs, _) => {
            fold(lhs);
            None
        }
        Expression::IndexAccess(ref mut lhs, ref mut exprs) => {
            fold(lhs);
            fold_all(exprs);
            None
        }
        Expression::FunctionCall(ref mut lhs, ref mut exprs) => {
            fold(lhs);
            fold_all(exprs);
            None
        }
        Expression::ObjectConstructor(_, ref mut lit) => {
            fold_obj_literal(lit);
            None
        }
        Expression::BinaryOp(ref mut lhs, op, ref mut rhs) => {
            fold(lhs);
            fold(rhs);
            simplify_binary(lhs, op, rhs)
        }
        Expression::Negate(ref mut rhs) => {
            fold(rhs);
            apply_negate(rhs)
        }
        Expression::Not(ref mut rhs) => {
            fold(rhs);
            apply_not(rhs)
        }
        Expression::Try(ref mut lhs) => {
            fold(lhs);
            None
        }
        Expression::Lambda(ref mut lambda) => {
            fold_block(&mut lambda.body);
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
        Literal::Array(ref mut arr) => fold_arr_literal(arr),
        Literal::Object(ref mut obj) => fold_obj_literal(obj),
        Literal::Simd(ref mut arr, _) => fold_all(arr),
        Literal::SimdSplat(ref mut expr, _) => fold(&mut **expr),
        _ => (),
    }
}

fn fold_arr_literal(arr: &mut ArrayLiteral) {
    match *arr {
        ArrayLiteral::List(ref mut exprs) => fold_all(exprs),
        ArrayLiteral::Splat(ref mut val, ref mut count) => {
            fold(val);
            fold(count);
        }
    }
}

fn fold_obj_literal(obj: &mut ast::ObjectLiteral) {
    for (_, expr) in obj.iter_mut() {
        fold(expr);
    }
}

fn apply_negate(rhs: &Expression) -> Option<Expression> {
    use ast::Literal::*;

    let lit = match *rhs {
        Expression::Literal(ref lit) => lit,
        _ => return None,
    };

    Some(Expression::Literal(match *lit {
        Integer(i) => Integer(-i),
        Float(f) => Float(-f),

        _ => return None,
    }))
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
        (&Integer(l), Mod, &Float(r)) => Float(fmod((l as f64), r)),
        (&Float(l), Mod, &Integer(r)) => Float(fmod(l, (r as f64))),
        (&Float(l), Mod, &Float(r)) => Float(fmod(l, r)),

        _ => return None,
    }))
}

fn fmod(x: f64, y: f64) -> f64 {
    x - y * (x / y).trunc()
}

fn frem(x: f64, y: f64) -> f64 {
    x - y * (x / y).round()
}
