/// <reference path="../../../crates/kurit-runtime/src/lib.kurit.d.ts" />

const args = Kurit.args.slice(1) // kurit [subcommand] [...args]

switch (args[0]) { // SubCommands
    case 'build':
        if (!(args.length === 2)) { // args not found
            console.error('File Not Found')
            break
        }
        const path = args[1]
        const filename = path.substring(path.lastIndexOf('/')+1).replace('.md', '')
        const contents = Kurit.md_to_html(filename, await Kurit.fs.readFile(path))
        await Kurit.fs.writeFile(path.replace('.md', '.html'), contents)
        console.log('Finish...🚀')
        break
    case 'version':
        console.log(Kurit.version)
        break
    case 'about':
        console.log(Kurit.about)
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