export * from './api/param.js'

export function FnApi(): MethodDecorator {
    return (prototype, key, descriptor) => { }

}


export interface HttpApiOptions {

}

export function Http(options: HttpApiOptions): MethodDecorator {
    return (prototype, key, descriptor) => { }
}
