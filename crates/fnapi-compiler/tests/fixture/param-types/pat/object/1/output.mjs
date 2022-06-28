import wrapApiClass from "@fnapi/api/lib/rt/wrapApiClass.js";
import wrapFnApiConfig from "@fnapi/api/lib/rt/wrapFnApiConfig.js";
const __fnapi_config_for_test = wrapFnApiConfig();
import '@fnapi/api';
export default wrapApiClass(class TestApi {
    static async test(_req, _reply) {
        const params = _req.params;
        let { foo  } = params[0];
    }
}, [
    {
        ...__fnapi_config_for_test,
        name: "test",
        parameterTypes: [
            JSON.parse('{"properties":{"foo":{"type":"string"}},"required":["foo"],"type":"object"}')
        ],
        returnType: JSON.parse('{"type":"string"}')
    }
]);
