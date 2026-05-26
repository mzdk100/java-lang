use java_lang::{ast::*, parse_str};

#[test]
fn test_empty_compilation_unit() {
    let cu: CompilationUnit = parse_str("").unwrap();
    assert!(cu.package.is_none());
    assert!(cu.imports.is_empty());
    assert!(cu.type_decls.is_empty());
}

#[test]
fn test_package_declaration() {
    let cu: CompilationUnit = parse_str("package com.example;").unwrap();
    assert!(cu.package.is_some());
    let pkg = cu.package.unwrap();
    assert_eq!(pkg.name.segments.len(), 2);
    assert_eq!(pkg.name.segments[0].ident.name, "com");
    assert_eq!(pkg.name.segments[1].ident.name, "example");
}

#[test]
fn test_annotated_package() {
    let cu: CompilationUnit = parse_str("@Deprecated package com.example;").unwrap();
    assert!(cu.package.is_some());
    let pkg = cu.package.unwrap();
    assert_eq!(pkg.annotations.len(), 1);
}

#[test]
fn test_single_import() {
    let cu: CompilationUnit = parse_str("import java.util.List;").unwrap();
    assert_eq!(cu.imports.len(), 1);
}

#[test]
fn test_static_import() {
    let cu: CompilationUnit = parse_str("import static java.util.Collections.emptyList;").unwrap();
    assert_eq!(cu.imports.len(), 1);
    match &cu.imports[0] {
        ImportDecl::SingleStatic { member, .. } => {
            assert_eq!(member.name, "emptyList");
        }
        _ => panic!("expected SingleStatic import"),
    }
}

#[test]
fn test_wildcard_import() {
    let cu: CompilationUnit = parse_str("import java.util.*;").unwrap();
    assert_eq!(cu.imports.len(), 1);
    match &cu.imports[0] {
        ImportDecl::TypeOnDemand { .. } => {}
        _ => panic!("expected TypeOnDemand import"),
    }
}

#[test]
fn test_simple_class() {
    let cu: CompilationUnit = parse_str("public class Hello {}").unwrap();
    assert_eq!(cu.type_decls.len(), 1);
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            assert_eq!(cls.name.name, "Hello");
            assert!(!cls.modifiers.is_empty());
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_class_with_extends() {
    let cu: CompilationUnit = parse_str("class Foo extends Bar {}").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            assert!(cls.extends_clause.is_some());
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_class_with_implements() {
    let cu: CompilationUnit = parse_str("class Foo implements Runnable, Serializable {}").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            assert!(cls.implements_clause.is_some());
            let impls = cls.implements_clause.as_ref().unwrap();
            assert_eq!(impls.supertypes.len(), 2);
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_interface() {
    let cu: CompilationUnit = parse_str("public interface Runnable { void run(); }").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Interface(iface) => {
            assert_eq!(iface.name.name, "Runnable");
        }
        _ => panic!("expected interface declaration"),
    }
}

#[test]
fn test_enum() {
    let cu: CompilationUnit = parse_str("enum Color { RED, GREEN, BLUE }").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Enum(enm) => {
            assert_eq!(enm.name.name, "Color");
            assert_eq!(enm.body.constants.len(), 3);
        }
        _ => panic!("expected enum declaration"),
    }
}

#[test]
fn test_record() {
    let cu: CompilationUnit = parse_str("record Point(int x, int y) {}").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Record(rec) => {
            assert_eq!(rec.name.name, "Point");
            assert_eq!(rec.components.components.len(), 2);
        }
        _ => panic!("expected record declaration"),
    }
}

