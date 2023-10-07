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
     * Kurit About
     */
    export const about: String
    /**
     * Kurit Console API
     * @param args
     */
    export function log(...args: any): void
    export function warn(...args: any): void
    export function error(...args: any): void
    /**
     * Markdown to HTML
     * @param name WebSite Name
     * @param contents Markdown File Contents
     * @returns {(String|Error)}
     */
    export function md_to_html(name: String, contents: String): String | Error
    /**
     * Kurit File System API
     */
    export module fs {
        export function readFile(path: String): Promise<String>
        export function writeFile(path: String, contents: String): Promise<void>
        export function removeFile(path: String): Promise<void>
        /**
         * KuritFS Version
         */
        export const version: String
    }
    /**
     * (Private) Kurit Ops API
     */
    module ops {
        /**
         * KuritOps Version
         */
        const version: String
    }
}