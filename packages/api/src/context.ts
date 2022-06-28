import { FnApiReply, FnApiRequest, InternalFnApiRequest } from "./rt/types";

/**
 * Request context. This is a magic type processed by the compiler, and does not exist on runtime.
 */
export declare class Context {
  static get<T>(provider: Provider<T>): T;
}

export class Provider<T> {
  /**
   * @internal
   */
  constructor(
    private readonly symbol: symbol,
    private readonly op: ProviderFn<T>
  ) { }

  /**
   * @internal
   */
  public async provide(req: InternalFnApiRequest, reply: FnApiReply): Promise<T> {
    return req.contexts[this.symbol] ??= this.op(req, reply);
  }
}

export function Provide<T>(op: ProviderFn<T>): Provider<T> {
  return new Provider<T>(Symbol("Provider"), op);
}

export type ProviderFn<T> = (
  req: FnApiRequest,
  reply: FnApiReply
) => Promise<T>;
