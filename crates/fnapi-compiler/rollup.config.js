import typescript from '@rollup/plugin-typescript';


export default {
    input: 'src/type_server/index.ts',
    output: {
        file: 'bundle.js',
        format: 'cjs'
    },
    plugins: [typescript()]
};