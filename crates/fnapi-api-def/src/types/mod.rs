use serde::{Deserialize, Serialize};
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_utils::quote_ident;

pub mod json_schema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawTypeText(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Type {
    Keyword(KeywordType),
    Array(ArrayType),
    Tuple(TupleType),
    Object(ObjectType),
    Intersection(IntersectionType),
    Union(UnionType),
}

impl From<Type> for TsType {
    fn from(t: Type) -> Self {
        match t {
            Type::Keyword(t) => TsType::TsKeywordType(t.into()),
            Type::Array(t) => TsType::TsArrayType(t.into()),
            Type::Tuple(t) => TsType::TsTupleType(t.into()),
            Type::Object(t) => TsType::TsTypeLit(t.into()),
            Type::Intersection(t) => TsType::TsUnionOrIntersectionType(
                TsUnionOrIntersectionType::TsIntersectionType(t.into()),
            ),
            Type::Union(t) => {
                TsType::TsUnionOrIntersectionType(TsUnionOrIntersectionType::TsUnionType(t.into()))
            }
        }
    }
}

impl From<Type> for Box<TsType> {
    fn from(t: Type) -> Self {
        box t.into()
    }
}

impl From<Box<Type>> for Box<TsType> {
    fn from(t: Box<Type>) -> Self {
        (*t).into()
    }
}

impl From<Box<Type>> for TsTypeAnn {
    fn from(t: Box<Type>) -> Self {
        TsTypeAnn {
            span: DUMMY_SP,
            type_ann: t.into(),
        }
    }
}

impl From<Type> for TsTupleElement {
    fn from(t: Type) -> Self {
        TsTupleElement {
            span: DUMMY_SP,
            label: Default::default(),
            ty: t.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KeywordType {
    pub keyword: TsKeywordTypeKind,
}

impl From<KeywordType> for TsKeywordType {
    fn from(t: KeywordType) -> Self {
        Self {
            span: DUMMY_SP,
            kind: t.keyword,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArrayType {
    pub elem: Box<Type>,
}

impl From<ArrayType> for TsArrayType {
    fn from(t: ArrayType) -> Self {
        Self {
            span: DUMMY_SP,
            elem_type: t.elem.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TupleType {
    pub elems: Vec<Type>,
}

impl From<TupleType> for TsTupleType {
    fn from(t: TupleType) -> Self {
        Self {
            span: DUMMY_SP,
            elem_types: t.elems.into_iter().map(From::from).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectType {
    pub members: Vec<TypeElement>,
}

impl From<ObjectType> for TsTypeLit {
    fn from(t: ObjectType) -> Self {
        Self {
            span: DUMMY_SP,
            members: t.members.into_iter().map(From::from).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum TypeElement {
    Property(Property),
}

impl From<TypeElement> for TsTypeElement {
    fn from(t: TypeElement) -> Self {
        match t {
            TypeElement::Property(t) => TsTypeElement::TsPropertySignature(t.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Property {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Box<Type>,
    pub optional: bool,
}

impl From<Property> for TsPropertySignature {
    fn from(p: Property) -> Self {
        TsPropertySignature {
            span: DUMMY_SP,
            readonly: Default::default(),
            key: box Expr::Ident(quote_ident!(DUMMY_SP, p.name)),
            computed: false,
            optional: p.optional,
            init: Default::default(),
            params: Default::default(),
            type_ann: Some(p.ty.into()),
            type_params: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnionType {
    pub types: Vec<Box<Type>>,
}

impl From<UnionType> for TsUnionType {
    fn from(ty: UnionType) -> Self {
        Self {
            span: DUMMY_SP,
            types: ty.types.into_iter().map(From::from).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IntersectionType {
    pub types: Vec<Type>,
}

impl From<IntersectionType> for TsIntersectionType {
    fn from(ty: IntersectionType) -> Self {
        Self {
            span: DUMMY_SP,
            types: ty.types.into_iter().map(From::from).collect(),
        }
    }
}
