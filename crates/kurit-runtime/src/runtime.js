((globalThis) => {
    const core = Deno.core;
    const { ops } = core;
  
    globalThis.console = {
      log: (...args) => {
        core.print(`${args}\n`, false)
      },
      error: (...args) => {
        core.print(`KuritError: ${args}\n`, true)
      },
    }

    globalThis.Kurit = {
      fs: { // Kurit File System
        readFile: (path) => {
          return ops.op_read_file(path)
        },
        writeFile: (path, contents) => {
          return ops.op_write_file(path, contents)
        },
        removeFile: (path) => {
          return ops.op_remove_file(path)
        },
        get version() { // Kurit FS Version
          return ops.op_fs_version()
        },
      },
      ops: { // Kurit Ops
        get version() { // Kurit Ops Version
          return ops.op_version()
        }
      },
      // Kurit Console
      log: (...args) => {
        core.print(`[kurit-log]: ${args}\n`, false)
      },
      warn: (...args) => {
        core.print(`[kurit-warn]: ${args}\n`, false)
      },
      error: (...args) => {
        core.print(`[kurit-error]: ${args}\n`, true)
      },
      // Kurit HTML
      md_to_html: (name, contents) => {
        return ops.op_md_to_html(name, contents)
      },
      // Kurit Utils
      get version() { // Kurit Runtime Version
        return "0.1.0"
      },
      get args() {
        const args = ops.op_args()
        return args.split(' ') // String to Array
      },
      get about() {
        return `Kurit Version\n\nKuritFS: ${Kurit.fs.version}\nKuritOps: ${Kurit.ops.version}\nKuritRuntime: ${Kurit.version}`
      }
    }
  
  delete Deno // We Love Deno!
})(globalThis)