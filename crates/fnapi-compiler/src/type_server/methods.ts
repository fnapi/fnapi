import { OpenrpcDocument } from "@open-rpc/meta-schema";
import { MethodMapping } from "@open-rpc/server-js/build/router";
import { Project, ClassDeclaration, ts, Type, Symbol } from "ts-morph";

const project = process.env.TS_CONFIG_PATH ? new Project({
    tsConfigFilePath: process.env.TS_CONFIG_PATH,
}) : new Project({});

if (process.env.TS_FILES) {
    const files = process.env.TS_FILES.split(';');
    project.addSourceFilesAtPaths(files);
}

interface BaseSType {
    kind: string
}

interface SUnionType extends BaseSType {
    kind: 'union'
    types: SType[]
}

interface SIntersectionType extends BaseSType {
    kind: 'intersection'
    types: SType[]
}

interface SArrayType extends BaseSType {
    kind: 'array'
    elem: SType
}

interface STupleType extends BaseSType {
    kind: 'tuple'
    elems: SType[]
}

interface SKeywordType extends BaseSType {
    kind: 'keyword'
    keyword: string
}

interface SObjectType extends BaseSType {
    kind: 'object'
    members: SProperty[]
}

type STypeElement = SProperty;

interface SProperty {
    kind: 'property'
    name: string
    type: SType
    optional: boolean
}

/**
 * Serialized type.
 */
type SType = SUnionType | SIntersectionType | SArrayType | STupleType | SKeywordType | SObjectType

function serializeProperty(s: Symbol): SProperty {
    const node = s.getValueDeclarationOrThrow();

    return {
        kind: 'property',
        name: s.getName(),
        type: serializeType(s.getTypeAtLocation(node)),
        optional: s.hasFlags(ts.SymbolFlags.Optional),
    }
}


function serializeType(t: Type): SType {



    if (t.isUnion()) {
        return {
            kind: 'union',
            types: t.getUnionTypes().map(serializeType)
        }
    }

    if (t.isIntersection()) {
        return {
            kind: 'intersection',
            types: t.getIntersectionTypes().map(serializeType)
        }
    }

    if (t.isArray()) {
        return {
            kind: 'array',
            elem: serializeType(t.getArrayElementTypeOrThrow())
        }
    }

    if (t.isTuple()) {
        return {
            kind: 'tuple',
            elems: t.getTupleElements().map(serializeType)
        }
    }

    if (t.isNull()) {
        return {
            kind: 'keyword',
            keyword: 'null',
        }
    }

    if (t.isUndefined()) {
        return {
            kind: 'keyword',
            keyword: 'undefined',
        }
    }

    if (t.isString()) {
        return {
            kind: 'keyword',
            keyword: 'string'
        }
    }
    if (t.isNumber()) {
        return {
            kind: 'keyword',
            keyword: 'number'
        }
    }
    if (t.isBoolean()) {
        return {
            kind: 'keyword',
            keyword: 'boolean'
        }
    }

    if (t.isObject()) {
        return {
            kind: 'object',
            members: t.getApparentProperties().map(serializeProperty)
        }
    }


    console.log(`Unhandled type: ${t.getText()}`);
    return `Unhandled type: ${t.getText()}` as any;
}

export const methods: MethodMapping = {
    queryTypesOfMethod: async (filename: string, methodName: string) => {
        const sf = project.getSourceFileOrThrow(filename);
        // console.log(sf);
        const defaultExport = sf.getStatementOrThrow(
            s => ts.isClassDeclaration(s.compilerNode) &&
                !!s.compilerNode.modifiers &&
                s.compilerNode.modifiers.some(m => m.kind === ts.SyntaxKind.DefaultKeyword) &&
                s.compilerNode.modifiers.some(m => m.kind === ts.SyntaxKind.ExportKeyword)
        ) as ClassDeclaration;

        const method = defaultExport.getMethods().find(m => m.getName() === methodName);
        if (!method) {
            throw new Error(`Method ${methodName} not found in ${filename}`);
        }
        const signature = method.getSignature();

        const params = signature.getParameters().map((p) => serializeProperty(p).type);


        const returnType = signature.getReturnType();

        // Unwrap Promise from Promise<T>
        const actualReturnType = returnType.getTypeArguments()[0];

        return JSON.stringify({
            params,
            returnType: serializeType(actualReturnType)
        })
    },

    checkStarted: async () => '',
};


export const openrpcDocument = {
    openrpc: "1.0.0",
    info: {
        title: "node-json-rpc-server example",
        version: "1.0.0"
    },
    methods: [
        {
            name: "queryTypesOfMethod",
            params: [
                { name: "filename", schema: { type: "string" } },
                { name: "methodName", schema: { type: "string" } },
            ],
            result: {
                name: "j", schema: { type: "string" }
            }
        },
        {
            name: "checkStarted",
            params: [],
            result: {
                name: "j", schema: { type: "string" }
            }
        }
    ],
} as OpenrpcDocument;