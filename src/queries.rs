use std::sync::OnceLock;

use tree_sitter::Query;

pub fn target_name() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(&crate::parser::language(), r"(target name: (identifier) @target_name)").unwrap()
    })
}

pub fn target_or_function_ref() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(
            &crate::parser::language(),
            r"(target_ref
                earthfile: (earthfile_ref)? @target_earthfile
                name: (identifier) @target_name) @ref
              (function_ref
                earthfile: (earthfile_ref)? @target_earthfile
                name: (identifier) @target_name) @ref
              ",
        )
        .unwrap()
    })
}
