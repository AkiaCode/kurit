fn main() -> Result<(), std::io::Error> {
    let kurit = kurit_runtime::Runtime::new();
    kurit.run(std::env::current_dir()?.join("crates/kurit-js").as_path());

    Ok(())
}
