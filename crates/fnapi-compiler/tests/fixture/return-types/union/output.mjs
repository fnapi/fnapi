import wrapApiClass from "@fnapi/api/lib/rt/wrapApiClass.js";
import wrapFnApiConfig from "@fnapi/api/lib/rt/wrapFnApiConfig.js";
const __fnapi_config_for_test = wrapFnApiConfig();
import '@fnapi/api';
export default wrapApiClass(class TestApi {
    static async test(_req, _reply) {
        const params = _req.params;
        let a = params[0];
        if (a === '') {
            return {
                foo: 'test'
            };
        } else {
            return {
                bar: 1
            };
        }
    }
}, [
    {
        ...__fnapi_config_for_test,
        name: "test",
        parameterTypes: [
            JSON.parse('{"type":"string"}')
        ],
        returnType: JSON.parse('{"oneOf":[{"properties":{"foo":{"type":"string"}},"required":["foo"],"type":"object"},{"properties":{"bar":{"type":"number"}},"required":["bar"],"type":"object"}]}')
    }
]);
