import { FnApi } from '@fnapi/api';




export default class TestApi {
    @FnApi()
    static async test([a, b, c]: [string, string, number]): Promise<string> {

    }
}