import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import json from "@rollup/plugin-json";

export default {
    input: 'src/type_server/index.ts',
    output: {
        file: 'type-server.js',
    },
    external: ['ts-morph'],
    plugins: [typescript(), nodeResolve(), commonjs(), json()]
};