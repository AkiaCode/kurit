declare namespace Kurit {
    /**
     * Kurit Args
     */
    export const args: Array<String>
    /**
     * Kurit Version
     */
    export const version: Array<String>
    /**
     * Markdown to HTML
     * @param contents Markdown File Contents
     * @returns {(String|Error)}
     */
    export function md_to_html(contents: String): String | Error
    /**
     * Kurit File System API
     */
    export namespace fs {
        export function readFile(path: String): String | Error
        export function writeFile(path: String, contents: String): void | Error
        export function removeFile(path: String): void | Error
    }
}