#[test]
fn test_field_declaration() {
    let cu: CompilationUnit =
        parse_str("class Foo { private int x = 42; public static final String MSG = \"hello\"; }")
            .unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            assert_eq!(cls.body.declarations.len(), 2);
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_method_declaration() {
    let cu: CompilationUnit =
        parse_str("class Foo { public int add(int a, int b) { return a + b; } }").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            assert_eq!(cls.body.declarations.len(), 1);
            match &cls.body.declarations[0] {
                ClassBodyDecl::Method(m) => {
                    assert_eq!(m.name.name, "add");
                    assert_eq!(m.params.len(), 2);
                }
                _ => panic!("expected method"),
            }
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_abstract_method() {
    let cu: CompilationUnit =
        parse_str("abstract class Foo { abstract void doSomething(); }").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => match &cls.body.declarations[0] {
            ClassBodyDecl::Method(m) => {
                assert_eq!(m.name.name, "doSomething");
                assert!(m.body.is_none());
            }
            _ => panic!("expected method"),
        },
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_generic_class() {
    let cu: CompilationUnit = parse_str("class Box<T> { T value; }").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            assert!(cls.type_params.is_some());
            let tp = cls.type_params.as_ref().unwrap();
            assert_eq!(tp.params.len(), 1);
            assert_eq!(tp.params[0].name.name, "T");
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_bounded_type_param() {
    let cu: CompilationUnit =
        parse_str("class Foo<T extends Comparable<T> & Serializable> {}").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            let tp = cls.type_params.as_ref().unwrap();
            assert!(tp.params[0].bound.is_some());
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_annotation() {
    let cu: CompilationUnit =
        parse_str("@Override public class Foo { @Deprecated void old() {} }").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            // Check class annotations
            assert!(cls.modifiers.iter().any(|m| m.is_annotation()));
            // Check method annotations
            match &cls.body.declarations[0] {
                ClassBodyDecl::Method(m) => {
                    assert!(m.modifiers.iter().any(|m| m.is_annotation()));
                }
                _ => panic!("expected method"),
            }
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_annotation_with_value() {
    let cu: CompilationUnit = parse_str("@SuppressWarnings(\"unchecked\") class Foo {}").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            let ann = cls.modifiers.iter().find(|m| m.is_annotation()).unwrap();
            match ann {
                Modifier::Annotation(a) => match a {
                    Annotation::SingleElement { value, .. } => match value.as_ref() {
                        ElementValue::Expr(e) => match e {
                            Expr::Literal(Lit::Str(s)) => {
                                assert_eq!(s.value, "unchecked");
                            }
                            _ => panic!("expected string literal"),
                        },
                        _ => panic!("expected expr element value"),
                    },
                    _ => panic!("expected single element annotation"),
                },
                _ => panic!("expected annotation modifier"),
            }
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_local_variable() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { int x = 10; String s = \"hello\"; } }").unwrap();
}

#[test]
fn test_var_type() {
    let _cu: CompilationUnit = parse_str("class Foo { void m() { var x = 10; } }").unwrap();
}

#[test]
fn test_if_statement() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { if (x > 0) { } else { } } }").unwrap();
}

#[test]
fn test_for_loop() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { for (int i = 0; i < 10; i++) { } } }").unwrap();
}

#[test]
fn test_enhanced_for() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { for (String s : list) { } } }").unwrap();
}

#[test]
fn test_while_loop() {
    let _cu: CompilationUnit = parse_str("class Foo { void m() { while (true) { } } }").unwrap();
}

#[test]
fn test_try_catch() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { try { } catch (Exception e) { } finally { } } }")
            .unwrap();
}

#[test]
fn test_try_with_resources() {
    let _cu: CompilationUnit = parse_str(
        "class Foo { void m() { try (var r = new FileReader(\"f\")) { } catch (Exception e) { } } }"
    ).unwrap();
}

#[test]
fn test_lambda() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { Runnable r = () -> { }; } }").unwrap();
}

#[test]
fn test_method_reference() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { var r = String::length; } }").unwrap();
}

#[test]
fn test_switch_expression() {
    let _cu: CompilationUnit = parse_str(
        "class Foo { int m(int x) { return switch (x) { case 1 -> 1; case 2 -> 2; default -> 0; }; } }"
    ).unwrap();
}

#[test]
fn test_text_block() {
    let _cu: CompilationUnit = parse_str(r#"class Foo { String s = """hello"""; }"#).unwrap();
}

#[test]
fn test_sealed_class() {
    let cu: CompilationUnit = parse_str("sealed class Shape permits Circle, Square {}").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            assert!(cls.permits_clause.is_some());
            let permits = cls.permits_clause.as_ref().unwrap();
            assert_eq!(permits.types.len(), 2);
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_module_declaration() {
    let cu: CompilationUnit =
        parse_str("module com.example { requires java.base; exports com.example.api; }").unwrap();
    assert!(cu.module.is_some());
    let mod_decl = cu.module.unwrap();
    assert_eq!(mod_decl.directives.len(), 2);
}

#[test]
fn test_inner_class() {
    let _cu: CompilationUnit = parse_str("class Outer { class Inner { } }").unwrap();
}

#[test]
fn test_anonymous_class() {
    let _cu: CompilationUnit = parse_str(
        "class Foo { void m() { Runnable r = new Runnable() { public void run() { } }; } }",
    )
    .unwrap();
}

#[test]
fn test_array_creation() {
    let _cu: CompilationUnit = parse_str(
        "class Foo { void m() { int[] arr = new int[10]; int[][] arr2 = new int[3][4]; } }",
    )
    .unwrap();
}

#[test]
fn test_array_initializer() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { int[] arr = {1, 2, 3}; } }").unwrap();
}

#[test]
fn test_instanceof_pattern() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m(Object o) { if (o instanceof String s) { } } }").unwrap();
}

#[test]
fn test_record_pattern() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m(Object o) { if (o instanceof Point(int x, int y)) { } } }")
            .unwrap();
}

#[test]
fn test_constructor() {
    let _cu: CompilationUnit =
        parse_str("class Foo { private int x; public Foo(int x) { this.x = x; } }").unwrap();
}

#[test]
fn test_static_initializer() {
    let _cu: CompilationUnit =
        parse_str("class Foo { static { System.out.println(\"init\"); } }").unwrap();
}

#[test]
fn test_instance_initializer() {
    let _cu: CompilationUnit = parse_str("class Foo { { System.out.println(\"init\"); } }").unwrap();
}

