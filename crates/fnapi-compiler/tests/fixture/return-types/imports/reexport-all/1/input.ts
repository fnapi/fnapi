import { FnApi } from '@fnapi/api';
import * as lib from './lib';


export default class TestApi {
    @FnApi()
    static async test(arg1: string): Promise<lib.Foo> {

    }
}