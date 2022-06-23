
export interface BaseParamOptions { }


export interface QueryOptions extends BaseParamOptions {

}

enum ParamKind {
    Query,
    Body,
    Path,
    Header
}

const parmaKindKey = Symbol('FnApi.ParamKind');

function impl(kind: ParamKind): ParameterDecorator {
    return (prototype, key, index) => {
        const desc = Object.getOwnPropertyDescriptor(prototype, key);
        Object.defineProperty(desc, parmaKindKey, {
            value: kind,
            enumerable: false,
            writable: false,
            configurable: false
        });
    }
}

export function Query(options?: QueryOptions): ParameterDecorator {
    return impl(ParamKind.Query);
}

export interface PathOptions extends BaseParamOptions {

}

export function Path(options?: PathOptions): ParameterDecorator {
    return impl(ParamKind.Path);
}


export interface HeaderOptions extends BaseParamOptions {

}

export function Header(options?: HeaderOptions): ParameterDecorator {
    return impl(ParamKind.Header);
}