#[test]
fn test_multiple_annotations() {
    let cu: CompilationUnit =
        parse_str("@Deprecated @SuppressWarnings(\"all\") public class Foo {}").unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Class(cls) => {
            let ann_count = cls.modifiers.iter().filter(|m| m.is_annotation()).count();
            assert_eq!(ann_count, 2);
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_varargs() {
    let _cu: CompilationUnit = parse_str("class Foo { void m(String... args) { } }").unwrap();
}

#[test]
fn test_throws_clause() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() throws IOException, SQLException { } }").unwrap();
}

#[test]
fn test_generic_method() {
    let _cu: CompilationUnit = parse_str("class Foo { <T> T identity(T t) { return t; } }").unwrap();
}

#[test]
fn test_wildcard_type() {
    let _cu: CompilationUnit =
        parse_str("class Foo { java.util.List<?> list; java.util.List<? extends Number> nums; }")
            .unwrap();
}

#[test]
fn test_ternary_operator() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { int x = (a > b) ? a : b; } }").unwrap();
}

#[test]
fn test_pre_post_increment() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { int i = 0; i++; ++i; i--; --i; } }").unwrap();
}

#[test]
fn test_synchronized() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { synchronized (lock) { } } }").unwrap();
}

#[test]
fn test_assert() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { assert x > 0; assert x > 0 : \"error\"; } }").unwrap();
}

#[test]
fn test_do_while() {
    let _cu: CompilationUnit = parse_str("class Foo { void m() { do { } while (true); } }").unwrap();
}

#[test]
fn test_break_continue() {
    let _cu: CompilationUnit = parse_str(
        "class Foo { void m() { outer: for (int i = 0; i < 10; i++) { if (i == 5) break outer; continue; } } }"
    ).unwrap();
}

#[test]
fn test_yield_statement() {
    let _cu: CompilationUnit = parse_str(
        "class Foo { int m(int x) { return switch (x) { case 1 -> { yield 1; } default -> 0; }; } }"
    ).unwrap();
}

#[test]
fn test_compact_constructor() {
    let _cu: CompilationUnit =
        parse_str("record Point(int x, int y) { public Point { /* validation */ } }").unwrap();
}

#[test]
fn test_enum_with_body() {
    let cu: CompilationUnit = parse_str(
        "enum Color { RED(255,0,0), GREEN(0,255,0), BLUE(0,0,255); private int r,g,b; Color(int r,int g,int b){ this.r=r; this.g=g; this.b=b; } }"
    ).unwrap();
    match &cu.type_decls[0] {
        TypeDecl::Enum(enm) => {
            assert_eq!(enm.body.constants.len(), 3);
            assert!(!enm.body.members.is_empty());
        }
        _ => panic!("expected enum"),
    }
}

#[test]
fn test_interface_with_default_method() {
    let _cu: CompilationUnit =
        parse_str("interface Foo { default void hello() { System.out.println(\"hello\"); } }")
            .unwrap();
}

#[test]
fn test_deeply_nested_generics() {
    let _cu: CompilationUnit =
        parse_str("class Foo { java.util.Map<String, java.util.List<Integer>> map; }").unwrap();
}

#[test]
fn test_primitive_types() {
    let input =
        "class Foo { byte b; short s; int i; long l; float f; double d; char c; boolean z; }";
    let cu: CompilationUnit = parse_str(input).unwrap();
    assert!(!cu.type_decls.is_empty());
}

#[test]
fn test_string_concatenation() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { String s = \"hello\" + \" \" + \"world\"; } }").unwrap();
}

#[test]
fn test_binary_operators() {
    let _cu: CompilationUnit = parse_str(
        "class Foo { void m() { boolean a = true && false || true; int b = 1 + 2 - 3 * 4 / 5 % 6; int c = 1 << 2 >> 3 >>> 4; int d = 1 & 2 | 3 ^ 4; } }"
    ).unwrap();
}

#[test]
fn test_comparison_operators() {
    let _cu: CompilationUnit = parse_str(
        "class Foo { void m() { boolean a = 1 == 2; boolean b = 1 != 2; boolean c = 1 < 2; boolean d = 1 > 2; boolean e = 1 <= 2; boolean f = 1 >= 2; } }"
    ).unwrap();
}

#[test]
fn test_cast_expression() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { String s = (String) obj; int i = (int) d; } }").unwrap();
}

#[test]
fn test_array_access() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { int x = arr[0]; int y = arr[i + 1]; } }").unwrap();
}

#[test]
fn test_method_call_chaining() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { int len = str.trim().toUpperCase().length(); } }")
            .unwrap();
}

#[test]
fn test_field_access() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { int x = this.value; String s = obj.name; } }").unwrap();
}

#[test]
fn test_class_literal() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { Class<?> c = String.class; } }").unwrap();
}

#[test]
fn test_super_keyword() {
    let _cu: CompilationUnit =
        parse_str("class Foo extends Bar { void m() { super.m(); } }").unwrap();
}
