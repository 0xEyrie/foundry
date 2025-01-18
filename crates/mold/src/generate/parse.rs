use std::{collections::BTreeMap, process::abort};

pub enum FieldType {
    Varchar,
    Numeric,
}

pub fn field_type_as_str<'a>(type_ident: &FieldType) -> &'a str {
    match type_ident {
        FieldType::Varchar => "VARCHAR",
        FieldType::Numeric => "NUMERIC",
    }
}

pub struct StructCtx {
    pub start: usize,
    pub end: usize,
    pub label: String,

    pub fields: BTreeMap<String, FieldType>,
}

pub fn parse(src: &mut String, struct_offsets: &mut Vec<StructCtx>) {
    let mut src_idx = 0;
    loop {
        match src.chars().nth(src_idx) {
            Some(_) => {
                parse_comments(src, &mut src_idx);
                parse_pragma(src, &mut src_idx);
                parse_struct(src, &mut src_idx, struct_offsets);
            }
            None => return,
        };

        src_idx += 1;
    }
}

fn parse_comments(src: &mut String, src_idx: &mut usize) {
    if src.chars().nth(*src_idx).unwrap() == '/' {
        if src.chars().nth(*src_idx + 1).unwrap() == '/' {
            *src_idx += 1;

            loop {
                if src.chars().nth(*src_idx).unwrap() == '\n' {
                    break;
                }
                *src_idx += 1;
            }
        }
    }
}

fn parse_pragma(src: &mut String, src_idx: &mut usize) {
    if *src_idx + 6 > src.len() {
        return;
    }

    let src_curr = &src[*src_idx..*src_idx + 6];

    if src_curr.starts_with("pragma") {
        *src_idx += 6;

        loop {
            if src.chars().nth(*src_idx).unwrap() == '\n' {
                break;
            }
            *src_idx += 1;
        }
    }
}

fn parse_translate_type(sol_type: &str) -> FieldType {
    match sol_type {
        "uint" => return FieldType::Numeric,
        "uint8" => return FieldType::Numeric,
        "uint16" => return FieldType::Numeric,
        "uint32" => return FieldType::Numeric,
        "uint64" => return FieldType::Numeric,
        "uint128" => return FieldType::Numeric,
        "uint256" => return FieldType::Numeric,

        "address" => return FieldType::Varchar,
        _ => {
            println!("Translate type error for {sol_type:?}");
            abort();
        }
    }
}

fn parse_struct(src: &mut String, src_idx: &mut usize, struct_ctxs: &mut Vec<StructCtx>) {
    if *src_idx + 6 > src.len() {
        return;
    }

    let src_curr = &src[*src_idx..*src_idx + 6];

    if src_curr.starts_with("struct") {
        let mut ctx =
            StructCtx { start: 0, end: 0, label: "".to_string(), fields: BTreeMap::new() };
        ctx.start = *src_idx;

        *src_idx += 7;

        loop {
            let char = match src.chars().nth(*src_idx) {
                Some(c) => c,
                None => return,
            };

            if char == '{' {
                let label = &src[ctx.start + 7..*src_idx - 1];
                ctx.label = label.to_string();
                break;
            }

            *src_idx += 1;
        }

        let mut type_start = *src_idx;
        let mut type_ident = "null";
        let mut label_start = 0;

        loop {
            let char = match src.chars().nth(*src_idx) {
                Some(c) => c,
                None => return,
            };

            if char == '\0' {
                println!("Unclosed brackets");
                abort();
            }

            if char == '}' {
                ctx.end = *src_idx;
                break;
            }

            if char == ' ' {
                type_ident = &src[type_start..*src_idx];
                label_start = *src_idx + 1;
            }

            if char == ';' {
                let type_translated = parse_translate_type(type_ident);
                let label = &src[label_start..*src_idx];
                ctx.fields.insert(label.to_string(), type_translated);
            }

            if char == '\n' {
                type_start = *src_idx + 2;
            }

            *src_idx += 1;
        }

        struct_ctxs.push(ctx);
    }
}
