#[macro_export]
macro_rules! sol_function {
    ($fn_name:expr, $data_type:expr) => {
        format!(
            "#[cheatcode(group = Evm, safety = Safe)]
            function {}({} calldata data) external;",
            $fn_name, $data_type
        )
    };
}

#[macro_export]
macro_rules! sol_handler {
    ($label:expr, $fields:expr) => {{
        let mut field_binds = String::new();
        let mut db_binds = String::new();

        for (_, name) in $fields.keys().enumerate() {
            field_binds.push_str(&format!("{}, ", name));
            db_binds.push_str(&format!(".bind({})\n", name));
        }

        if !field_binds.is_empty() {
            field_binds.truncate(field_binds.len() - 2);
        }

        let mut placeholder = String::new();
        for n in 1..$fields.len() + 1 {
            placeholder.push_str(&format!("${n}, "));
        }
        placeholder.truncate(placeholder.len() - 2);

        format!(
            r#"
impl Cheatcode for {label} {{
    fn apply(&self, _state: &mut Cheatcodes) -> Result {{
        let Self {{ data }} = self;

        let {label} {{ {field_binds} }} = data;

        let query = format!("
        INSERT INTO {label}
        VALUES
        ({placeholder})
        ");

        sqlx::query(&query)
            {db_binds}
            .execute(&db.pool)
            .await
            .unwrap();

        Ok(Default::default())
    }}
}}"#,
            label = $label,
            field_binds = field_binds,
            placeholder = placeholder,
            db_binds = db_binds,
        )
    }};
}
