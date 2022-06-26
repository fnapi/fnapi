import __client from "@fnapi/api/client/web.js";
export const TestApi = {
    async test () {
        return __client.invoke("TestApi", "test", arguments);
    }
};
