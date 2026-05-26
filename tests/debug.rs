use java_lang::{ast::*, parse_str};

#[test]
fn debug_record() {
    let cu: CompilationUnit = parse_str("record Point(int x, int y) {}").unwrap();
    println!("{:?}", cu.type_decls);
}
