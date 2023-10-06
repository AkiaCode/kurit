use deno_core::{
    error::{generic_error, AnyError},
    op2, OpState,
};

#[op2]
#[string]
pub fn op_args(_state: &mut OpState) -> String {
    let args = std::env::args();
    return args.collect::<Vec<String>>().join(" ");
}

#[op2]
#[string]
pub fn op_md_to_html(#[string] contents: &str) -> Result<String, AnyError> {
    return markdown::to_html_with_options(contents, &markdown::Options::gfm())
        .or_else(|err| Err(generic_error(err)));
}

#[op2(async)]
#[string]
pub async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op2(async)]
#[string]
pub async fn op_write_file(
    #[string] path: String,
    #[string] contents: String,
) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op2(async)]
#[string]
pub async fn op_remove_file(#[string] path: String) -> Result<(), AnyError> {
    tokio::fs::remove_file(path).await?;
    Ok(())
}
