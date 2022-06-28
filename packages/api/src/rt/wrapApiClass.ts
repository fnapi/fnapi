import { FastifyInstance } from "fastify";

import { FnApiHandler, FnApiReply, InternalFnApiRequest } from "./types";

type JsonSchema = object;

export interface MethodDescriptor {
  readonly name: string;
  readonly httpMethod: string;

  readonly returnType: JsonSchema;
  readonly parameterTypes: JsonSchema[];
}

function parseParams(params: object): any[] {
  const arr = new Array(Object.keys(params).length);
  for (const [key, value] of Object.entries(params)) {
    arr[parseInt(key.substring(1))] = value;
  }
  return arr;
}

/**
 * @internal This is helper for generated codes.
 */
export default function wrapApiClass(
  cls: any,
  methods: MethodDescriptor[]
): (fastify: FastifyInstance) => void {
  const className = cls.name as string;
  if (!className) {
    throw new Error(`@FnApi requires a named class`);
  }

  return (fastify: FastifyInstance) => {
    for (const methodDesc of methods) {
      const bodyJsonSchema =
        methodDesc.parameterTypes.length > 0
          ? {
              type: "object",
              properties: Object.fromEntries(
                methodDesc.parameterTypes.map((ty, idx) => [`p${idx}`, ty])
              ),
            }
          : undefined;

      const responseSchema = {
        "2xx": methodDesc.returnType,
      };

      console.log("Parameters:", bodyJsonSchema);
      console.log(`Response:`, JSON.stringify(responseSchema));

      fastify.route({
        // TODO:
        method: "POST",
        url: `/${className}/${methodDesc.name}`,
        schema: {
          body: bodyJsonSchema,
          response: responseSchema,
        },
        handler: async (req, reply) => {
          const params = Object.freeze(parseParams(req.body as object));

          const fReq: InternalFnApiRequest = {
            raw: req,
            params,
            contexts: {},
          };
          const fReply: FnApiReply = {
            raw: reply,
          };
          const handler = cls[methodDesc.name] as FnApiHandler;
          const returnValue = await handler.call(cls, fReq, fReply);

          reply.send(returnValue);
        },
      });
    }
  };
}
