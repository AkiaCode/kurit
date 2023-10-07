use deno_core::{
    error::{generic_error, AnyError},
    op2, Extension, Op, OpState,
};
use kurit_template::{KuritDefault, Template};
use markdown::Options;
mod fs;

pub fn ops_extension() -> Extension {
    Extension {
        name: "kuritjs",
        ops: vec![
            op_version::DECL,
            op_args::DECL,
            op_md_to_html::DECL,
            crate::fs::op_fs_version::DECL,
            crate::fs::op_read_file::DECL,
            crate::fs::op_write_file::DECL,
            crate::fs::op_remove_file::DECL,
        ]
        .into(),
        ..Default::default()
    }
}

#[op2]
#[string]
fn op_version() -> String {
    return "0.1.0-beta".into();
}

#[op2]
#[string]
fn op_args(_state: &mut OpState) -> String {
    let args = std::env::args();
    return args.collect::<Vec<String>>().join(" ");
}

#[op2]
#[string]
fn op_md_to_html(#[string] name: String, #[string] contents: &str) -> Result<String, AnyError> {
    let html = markdown::to_html_with_options(
        contents,
        &Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..markdown::CompileOptions::default()
            },
            ..Options::default()
        },
    )
    .or_else(|err| Err(generic_error(err)))?;
    // TODO: Change Template API
    return Ok(KuritDefault::html(name, html));
}
