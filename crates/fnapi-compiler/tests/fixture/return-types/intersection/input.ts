import { FnApi } from '@fnapi/api';

type A = {
    foo: string
}

type B = {
    bar: number
}


export default class TestApi {
    @FnApi()
    static async test(a: string): Promise<A & B> {
        return {
            foo: 'foo',
            bar: 1
        }
    }
}