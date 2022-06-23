import { FnApi } from '@fnapi/api';


type Foo<T> = [T, string, number, T];

export default class TestApi {
    @FnApi()
    static async test(arg1: string): Promise<Foo<string>> {

    }
}