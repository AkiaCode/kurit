use std::{path::Path, rc::Rc};

use deno_core::error::AnyError;

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }

    async fn run_js(self, current_dir: &Path) -> Result<(), AnyError> {
        let main_module = deno_core::resolve_path("src/cli.js", current_dir)?;
        let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            extensions: vec![kurit_ops::ops_extension()],
            ..Default::default()
        });
        js_runtime
            .execute_script(
                "[kurit:runtime.js]",
                include_str!("./runtime.js").to_owned().into(),
            )
            .unwrap();
        let mod_id = js_runtime
            .load_main_module(&main_module, Some(kurit_js::CLI_CODE.to_owned().into()))
            .await?;
        let result = js_runtime.mod_evaluate(mod_id);
        js_runtime.run_event_loop(false).await?;
        result.await?
    }

    pub fn run(self, current_dir: &Path) {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        if let Err(error) = runtime.block_on(self.run_js(current_dir)) {
            eprintln!("{}", error);
        }
    }
}
