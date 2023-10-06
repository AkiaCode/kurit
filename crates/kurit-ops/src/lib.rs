use deno_core::{
    error::{generic_error, AnyError},
    op2, Extension, Op, OpState,
};
mod fs;

pub fn ops_extension() -> Extension {
    Extension {
        name: "kuritjs",
        ops: vec![
            op_args::DECL,
            op_md_to_html::DECL,
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
fn op_args(_state: &mut OpState) -> String {
    let args = std::env::args();
    return args.collect::<Vec<String>>().join(" ");
}

#[op2]
#[string]
fn op_md_to_html(#[string] contents: &str) -> Result<String, AnyError> {
    return markdown::to_html_with_options(contents, &markdown::Options::gfm())
        .or_else(|err| Err(generic_error(err)));
}
