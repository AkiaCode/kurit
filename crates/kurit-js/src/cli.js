/// <reference path="../../../crates/kurit-runtime/src/lib.kurit.d.ts" />

if (Kurit.args.length === 2) {
    const contents = Kurit.md_to_html(await Kurit.fs.readFile(Kurit.args[1]))
    console.log(contents)
}