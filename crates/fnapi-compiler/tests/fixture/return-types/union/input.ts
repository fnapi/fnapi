import { FnApi } from '@fnapi/api';

type A = {
    foo: string
}

type B = {
    bar: number
}


export default class TestApi {
    @FnApi()
    static async test(a: string): Promise<A | B> {
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
}