use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
    let sql = "SELECT * FROM users WHERE id = 1";
    let stmts = Parser::parse_sql(&GenericDialect, sql).unwrap();
    if let sqlparser::ast::Statement::Query(q) = &stmts[0] {
        if let sqlparser::ast::SetExpr::Select(sel) = &*q.body {
            println!("Selection type: {:?}", std::any::type_name_of_val(&sel.selection));
        }
    }
}