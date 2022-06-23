import { FnApi } from '@fnapi/api';


interface Foo<T> {
    foo: T

}

export default class TestApi {
    @FnApi()
    static async test(arg1: string): Promise<Foo<string>> {

    }
}