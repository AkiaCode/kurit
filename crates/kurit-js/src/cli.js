/// <reference path="../../../crates/kurit-runtime/src/lib.kurit.d.ts" />

const args = Kurit.args.slice(1) // kurit [subcommand] [...args]

switch (args[0]) { // SubCommands
    case 'build':
        if (!(args.length === 2)) { // args not found
            console.error('File Not Found')
            break
        }
        const contents = Kurit.md_to_html(await Kurit.fs.readFile(args[1]))
        console.log(contents)
        break
    case 'version':
        console.log(Kurit.version)
        break
    case 'about':
        console.log(Kurit.about())
        break
    case 'help': // Help Command
    default:
        console.log(`Kurit ${Kurit.version}

Commands:
    build [path] - Markdown to HTML
    version - Kurit Version
    about - Kurit About
`)
}