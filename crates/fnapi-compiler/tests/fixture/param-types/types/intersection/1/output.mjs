import wrapApiClass from "@fnapi/api/rt/wrapApiClass.js";
import wrapFnApiConfig from "@fnapi/api/rt/wrapFnApiConfig.js";
const __fnapi_config_for_test = wrapFnApiConfig();
import '@fnapi/api';
export default wrapApiClass(class TestApi {
    static async test(_req, _reply) {
        const params = _req.params;
        let a = params[0];
        return '';
    }
}, [
    {
        ...__fnapi_config_for_test,
        name: "test",
        parameterTypes: [
            JSON.parse('{"allOf":[{"properties":{"foo":{"type":"string"}},"required":["foo"],"type":"object"},{"properties":{"bar":{"type":"number"}},"required":["bar"],"type":"object"}]}')
        ],
        returnType: JSON.parse('{"type":"string"}')
    }
]);
