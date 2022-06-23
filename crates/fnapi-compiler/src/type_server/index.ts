import { Server, ServerOptions } from "@open-rpc/server-js";
import { HTTPServerTransportOptions } from "@open-rpc/server-js/build/transports/http";
import { parseOpenRPCDocument } from "@open-rpc/schema-utils-js";
import { methods, openrpcDocument } from "./methods";

const port = parseInt(process.env.PORT!);

async function start() {
    const serverOptions: ServerOptions = {
        openrpcDocument: await parseOpenRPCDocument(openrpcDocument),
        transportConfigs: [
            {
                type: "HTTPTransport",
                options: {
                    port,
                    middleware: [],
                } as HTTPServerTransportOptions,
            }
        ],
        methodMapping: methods,
    };

    console.log("Starting Server");
    const s = new Server(serverOptions);

    s.start();
    console.log('Started');
}
start();

