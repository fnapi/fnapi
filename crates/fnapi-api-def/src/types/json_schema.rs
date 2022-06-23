use serde_json::{Map, Value};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{CallExpr, Expr, TsKeywordTypeKind};
use swc_ecma_utils::{member_expr, ExprFactory};

use super::{
    ArrayType, IntersectionType, KeywordType, ObjectType, Property, TupleType, Type, TypeElement,
    UnionType,
};

pub type JsonMap = Map<String, Value>;

/// Convert a type to ajv schema.
pub trait ToJsonSchema {
    fn to_json_schema(&self) -> JsonMap;

    fn to_js_expr(&self) -> Box<Expr> {
        let schema = Value::Object(self.to_json_schema());

        box Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: member_expr!(DUMMY_SP, JSON.parse).as_callee(),
            args: vec![schema.to_string().as_arg()],
            type_args: Default::default(),
        })
    }
}

impl ToJsonSchema for Type {
    fn to_json_schema(&self) -> JsonMap {
        match self {
            Type::Keyword(n) => n.to_json_schema(),
            Type::Array(n) => n.to_json_schema(),
            Type::Tuple(n) => n.to_json_schema(),
            Type::Object(n) => n.to_json_schema(),
            Type::Intersection(n) => n.to_json_schema(),
            Type::Union(n) => n.to_json_schema(),
        }
    }
}

impl ToJsonSchema for KeywordType {
    fn to_json_schema(&self) -> JsonMap {
        let s = match self.keyword {
            TsKeywordTypeKind::TsNumberKeyword => "number",
            TsKeywordTypeKind::TsBooleanKeyword => "boolean",
            TsKeywordTypeKind::TsStringKeyword => "string",
            TsKeywordTypeKind::TsNullKeyword | TsKeywordTypeKind::TsUndefinedKeyword => {
                panic!("null/undefined should be part of a union type")
            }
            _ => {
                unimplemented!("keyword type: {:?}", self.keyword)
            }
        };

        let mut map = Map::default();
        map.insert("type".into(), Value::String(s.into()));
        map
    }
}

impl ToJsonSchema for ArrayType {
    fn to_json_schema(&self) -> JsonMap {
        let mut map = Map::default();
        map.insert("type".into(), Value::String("array".into()));
        map.insert("items".into(), Value::Object(self.elem.to_json_schema()));
        map
    }
}

impl ToJsonSchema for TupleType {
    fn to_json_schema(&self) -> JsonMap {
        let mut map = Map::default();
        map.insert("type".into(), Value::String("array".into()));
        map.insert("items".into(), {
            let types = self
                .elems
                .iter()
                .map(|v| v.to_json_schema())
                .map(Value::Object)
                .collect();
            let mut map = Map::default();
            map.insert("oneOf".into(), Value::Array(types));
            Value::Object(map)
        });
        map
    }
}

impl ToJsonSchema for ObjectType {
    fn to_json_schema(&self) -> JsonMap {
        let mut map = Map::default();
        map.insert("type".into(), Value::String("object".into()));

        map.insert(
            "required".into(),
            Value::Array(
                self.members
                    .iter()
                    .filter_map(|m| match m {
                        TypeElement::Property(p) => {
                            if !p.optional && !is_optional(&p.ty) {
                                Some(p.name.clone())
                            } else {
                                None
                            }
                        }
                    })
                    .map(Value::String)
                    .collect(),
            ),
        );

        let mut properties = Map::default();
        for m in self.members.iter() {
            match m {
                TypeElement::Property(m) => {
                    properties.insert(m.name.clone(), Value::Object(m.to_json_schema()));
                }
            }
        }
        map.insert("properties".into(), Value::Object(properties));
        map
    }
}

impl ToJsonSchema for IntersectionType {
    fn to_json_schema(&self) -> JsonMap {
        let mut map = Map::default();
        map.insert(
            "allOf".into(),
            Value::Array(
                self.types
                    .iter()
                    .map(|v| v.to_json_schema())
                    .map(Value::Object)
                    .collect(),
            ),
        );
        map
    }
}

impl ToJsonSchema for UnionType {
    fn to_json_schema(&self) -> JsonMap {
        let mut map = Map::default();
        map.insert(
            "oneOf".into(),
            Value::Array(
                self.types
                    .iter()
                    .filter_map(|t|   match &**t {
                        Type::Keyword(KeywordType {
                            keyword: TsKeywordTypeKind::TsUndefinedKeyword | TsKeywordTypeKind::TsNullKeyword,
                            ..
                        }) => None,
                        _ => Some(t.to_json_schema()),
                    })
                    .map(Value::Object)
                    .collect(),
            ),
        );
        map
    }
}

impl ToJsonSchema for Property {
    fn to_json_schema(&self) -> JsonMap {
        self.ty.to_json_schema()
    }
}

fn is_optional(t: &Type) -> bool {
    match t {
        Type::Keyword(KeywordType {
            keyword: TsKeywordTypeKind::TsUndefinedKeyword | TsKeywordTypeKind::TsNullKeyword,
            ..
        }) => true,
        Type::Union(u) => u.types.iter().any(|t| is_optional(t)),
        _ => false,
    }
}
