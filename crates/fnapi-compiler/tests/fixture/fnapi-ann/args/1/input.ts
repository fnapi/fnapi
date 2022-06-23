import { FnApi } from '@fnapi/api';

export default class TestApi {

    @FnApi({ httpMethod: 'GET' })
    static async test(arg1: string): Promise<string> {

    }
}