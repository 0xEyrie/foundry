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
static IMPL_NEW_INDICATOR: &str = "// Impl new";
static SRC_PATH: &str = "crates/mold/a.sol";
static SPEC_VM_PATH: &str = "crates/mold/test.rs";
static INDEX_PATH: &str = "crates/mold/test.rs";
static SPEC_LIB_PATH: &str = "crates/mold/test.rs";
//static SPEC_VM_PATH: &str = "crates/cheatcodes/spec/src/vm.rs";
//static INDEX_PATH: &str = "crates/cheatcodes/src/index.rs";
//static SPEC_LIB_PATH: &str = "crates/cheatcodes/spec/src/lib.rs";

fn file_read(buf: &mut String, path: &str) -> Result<()> {
    let mut file = File::open(path)?;
    file.read_to_string(buf)?;

    Ok(())
}

fn dst_structs_insert(dst: &mut String, src: &String, struct_ctxs: &Vec<StructCtx>) {
    if let Some(pos) = dst.find(STRUCTS_INDICATOR) {
        for ctx in struct_ctxs {
            let mut struct_str = src[ctx.start..ctx.end + 1].to_string();
            struct_str.push_str("\n");
            dst.insert_str(pos, &struct_str);
        }

        fs::write(SPEC_VM_PATH, dst).unwrap();
    } else {
        println!("Not found struct");
    }
}

fn dst_functions_insert(dst: &mut String, struct_ctxs: &Vec<StructCtx>) {
    if let Some(pos) = dst.find(FN_INDICATOR) {
        for ctx in struct_ctxs {
            let mut fn_str = sol_function!(ctx.label).to_string();
            fn_str.push_str("\n");
            dst.insert_str(pos, &fn_str);
        }

        fs::write(SPEC_VM_PATH, dst).unwrap();
    } else {
        println!("Not found fn");
    }
}

fn dst_handler_insert(dst: &mut String, src: &String, struct_ctxs: &Vec<StructCtx>) {
    if let Some(mut pos) = dst.find(HANDLER_INDICATOR) {
        for ctx in struct_ctxs {
            let mut handler_str = sol_handler!(ctx.label, ctx.fields);
            handler_str.push_str("\n");
            dst.insert_str(pos, &handler_str);
        }

        fs::write(INDEX_PATH, dst).unwrap();
    } else {
        println!("Not found handler");
    }
}

fn dst_impl_new_insert(dst: &mut String, struct_ctxs: &Vec<StructCtx>) {
    if let Some(pos) = dst.find(IMPL_NEW_INDICATOR) {
        for ctx in struct_ctxs {
            let mut struct_str = sol_impl_new!(ctx.label).to_string();
            struct_str.push_str("\n");
            dst.insert_str(pos, &struct_str);
        }

        fs::write(SPEC_LIB_PATH, dst).unwrap();
    } else {
        println!("Not found impl new");
    }
}

impl GenerateArgs {
    pub async fn run(self) -> Result<()> {
        let db = connect_db().await;

        let mut src = String::new();
        file_read(&mut src, SRC_PATH).unwrap();

        let mut struct_ctxs = Vec::new();
        parse(&mut src, &mut struct_ctxs);

        let mut spec_vm = String::new();
        file_read(&mut spec_vm, SPEC_VM_PATH).unwrap();
        dst_structs_insert(&mut spec_vm, &mut src, &struct_ctxs);

        let mut spec_vm = String::new();
        file_read(&mut spec_vm, SPEC_VM_PATH).unwrap();
        dst_functions_insert(&mut spec_vm, &struct_ctxs);

        let mut spec_lib = String::new();
        file_read(&mut spec_lib, SPEC_LIB_PATH).unwrap();
        dst_impl_new_insert(&mut spec_lib, &struct_ctxs);

        let mut index = String::new();
        file_read(&mut index, INDEX_PATH).unwrap();
        dst_handler_insert(&mut index, &mut src, &struct_ctxs);

        db_table_create(db.clone(), &struct_ctxs[0]).await;

        Ok(())
    }
}
