import expect from 'expect';
import { rt } from '../src/index.server.js';
import TodoApi from '../src/todo.server.js';

const server = rt.createServer({
    logger: true,
}, [
    TodoApi
]);
const resp = await server.inject({
    method: 'POST',
    url: '/TodoApi/addTodo',
    payload: {
        p0: 'my todo',
    }
})
expect(resp.statusCode).toBe(201);
expect(resp.body).toBe(1);

