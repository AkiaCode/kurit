use std::path::Path;

use deno_core::{
    error::{bad_resource_id, generic_error, AnyError},
    op2, Extension, Op, OpState,
};
use kurit_template::{KuritDefault, Template, Templates};
use markdown::Options;
mod fs;

pub fn ops_extension() -> Extension {
    Extension {
        name: "kuritjs",
        ops: vec![
            op_version::DECL,
            op_args::DECL,
            op_md_to_html::DECL,
            op_template::DECL,
            op_devserver::DECL,
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
    "0.1.0-beta".into()
}

#[op2]
#[string]
fn op_args(_state: &mut OpState) -> String {
    let args = std::env::args();
    args.collect::<Vec<String>>().join(" ")
}

#[op2]
#[string]
fn op_md_to_html(
    _state: &mut OpState,
    #[string] name: String,
    #[string] contents: &str,
) -> Result<String, AnyError> {
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
    .map_err(generic_error)?;
    // TODO: Change Template API
    //if let Some(tmpl) = state.try_take::<Templates>() {
    //   Templates::to_tmpl(tmpl).html(name, html);
    //}
    Ok(KuritDefault::new().html(name, html))
}

#[op2(fast)]
fn op_template(state: &mut OpState, #[string] name: String) -> Result<(), AnyError> {
    if let Some(name) = kurit_template::Templates::check(name) {
        state.put::<Templates>(name);
        Ok(())
    } else {
        Err(bad_resource_id())
    }
}

#[op2(fast)]
fn op_devserver(#[string] path: String) -> Result<(), AnyError> {
    let path = if let Some(path) = Path::new(&path).parent() {
        path.to_string_lossy()
    } else {
        std::borrow::Cow::Borrowed("./")
    };
    kurit_devserver::run("localhost", 4101, &path, false, "\n\rServer: Kurit");
    Ok(())
}
