import { FnApi } from '@fnapi/api';


type Foo = [string, string, number]

export default class TestApi {
    @FnApi()
    static async test(arg1: string): Promise<Foo> {

    }
}