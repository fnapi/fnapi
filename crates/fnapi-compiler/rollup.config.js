import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import typescript from "@rollup/plugin-typescript";
import * as path from "node:path";
import { env } from "node:process";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default {
  input: path.join(__dirname, "src/type_server/index.ts"),
  output: {
    file: path.join(env.OUT_DIR, "type-server.js"),
    format: "cjs",
  },
  external: ["ts-morph"],
  plugins: [typescript(), nodeResolve(), commonjs(), json()],
};
