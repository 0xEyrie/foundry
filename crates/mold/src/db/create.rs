use std::sync::Arc;

use crate::generate::parse::{field_type_as_str, StructCtx};

use super::connect::Database;

pub fn db_gen_query_create(ctx: &StructCtx) -> String {
    let mut query = format!("CREATE TABLE {} (", ctx.label);

    for (name, type_ident) in &ctx.fields {
        query.push_str(&format!("{} {}, ", name, field_type_as_str(type_ident)));
    }

    query.truncate(query.len() - 2);
    query.push_str(");");

    query
}

pub async fn db_table_create(db: Arc<Database>, ctx: &StructCtx) {
    let drop_query = r#"
DROP TABLE IF EXISTS UniswapCall;
"#;
    let query = db_gen_query_create(ctx);

    sqlx::query(drop_query).execute(&db.pool).await.unwrap();
    sqlx::query(&query).execute(&db.pool).await.unwrap();
}
