mod valid {
    use std::{path::Path, sync::Arc};

    use fnapi_api_def::types::{KeywordType, Type};
    use fnapi_testing::{run_async_test, swc_handler::HandlerOpts};
    use swc_common::errors::ColorConfig;

    use crate::{
        project::InputFiles,
        type_server::{MethodTypes, TypeServer},
    };

    /// Create a [TypeServer] for valid typescript types using typescript files
    /// in tests/type_server/valid
    async fn start() -> Arc<TypeServer> {
        TypeServer::start(
            Path::new("src/type_server/index.js"),
            &InputFiles::Files(vec!["tests/type_server/valid/simple.ts".into()]),
        )
        .await
        .unwrap()
    }

    fn fname(s: &str) -> String {
        format!("tests/type_server/valid/{}", s)
    }

    #[test]
    fn fetch_simple_return_type() {
        run_async_test(
            HandlerOpts {
                color: ColorConfig::Always,
            },
            |_cm| async move {
                let ts = start().await;

                let res = ts
                    .query_types_of_method(&fname("simple.ts"), "foo")
                    .await
                    .unwrap();
                dbg!(&res);

                assert_eq!(
                    res,
                    MethodTypes {
                        params: Default::default(),
                        return_type: Type::Keyword(KeywordType {
                            keyword: swc_ecmascript::ast::TsKeywordTypeKind::TsStringKeyword
                        })
                    }
                );

                Ok(())
            },
        )
        .unwrap();
    }
}
