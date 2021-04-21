use rupy_ast::{Suite, Stmt, StmtKind, Expr, ExprKind};
use anyhow::Result;

use crate::traits::WrappingType;
use crate::wrappers;


pub fn build_program(suite: Suite) -> Result<String> {
    let mut built_code = vec![];
    for stmt in suite {
        let built = handle_stmt(stmt)?;

        built_code.push(built);
    }

    Ok(built_code.join("\n"))
}


fn handle_stmt(stmt: Stmt) -> Result<String>  {
    match stmt.node {
        StmtKind::FunctionDef { .. } => {}
        StmtKind::AsyncFunctionDef { .. } => {}
        StmtKind::ClassDef { .. } => {}
        StmtKind::Return { .. } => {}
        StmtKind::Delete { .. } => {}
        StmtKind::Assign { targets, value, .. } => {
            let mut assignees = vec![];
            for target in targets {
                let out = target.node.to_lua()?;
                assignees.push(out);
            }

            return wrappers::assign(assignees, value.node, true)
        },
        StmtKind::AugAssign { .. } => {}
        StmtKind::AnnAssign { .. } => {}
        StmtKind::For { .. } => {}
        StmtKind::AsyncFor { .. } => {}
        StmtKind::While { .. } => {}
        StmtKind::If { .. } => {}
        StmtKind::With { .. } => {}
        StmtKind::AsyncWith { .. } => {}
        StmtKind::Raise { .. } => {}
        StmtKind::Try { .. } => {}
        StmtKind::Assert { .. } => {}
        StmtKind::Import { .. } => {}
        StmtKind::ImportFrom { .. } => {}
        StmtKind::Global { .. } => {}
        StmtKind::Nonlocal { .. } => {}
        StmtKind::Expr { .. } => {}
        StmtKind::Pass => {}
        StmtKind::Break => {}
        StmtKind::Continue => {}
    };

    unimplemented!()
}
