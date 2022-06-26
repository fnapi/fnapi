import __client from "@fnapi/api/client/nodejs.js";
export const TestApi = {
    async test () {
        return __client.invoke("TestApi", "test", arguments);
    }
};
