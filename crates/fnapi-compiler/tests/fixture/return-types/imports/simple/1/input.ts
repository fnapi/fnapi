import { FnApi } from '@fnapi/api';
import { Foo } from './lib';


export default class TestApi {
    @FnApi()
    static async test(arg1: string): Promise<Foo> {

    }
}