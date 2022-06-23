import { FnApi } from '@fnapi/api';


export default class TestApi {
    @FnApi()
    static async test({ foo }: { foo?: string }): Promise<string> {

    }
}