import { Validator } from "./validators/index.js";

export * from "./api.js";
export * from "./context.js";
export * as validators from "./validators/index.js";

export function Validate<T>(
  fieldName: string,
  validators: Validator<T>[]
): ParameterDecorator {
  return () => {};
}
