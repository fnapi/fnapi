import { FastifyReply, FastifyRequest } from "fastify";

export interface FnApiRequest {
  readonly raw: FastifyRequest;

  readonly params: ReadonlyArray<any>;

  /**
   * @internal
   */
  contexts: {
    [key: symbol]: any;
  };
}

export interface FnApiReply {
  readonly raw: FastifyReply;
}

export type FnApiHandler = (
  req: FnApiRequest,
  reply: FnApiReply
) => Promise<any>;
