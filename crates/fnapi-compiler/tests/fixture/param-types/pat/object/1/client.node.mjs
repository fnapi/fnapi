import __client from "@fnapi/api/client/nodejs.js";
export default {
    async test () {
        return __client.invoke("TestApi", "test", arguments);
    }
};
