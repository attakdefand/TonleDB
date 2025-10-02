use sqlparser::{dialect::GenericDialect, parser::Parser};
use tonledb_core::{Db, DbError, Result, Space};

const TBL_PREFIX: &str = "tbl/";

pub fn execute_sql(db: &Db, sql: &str) -> Result<serde_json::Value> {
    let stmts = Parser::parse_sql(&GenericDialect, sql).map_err(|e| DbError::Invalid(e.to_string()))?;
    if stmts.len() != 1 {
        return Err(DbError::Invalid("only single statement supported".into()));
    }
    
    match &stmts[0] {
        sqlparser::ast::Statement::Query(q) => {
            if let sqlparser::ast::SetExpr::Select(sel) = &*q.body {
                if sel.from.len() != 1 {
                    return Err(DbError::Invalid("SELECT from exactly one table".into()));
                }
                
                let tname = &sel.from[0].relation.to_string();
                let projection = &sel.projection;
                let selection = &sel.selection;
                
                // Simplified version without index lookup
                // Fallback: full scan with selection
                let prefix = format!("{}{}{}", TBL_PREFIX, tname, "/").into_bytes();
                let iter = db.storage.scan_prefix(&Space("data".into()), &prefix)?;
                let mut out = vec![];
                for (_, v) in iter { 
                    let mut obj: serde_json::Value = serde_json::from_slice(&v).map_err(|e| DbError::Storage(e.to_string()))?; 
                    if let Some(sel) = selection { 
                        if !eval_simple_where(&obj, &sel)? { 
                            continue; 
                        } 
                    } 
                    out.push(project_simple(&projection, &mut obj)?); 
                }
                Ok(serde_json::Value::Array(out))
            } else {
                Err(DbError::Invalid("only SELECT supported".into()))
            }
        }
        _ => Err(DbError::Invalid("only SELECT supported".into())),
    }
}

fn eval_simple_where(row: &serde_json::Value, expr: &sqlparser::ast::Expr) -> Result<bool> {
    match expr { 
        sqlparser::ast::Expr::BinaryOp { left, op: sqlparser::ast::BinaryOperator::Eq, right } => { 
            let (l, r) = (value_of(row, left)?, value_of(row, right)?); 
            Ok(l == r) 
        } 
        _ => Err(DbError::Invalid("WHERE supports only = in MVP".into())), 
    }
}

fn value_of(row: &serde_json::Value, expr: &sqlparser::ast::Expr) -> Result<serde_json::Value> {
    match expr { 
        sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident { value, .. }) => 
            Ok(row.get(value).cloned().unwrap_or(serde_json::Value::Null)), 
        sqlparser::ast::Expr::Value(v) => 
            Ok(lit_sql_to_json(v.clone())), 
        _ => Err(DbError::Invalid("unsupported expression".into())), 
    }
}

fn project_simple(proj: &Vec<sqlparser::ast::SelectItem>, row: &mut serde_json::Value) -> Result<serde_json::Value> {
    let obj = row.as_object().ok_or_else(|| DbError::Invalid("row not object".into()))?;
    if proj.len()==1 { 
        // Let's handle wildcard by checking if it's a wildcard pattern
        match &proj[0] {
            sqlparser::ast::SelectItem::Wildcard(_) => {
                return Ok(serde_json::Value::Object(obj.clone()));
            }
            _ => {}
        }
    }
    let mut out = serde_json::Map::new();
    for it in proj { 
        match it {
            sqlparser::ast::SelectItem::UnnamedExpr(sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident { value, .. })) => { 
                if let Some(v) = obj.get(value) { 
                    out.insert(value.clone(), v.clone()); 
                } 
            }
            sqlparser::ast::SelectItem::ExprWithAlias { 
                expr: sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident { value, .. }), 
                alias 
            } => { 
                if let Some(v) = obj.get(value) { 
                    out.insert(alias.value.clone(), v.clone()); 
                } 
            }
            _ => return Err(DbError::Invalid("projection supports identifiers only".into())), 
        } 
    }
    Ok(serde_json::Value::Object(out))
}

fn lit_sql_to_json(v: sqlparser::ast::Value) -> serde_json::Value { 
    match v {
        sqlparser::ast::Value::Number(n, _) => 
            serde_json::json!(n.parse::<f64>().unwrap()),
        sqlparser::ast::Value::SingleQuotedString(s) | sqlparser::ast::Value::DoubleQuotedString(s) => 
            serde_json::json!(s),
        sqlparser::ast::Value::Boolean(b) => 
            serde_json::json!(b),
        sqlparser::ast::Value::Null => 
            serde_json::Value::Null,
        _ => serde_json::Value::Null,
    } 
}