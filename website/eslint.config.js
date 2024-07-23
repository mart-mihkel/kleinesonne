import js from "@eslint/js";
import ts from "typescript-eslint";
import svelte from "eslint-plugin-svelte";
import prettier from "eslint-config-prettier";
import globals from "globals";

/** @type {import('eslint').Linter.FlatConfig[]} */
export default [
    prettier,
    js.configs.recommended,
    ...ts.configs.recommended,
    ...svelte.configs["flat/recommended"],
    ...svelte.configs["flat/prettier"],
    {
        languageOptions: {
            globals: {
                ...globals.browser,
                ...globals.node,
            },
        },
    },
    {
        files: ["**/*.svelte"],
        languageOptions: {
            parserOptions: {
                parser: ts.parser,
            },
        },
    },
    {
        ignores: ["build/", ".svelte-kit/"],
    },
];
