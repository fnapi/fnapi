import { FnApiReply, FnApiRequest } from "./rt/types";

/**
 * Request context. This is a magic type processed by the compiler, and does not exist on runtime.
 */
export declare class Context {
  static get<T>(provider: Provider<T>): T;
}

export class Provider<T> {
  #op: ProvideFn<T>;
  #symbol: unique symbol;
}

export function Provide<T>(op: ProvideFn<T>): Provider<T> {}

export type ProvideFn<T> = (req: FnApiRequest, res: FnApiReply) => Promise<T>;

/**
 * Request context. This is a magic type processed by the compiler, and does not exist on runtime.
 */
export declare class ServerConfig {
  static get<T = never>(): T;
}
