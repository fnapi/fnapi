
import Fastify, { FastifyInstance, FastifyServerOptions } from "fastify";


export default function createServer(
    options: FastifyServerOptions,
    files: ((fastify: FastifyInstance) => void)[]
) {
    const fastify = Fastify(options);


    for (const file of files) {
        file(fastify);
    }

    return fastify;
}