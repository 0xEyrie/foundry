use clap::{command, Parser};
use eyre::Result;
use foundry_cli::opts::GlobalArgs;
use parse::{parse, StructCtx};
use std::{
    fs::{self, File},
    io::Read,
};

use crate::db::{connect::connect_db, create::db_table_create};

#[macro_use]
pub mod macros;
pub mod parse;

/// CLI arguments for `mold generate`.
#[derive(Clone, Debug, Parser)]
#[command(next_help_heading = "Test options")]
pub struct GenerateArgs {
    // Include global options for users of this struct.
    #[command(flatten)]
    pub global: GlobalArgs,
}

static STRUCTS_INDICATOR: &str = "// Structs";
static FN_INDICATOR: &str = "// Functions";
static HANDLER_INDICATOR: &str = "// Handlers";
static SRC_PATH: &str = "crates/mold/a.sol";
static DST_PATH: &str = "crates/cheatcodes/spec/src/vm.rs";

fn file_read(buf: &mut String, path: &str) -> Result<()> {
    let mut file = File::open(path)?;
    file.read_to_string(buf)?;

    Ok(())
}

fn dst_structs_insert(dst: &mut String, src: &String, struct_ctxs: &Vec<StructCtx>) {
    if let Some(mut pos) = dst.find(STRUCTS_INDICATOR) {
        loop {
            if src.chars().nth(pos).unwrap() == '\n' {
                break;
            }
            pos += 1;
        }

        for ctx in struct_ctxs {
            let mut struct_str = src[ctx.start..ctx.end + 1].to_string();
            struct_str.push_str("\n");
            dst.insert_str(pos, &struct_str);
        }

        fs::write(DST_PATH, dst).unwrap();
    } else {
        println!("Not found struct");
    }
}

fn dst_functions_insert(dst: &mut String, src: &String, struct_ctxs: &Vec<StructCtx>) {
    if let Some(mut pos) = dst.find(FN_INDICATOR) {
        loop {
            if src.chars().nth(pos).unwrap() == '\n' {
                break;
            }
            pos += 1;
        }

        for ctx in struct_ctxs {
            let mut fn_str = sol_function!(ctx.label, ctx.label).to_string();
            fn_str.push_str("\n");
            dst.insert_str(pos, &fn_str);
        }

        fs::write(DST_PATH, dst).unwrap();
    } else {
        println!("Not found fn");
    }
}

fn dst_handler_insert(dst: &mut String, src: &String, struct_ctxs: &Vec<StructCtx>) {
    if let Some(mut pos) = dst.find(HANDLER_INDICATOR) {
        loop {
            if src.chars().nth(pos).unwrap() == '\n' {
                break;
            }
            pos += 1;
        }

        for ctx in struct_ctxs {
            let mut handler_str = sol_handler!(ctx.label, ctx.fields);
            handler_str.push_str("\n");
            dst.insert_str(pos, &handler_str);
        }

        fs::write(DST_PATH, dst).unwrap();
    } else {
        println!("Not found handler");
    }
}

impl GenerateArgs {
    pub async fn run(self) -> Result<()> {
        let db = connect_db().await;

        let mut src = String::new();
        file_read(&mut src, SRC_PATH).unwrap();

        let mut dst = String::new();
        file_read(&mut dst, DST_PATH).unwrap();

        let mut struct_ctxs = Vec::new();
        parse(&mut src, &mut struct_ctxs);

        db_table_create(db.clone(), &struct_ctxs[0]).await;

        let a = sol_handler!(struct_ctxs[0].label, struct_ctxs[0].fields);
        println!("A: {a:?}");

        dst_structs_insert(&mut dst, &mut src, &struct_ctxs);
        dst_functions_insert(&mut dst, &mut src, &struct_ctxs);
        dst_handler_insert(&mut dst, &mut src, &struct_ctxs);

        Ok(())
    }
}
