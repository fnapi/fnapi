import { FnApi } from '@fnapi/api';


export default class TestApi {
    @FnApi()
    static async test(arg1: string): Promise<[{ foo: string }, number, { foo: number, bar?: number }]> {

    }
}