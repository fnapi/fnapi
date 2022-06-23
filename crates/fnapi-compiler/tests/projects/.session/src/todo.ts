import { FnApi, Context } from '@fnapi/api';

// TODO: Session API

export interface Todo {
    id: number
    title: string
}

interface User {
    uid: string
}

/**
 * TODO: We may change this to core api
 */
interface Session {
    uid: string
}


export default class TodoApi {

    private static id: number = 0;
    private static db: Todo[] = [];

    private users: string[] = [];

    @FnApi()
    static async addTodo(title: string): Promise<number> {
        console.log(`Title: ${title}`);
        const sess = Context.get<Session>();
        console.log(sess);

        const todo: Todo = {
            id: ++this.id,
            title,
        };

        return todo.id;
    }

    @FnApi()
    static async list(): Promise<Todo[]> {
        return this.db;
    }

    @FnApi()
    static async get(id: number): Promise<Todo | null> {
        console.log(`Id: ${id}`);

        return this.db.find((todo) => todo.id === id) || null;
    }
}