use stdx::format_to;
use syntax::{
    SyntaxKind,
    ast::{self, edit_in_place::Indent, HasArgList, Pat, Expr},
    AstNode,
};
use ide_db::helpers::node_ext::walk_pat;

use crate::{AssistContext, AssistId, AssistKind, Assists};

// Assist: convert_if_to_filter
//
// Converts a if into a filter when placed in a for_each().
// ```
// # //- minicore: iterators
// # use core::iter;
// fn main() {
//     let it = core::iter::repeat(92);
//     it.for_each$0(|x| {
//         if x > 4 {
//             println!("{}", x);
//         };
//     });
// }
// ```
// ->
// ```
// # use core::iter;
// fn main() {
//     let it = core::iter::repeat(92);
//     it.filter(|&x| x > 4).for_each(|x| {
//         println!("{}", x);
//     });
// }
// ```
pub(crate) fn convert_if_to_filter(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    let method = ctx.find_node_at_offset::<ast::MethodCallExpr>()?;

    let closure = match method.arg_list()?.args().next()? {
        ast::Expr::ClosureExpr(expr) => expr,
        _ => return None,
    };

    let (method, receiver) = validate_method_call_expr(method)?;

    let param_list = closure.param_list()?;
    let param = param_list.params().next()?.pat()?;
    let body = closure.body()?;

    let range = method.syntax().text_range();

    let if_expr = match body.clone() {
        Expr::IfExpr(if_expr) => {
            if_expr
        },
        Expr::BlockExpr(block) => {
            let mut stmts = block.statements();
            let fst_stmt = stmts.next()?;
            continue_iff(stmts.next().is_none())?; // Only one statement
            // First statement is an expression...
            let expr_stmt = match fst_stmt {
                ast::Stmt::ExprStmt(expr_stmt) => expr_stmt,
                _ => return None,
            };

            // ...and even an if clause...
            let expr = expr_stmt.expr()?;
            let if_expr = match expr {
                ast::Expr::IfExpr(my_if_expr) => my_if_expr,
                _ => return None,
            };
            if_expr
        },
        _ => return None,
    };

    let condition = if_expr.condition()?; // ... with a condition...
    continue_iff(if_expr.else_branch().is_none()); // ... and no else branch...
    let then_branch = if_expr.then_branch()?; // ... and a then branch

    acc.add(
        AssistId("convert_if_to_filter", AssistKind::RefactorRewrite),
        "Replace this `if { ... }` with a `filter()`",
        range,
        |builder| {
            let indent = method.indent_level();
            
            let mut buf = String::new();
            // Recursively remove unnecessary `mut`s in the parameter
            let pat_filter = param.clone_for_update();
            let mut to_be_removed = vec![];
            walk_pat(&pat_filter, &mut |cb|
                if let Pat::IdentPat(ident) = cb {
                    if let Some(mut_token) = ident.mut_token() {
                        to_be_removed.push(mut_token);
                    }
                }
            );
            for mut_token in to_be_removed.into_iter() {
                if let Some(ws) = mut_token.next_token().filter(|it| it.kind() == SyntaxKind::WHITESPACE) {
                    ws.detach();
                }
                mut_token.detach();
            }
            format_to!(buf, "{}.filter(|&{}| {})", receiver, pat_filter, condition);

            // Because we removed a if block, reident accordingly the rest of the block
            let block = then_branch.clone_for_update();
            block.reindent_to(indent);

            format_to!(buf, ".for_each(|{}| {})", param, block);

            builder.replace(range, buf)
        },
    )
}

fn validate_method_call_expr(
    expr: ast::MethodCallExpr,
) -> Option<(ast::Expr, ast::Expr)> {
    let name_ref = expr.name_ref()?;
    if name_ref.text() != "for_each" {
        return None;
    }

    let receiver = expr.receiver()?;
    let expr = ast::Expr::MethodCallExpr(expr);

    Some((expr, receiver))
}

fn continue_iff(b: bool) -> Option<()> {
    if b { Some(()) } else { None }
}

#[cfg(test)]
mod tests {
    use crate::tests::check_assist;

    use super::*;

    #[test]
    fn if_to_filter() {
        check_assist(
            convert_if_to_filter,
            r#"
fn main() {
    let it = core::iter::repeat((92,42));
    it.for_each$0(|(mut i,mut j)| {
        if (i*j)%3 == 2 {
            i *= 2;
        };
    });
}"#,
            r#"
fn main() {
    let it = core::iter::repeat((92,42));
    it.filter(|&(i,j)| (i*j)%3 == 2).for_each(|(mut i,mut j)| {
        i *= 2;
    });
}"#,
        )
    }
}
