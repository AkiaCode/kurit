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
      fs: { // File System
        readFile: (path) => {
          return ops.op_read_file(path)
        },
        writeFile: (path, contents) => {
          return ops.op_write_file(path, contents)
        },
        removeFile: (path) => {
          return ops.op_remove_file(path)
        },
      },
      md_to_html: (contents) => {
        return ops.op_md_to_html(contents)
      },
      get version() {
          return "0.0.0-alpha"
      },
      get args() {
        const args = ops.op_args()
        return args.split(' ') // String to Array
      },
      about() {
        return `Kurit\n\nVersion: ${Kurit.version}\nArgs: ${Kurit.args}`
      }
    }
  
  Deno = null // We Love Deno!
})(globalThis)