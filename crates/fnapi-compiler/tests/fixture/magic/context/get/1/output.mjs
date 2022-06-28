import wrapApiClass from "@fnapi/api/rt/wrapApiClass.js";
import wrapFnApiConfig from "@fnapi/api/rt/wrapFnApiConfig.js";
const __fnapi_config_for_test = wrapFnApiConfig();
import { provide } from "@fnapi/api";
export default wrapApiClass(class TestApi {
    static async test(_req, _reply) {
        const params = _req.params;
        let arg1 = params[0];
        const user = UserProvider.get(_req, _reply);
    }
}, [
    {
        ...__fnapi_config_for_test,
        name: "test",
        parameterTypes: [
            JSON.parse('{"type":"string"}')
        ],
        returnType: JSON.parse('{"type":"string"}')
    }
]);
const UserProvider = provide();
