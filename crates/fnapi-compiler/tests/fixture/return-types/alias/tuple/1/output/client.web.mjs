import __client from "@fnapi/api/client/web.js";
export default {
    TestApi: {
        async test () {
            return __client.invoke("TestApi", "test", arguments);
        }
    }
};