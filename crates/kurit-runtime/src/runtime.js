((globalThis) => {
    const core = Deno.core;
  
  
    globalThis.console = {
      log: (...args) => {
        core.print(`[KuritLog]: ${args}\n`, false)
      },
      error: (...args) => {
        core.print(`[KuritError]: ${args}\n`, true)
      },
    }

    globalThis.Kurit = {
        get version() {
            return "0.0.0-alpha"
        }
    }

})(globalThis)