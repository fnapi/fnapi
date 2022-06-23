import { Api, Context } from "@fnapi/api";


/**
 * For security issues, you should declare all your required fields.
 * 
 * Even if you return an object with full of information, only the fields declared in return type will be sent.
 * If you need a hashmap, use index signature. 
 */
export interface TodoItem {

}

export default class TodoListApi {

    @Api()
    static async all(): Promise<TodoItem[]> {
        const curUser = Context.get<User>();

    }

    @Api()
    static async search(query: string): Promise<TodoItem[]> { }



}