use deno_core::{op2, OpState};

#[op2]
#[string]
pub fn op_args(_state: &mut OpState) -> String {
    let args = std::env::args();
    return args.collect::<Vec<String>>().join(" ");
}
