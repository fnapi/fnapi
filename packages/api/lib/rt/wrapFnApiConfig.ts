import { MethodDescriptor } from "./wrapApiClass";

export type WrappedConfig = Omit<
  MethodDescriptor,
  | "name"
  | "filterReturnValue"
  | "bodyJsonSchema"
  | "queryStringJsonSchema"
  | "headersJsonSchema"
>;

export default function wrapFnApiConfig(arg: object): WrappedConfig {
  return {
    httpMethod: "POST",
    ...(arg as any),
  };
}
