((globalThis) => {
    const core = Deno.core;
    const { ops } = core;
  
    globalThis.console = {
      log: (...args) => {
        core.print(`[kurit-log]: ${args}\n`, false)
      },
      error: (...args) => {
        core.print(`[kurit-error]: ${args}\n`, true)
      },
    }

    globalThis.Kurit = {
        get version() {
            return "0.0.0-alpha"
        },
        get args () {
          const args = ops.op_args()
          return args.split(' ') // String to Array
        },
        about() {
          return `Kurit\n\nVersion: ${Kurit.version}\nArgs: ${Kurit.args}`
        }
    }
  
  Deno = null
})(globalThis)