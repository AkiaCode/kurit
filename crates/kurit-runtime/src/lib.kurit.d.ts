declare namespace Kurit {
    /**
     * Kurit Args
     */
    export const args: String[]
    /**
     * Kurit Version
     */
    export const version: String
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
        export async function readFile(path: String): Promise<String>
        export async function writeFile(path: String, contents: String): Promise<void>
        export async function removeFile(path: String): Promise<void>
    }
}