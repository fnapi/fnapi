import { FnApi } from '@fnapi/api';

type A = {
    foo: string
}

type B = {
    bar: number
}


export default class TestApi {
    @FnApi()
    static async test(a: A & B): Promise<string> {
        return '';
    }
}