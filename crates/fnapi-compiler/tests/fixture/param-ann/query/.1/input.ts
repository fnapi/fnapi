import { FnApi, Query } from '@fnapi/api';

export default class TestApi {

    @FnApi({ httpMethod: 'GET' })
    static async search(@Query() token: string): Promise<string> {

    }

    @FnApi({ httpMethod: 'GET' })
    static async list(@Query() token: string): Promise<string> {

    }
}