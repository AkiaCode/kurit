declare namespace Kurit {
    /**
     * Kurit Args
     */
    export declare const args: Array<String>
    /**
     * Kurit Version
     */
    export declare const version: Array<String>
    /**
     * Markdown to HTML
     * @param contents Markdown File Contents
     * @returns {(String|Error)}
     */
    export declare function md_to_html(contents: String): String | Error
    /**
     * Kurit File System API
     */
    export declare namespace fs {
        export declare function readFile(path: String): String | Error
        export declare function writeFile(path: String, contents: String): void | Error
        export declare function removeFile(path: String): void | Error
    }
}