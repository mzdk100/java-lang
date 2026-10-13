use java_lang::{CommentKind, ast::*, parse_str};

fn type_decls(cu: &CompilationUnit) -> &[TypeDecl] {
    assert!(
        matches!(cu.kind, CompilationUnitKind::Ordinary { .. }),
        "expected an ordinary compilation unit"
    );
    cu.type_decls()
}

fn package(cu: &CompilationUnit) -> Option<&PackageDecl> {
    assert!(
        matches!(cu.kind, CompilationUnitKind::Ordinary { .. }),
        "expected an ordinary compilation unit"
    );
    cu.package()
}

fn module(cu: &CompilationUnit) -> Option<&ModuleDecl> {
    match &cu.kind {
        CompilationUnitKind::Modular(m) => Some(m),
        _ => None,
    }
}

fn implicit_class(cu: &CompilationUnit) -> Option<&ImplicitClassDecl> {
    match &cu.kind {
        CompilationUnitKind::Compact(c) => Some(c),
        _ => None,
    }
}

#[test]
fn test_empty_compilation_unit() {
    let cu: CompilationUnit = parse_str("").unwrap();
    assert!(package(&cu).is_none());
    assert!(cu.imports.is_empty());
    assert!(type_decls(&cu).is_empty());
    assert!(implicit_class(&cu).is_none());
}

#[test]
fn test_implicit_class_compact_compilation_unit() {
    let cu: CompilationUnit = parse_str("void main() { System.out.println(\"hi\"); }").unwrap();
    let members = implicit_class(&cu).unwrap().members.clone();
    assert_eq!(members.len(), 1);
    assert!(matches!(members[0], ClassBodyDecl::Method(_)));
}

#[test]
fn test_implicit_class_compilation_unit_span() {
    let src = "void main() {} ;";
    let cu: CompilationUnit = parse_str(src).unwrap();
    assert_eq!(cu.span().source_text(src), src);
}

#[test]
fn test_implicit_class_span_includes_leading_field() {
    let src = "int x; void main() {}";
    let cu: CompilationUnit = parse_str(src).unwrap();
    assert_eq!(cu.span().source_text(src), src);
}

#[test]
fn test_implicit_class_mixed_members() {
    let cu: CompilationUnit = parse_str("int x; void main() {} class Nested {}").unwrap();
    let members = implicit_class(&cu).unwrap().members.clone();
    assert!(matches!(members[0], ClassBodyDecl::Field(_)));
    assert!(matches!(members[1], ClassBodyDecl::Method(_)));
    assert!(matches!(members[2], ClassBodyDecl::Class(_)));
}

#[test]
fn test_implicit_class_reclassifies_leading_type_decl() {
    // A class seen before the member that reveals this is a compact unit must
    // become an implicit-class member too, not stay in type_decls.
    let cu: CompilationUnit = parse_str("class Helper {} void main() {}").unwrap();
    let members = implicit_class(&cu).unwrap().members.clone();
    assert!(matches!(members[0], ClassBodyDecl::Class(_)));
    assert!(matches!(members[1], ClassBodyDecl::Method(_)));
}

#[test]
fn test_ordinary_compilation_unit_has_no_implicit_class() {
    let cu: CompilationUnit = parse_str("class Foo {}").unwrap();
    assert!(implicit_class(&cu).is_none());
    assert_eq!(type_decls(&cu).len(), 1);
}

#[test]
fn test_implicit_class_requires_a_method() {
    parse_str::<CompilationUnit>("int x;").unwrap_err();
}

#[test]
fn test_implicit_class_rejects_package_declaration() {
    parse_str::<CompilationUnit>("package p; void main() {}").unwrap_err();
}

#[test]
fn test_implicit_class_rejects_constructor() {
    parse_str::<CompilationUnit>("Foo() {} void main() {}").unwrap_err();
}

#[test]
fn test_implicit_class_rejects_module_declaration_before() {
    parse_str::<CompilationUnit>("module demo {} void main() {}").unwrap_err();
}

#[test]
fn test_implicit_class_rejects_module_declaration_after() {
    parse_str::<CompilationUnit>("void main() {} module demo {}").unwrap_err();
}

#[test]
fn test_import_after_compact_unit_member_rejected() {
    parse_str::<CompilationUnit>("void main() {} import java.util.List;").unwrap_err();
}

#[test]
fn test_import_after_leading_compact_unit_member_rejected() {
    parse_str::<CompilationUnit>("int x; import java.util.List; void main() {}").unwrap_err();
}

#[test]
fn test_import_after_type_decl_rejected() {
    parse_str::<CompilationUnit>("class Foo {} import java.util.List;").unwrap_err();
}

#[test]
fn test_import_after_module_declaration_rejected() {
    parse_str::<CompilationUnit>("module demo {} import java.util.List;").unwrap_err();
}

#[test]
fn test_import_after_leading_semicolon_rejected() {
    parse_str::<CompilationUnit>("; import java.util.List; void main() {}").unwrap_err();
}

#[test]
fn test_package_declaration() {
    let cu: CompilationUnit = parse_str("package com.example;").unwrap();
    assert!(package(&cu).is_some());
    let pkg = package(&cu).unwrap().clone();
    assert_eq!(pkg.name.segments.len(), 2);
    assert_eq!(pkg.name.segments[0].ident.name, "com");
    assert_eq!(pkg.name.segments[1].ident.name, "example");
}

#[test]
fn test_compilation_unit_span_starts_at_package() {
    let src = "package p; import java.util.List; class Foo {}";
    let cu: CompilationUnit = parse_str(src).unwrap();
    assert_eq!(cu.span().source_text(src), src);
}

#[test]
fn test_annotated_package() {
    let cu: CompilationUnit = parse_str("@Deprecated package com.example;").unwrap();
    assert!(package(&cu).is_some());
    let pkg = package(&cu).unwrap().clone();
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
    assert_eq!(type_decls(&cu).len(), 1);
    match &type_decls(&cu)[0] {
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
    match &type_decls(&cu)[0] {
        TypeDecl::Class(cls) => {
            assert!(cls.extends_clause.is_some());
        }
        _ => panic!("expected class declaration"),
    }
}

#[test]
fn test_class_with_implements() {
    let cu: CompilationUnit = parse_str("class Foo implements Runnable, Serializable {}").unwrap();
    match &type_decls(&cu)[0] {
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
    match &type_decls(&cu)[0] {
        TypeDecl::Interface(iface) => {
            assert_eq!(iface.name.name, "Runnable");
        }
        _ => panic!("expected interface declaration"),
    }
}

#[test]
fn test_enum() {
    let cu: CompilationUnit = parse_str("enum Color { RED, GREEN, BLUE }").unwrap();
    match &type_decls(&cu)[0] {
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
    match &type_decls(&cu)[0] {
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
    match &type_decls(&cu)[0] {
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
    match &type_decls(&cu)[0] {
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
fn test_method_span_includes_return_type() {
    let src = "void main() {}";
    let cu: CompilationUnit = parse_str(src).unwrap();
    let ClassBodyDecl::Method(m) = &implicit_class(&cu).unwrap().members[0] else {
        panic!("expected method");
    };
    assert_eq!(m.span().source_text(src), "void main() {}");
}

#[test]
fn test_primitive_return_type_span_excludes_enclosing_class() {
    let src = "class Foo { int value() {} }";
    let cu: CompilationUnit = parse_str(src).unwrap();
    let TypeDecl::Class(cls) = &type_decls(&cu)[0] else {
        panic!("expected class");
    };
    let ClassBodyDecl::Method(m) = &cls.body.declarations[0] else {
        panic!("expected method");
    };
    assert_eq!(m.span().source_text(src), "int value() {}");
}

#[test]
fn test_abstract_method() {
    let cu: CompilationUnit =
        parse_str("abstract class Foo { abstract void doSomething(); }").unwrap();
    match &type_decls(&cu)[0] {
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
    match &type_decls(&cu)[0] {
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
    match &type_decls(&cu)[0] {
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
    match &type_decls(&cu)[0] {
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
    match &type_decls(&cu)[0] {
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
fn test_lambda_parameter_names() {
    for expression in [
        "x -> x",
        "_ -> 0",
        "() -> {}",
        "(_) -> {}",
        "(_, _) -> {}",
        "(x, _) -> x",
        "(String _) -> 0",
        "(String x, String _) -> x",
        "(String _, String _) -> {}",
    ] {
        let source = format!("class Foo {{ Object f = {expression}; }}");
        assert!(
            parse_str::<CompilationUnit>(&source).is_ok(),
            "{expression}"
        );
    }
}

#[test]
fn test_invalid_lambda_parameter_names() {
    for expression in [
        "(123) -> 0",
        "(_, 123) -> 0",
        "(String 123) -> 0",
        "_",
        "(String x, y) -> 0",
        "(x, String y) -> 0",
        "(String _, _) -> 0",
        "(_, String _) -> 0",
    ] {
        let source = format!("class Foo {{ Object f = {expression}; }}");
        assert!(
            parse_str::<CompilationUnit>(&source).is_err(),
            "{expression}"
        );
    }
}

#[test]
fn test_cast_unnamed_lambda() {
    parse_str::<CompilationUnit>(
        "class Foo {
            Object f = (java.util.function.Function<String, Integer>) _ -> 0;
        }",
    )
    .unwrap();
}

#[test]
fn test_method_reference() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { var r = String::length; } }").unwrap();
}

#[test]
fn test_method_reference_qualified_generic_target() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { var r = Outer<String>.Inner<Integer>::get; } }").unwrap();
}

#[test]
fn test_qualified_constructor_reference_multi_segment_path() {
    let _cu: CompilationUnit = parse_str(
        "class Foo { void bar() {
            x = Builder<K, V>::new;             // single-segment path
            y = ImmutableMap.Builder<K, V>::new; // multi-segment path
        } }",
    )
    .unwrap();
}

#[test]
fn test_qualified_method_reference_multi_segment_path() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void bar() { z = Map.Entry<String, Long>::getValue; } }").unwrap();
}

#[test]
fn test_method_reference_array_type_target() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { var r = java.util.List<?>[]::new; } }").unwrap();
}

#[test]
fn test_method_reference_array_type_non_constructor() {
    for src in [
        "class Foo { void bar() { x = String[]::clone; } }",
        "class Foo { void bar() { x = int[]::clone; } }",
        "class Foo { void bar() { x = java.util.List<?>[]::clone; } }",
    ] {
        parse_str::<CompilationUnit>(src).unwrap();
    }
}

#[test]
fn test_lt_and_method_ref_disambiguation() {
    for src in [
        "class Foo { void bar() { boolean b = a < c; } }",
        "class Foo { void bar() { boolean b = a.b < c; } }",
        "class Foo { void bar() { List<String> l = new ArrayList<>(); } }",
        "class Foo { void bar() { int x = Collections.<String>emptyList().size(); } }",
        "class Foo { void bar() { Runnable r = String::new; } }",
        "class Foo { void bar() { Runnable r = int[]::new; } }",
        "class Foo { void bar() { Runnable r = ArrayList<String>::new; } }",
    ] {
        parse_str::<CompilationUnit>(src).unwrap();
    }
}

#[test]
fn test_explicit_type_witness_with_nested_generics() {
    for src in [
        "class Foo { void bar() { a.<B<C>>m(); } }",
        "class Foo { void bar() { a.<B<C<D>>>m(); } }",
    ] {
        parse_str::<CompilationUnit>(src).unwrap();
    }
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
    match &type_decls(&cu)[0] {
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
    assert!(module(&cu).is_some());
    let mod_decl = module(&cu).unwrap().clone();
    assert_eq!(mod_decl.directives.len(), 2);
}

#[test]
fn test_module_requires_static_transitive_order() {
    let _cu: CompilationUnit =
        parse_str("module org.example { requires static transitive org.apiguardian.api; }")
            .unwrap();
}

#[test]
fn test_annotated_module_declaration() {
    let _cu: CompilationUnit =
        parse_str("@SuppressWarnings(\"deprecation\") module org.example { }").unwrap();
}

#[test]
fn test_module_import_declaration() {
    let _cu: CompilationUnit =
        parse_str("import module org.junit.jupiter.api;\n\nclass Foo {\n}\n").unwrap();
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
    let _cu: CompilationUnit =
        parse_str("class Foo { { System.out.println(\"init\"); } }").unwrap();
}

#[test]
fn test_multiple_annotations() {
    let cu: CompilationUnit =
        parse_str("@Deprecated @SuppressWarnings(\"all\") public class Foo {}").unwrap();
    match &type_decls(&cu)[0] {
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
fn test_varargs_with_type_use_annotation() {
    let _cu: CompilationUnit =
        parse_str("class Foo { static String bar(Object @Nullable ... args) { return null; } }")
            .unwrap();
}

#[test]
fn test_record_varargs_component() {
    let _cu: CompilationUnit = parse_str("record Foo(Object... elements) {}").unwrap();
}

#[test]
fn test_throws_clause() {
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() throws IOException, SQLException { } }").unwrap();
}

#[test]
fn test_generic_method() {
    let _cu: CompilationUnit =
        parse_str("class Foo { <T> T identity(T t) { return t; } }").unwrap();
}

#[test]
fn test_wildcard_type() {
    let _cu: CompilationUnit =
        parse_str("class Foo { java.util.List<?> list; java.util.List<? extends Number> nums; }")
            .unwrap();
}

#[test]
fn test_wildcard_type_with_type_use_annotation() {
    let _cu: CompilationUnit =
        parse_str("class Foo { ImmutableList<@UnknownKeyFor ? extends Bar> x; }").unwrap();
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
    let _cu: CompilationUnit =
        parse_str("class Foo { void m() { do { } while (true); } }").unwrap();
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
    match &type_decls(&cu)[0] {
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
    assert!(!type_decls(&cu).is_empty());
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
fn test_cast_of_contextual_keyword_ident() {
    for word in [
        "record",
        "sealed",
        "var",
        "yield",
        "open",
        "provides",
        "requires",
        "uses",
        "with",
        "when",
        "to",
        "exports",
        "opens",
        "transitive",
        "permits",
        "module",
    ] {
        let src = format!("class Foo {{ void m() {{ String s = (String) {word}; }} }}");
        parse_str::<CompilationUnit>(&src).unwrap();
    }
}

#[test]
fn test_qualified_super_with_compound_target() {
    for target in ["holder.outer", "getOuter()", "outers[0]"] {
        let src = format!("class Sub {{ Sub() {{ {target}.super(); }} }}");
        parse_str::<CompilationUnit>(&src).unwrap();
    }
}

#[test]
fn test_qualified_super_with_explicit_type_args() {
    parse_str::<CompilationUnit>("class Sub { Sub() { outer.<String>super(\"x\"); } }").unwrap();
}

#[test]
fn test_qualified_super_call_spans() {
    let src = "class Sub { Sub() { outer.super(42); } }";
    let cu: CompilationUnit = parse_str(src).unwrap();
    let ClassBodyDecl::Constructor(ctor) = &as_class(&cu).body.declarations[0] else {
        panic!("expected constructor declaration");
    };
    let ExplicitConstructorCall::SuperFromExpr {
        paren_span,
        semi_span,
        ..
    } = ctor.body.explicit_constructor_call.as_ref().unwrap()
    else {
        panic!("expected qualified super call");
    };
    assert_eq!(paren_span.0.source_text(src), "(");
    assert_eq!(paren_span.1.source_text(src), ")");
    assert_eq!(semi_span.source_text(src), ";");
}

fn as_class(cu: &CompilationUnit) -> &ClassDecl {
    match &type_decls(cu)[0] {
        TypeDecl::Class(cls) => cls,
        _ => panic!("expected class declaration"),
    }
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

// ============================================================================
// Comment parsing tests
// ============================================================================

#[test]
fn test_doc_comment_on_class() {
    let cu: CompilationUnit = parse_str("/** A class. */\nclass Foo {}").unwrap();
    let decl = match &type_decls(&cu)[0] {
        TypeDecl::Class(c) => c,
        _ => panic!("expected class"),
    };
    assert_eq!(decl.doc_comment.len(), 1);
    assert_eq!(decl.doc_comment[0].kind, CommentKind::DocBlock);
}

#[test]
fn test_doc_comment_on_method() {
    let cu: CompilationUnit = parse_str("class Foo {\n/** A method. */\nvoid m() {}\n}").unwrap();
    let class = match &type_decls(&cu)[0] {
        TypeDecl::Class(c) => c,
        _ => panic!("expected class"),
    };
    let method = match &class.body.declarations[0] {
        ClassBodyDecl::Method(m) => m,
        _ => panic!("expected method"),
    };
    assert_eq!(method.doc_comment.len(), 1);
    assert_eq!(method.doc_comment[0].kind, CommentKind::DocBlock);
}

#[test]
fn test_doc_comment_on_field() {
    let cu: CompilationUnit = parse_str("class Foo {\n/** A field. */\nint x;\n}").unwrap();
    let class = match &type_decls(&cu)[0] {
        TypeDecl::Class(c) => c,
        _ => panic!("expected class"),
    };
    let field = match &class.body.declarations[0] {
        ClassBodyDecl::Field(f) => f,
        _ => panic!("expected field"),
    };
    assert_eq!(field.doc_comment.len(), 1);
    assert_eq!(field.doc_comment[0].kind, CommentKind::DocBlock);
}

#[test]
fn test_doc_line_comment_on_class() {
    let cu: CompilationUnit = parse_str("/// A class.\nclass Foo {}").unwrap();
    let decl = match &type_decls(&cu)[0] {
        TypeDecl::Class(c) => c,
        _ => panic!("expected class"),
    };
    assert_eq!(decl.doc_comment.len(), 1);
    assert_eq!(decl.doc_comment[0].kind, CommentKind::DocLine);
}

#[test]
fn test_regular_comment_on_statement() {
    let cu: CompilationUnit =
        parse_str("class Foo {\nvoid m() {\n// a comment\nint x = 1;\n}\n}").unwrap();
    let class = match &type_decls(&cu)[0] {
        TypeDecl::Class(c) => c,
        _ => panic!("expected class"),
    };
    let method = match &class.body.declarations[0] {
        ClassBodyDecl::Method(m) => m,
        _ => panic!("expected method"),
    };
    let block = method.body.as_ref().unwrap();
    let local = match &block.stmts[0] {
        Stmt::LocalVarDecl(d) => d,
        _ => panic!("expected local var decl"),
    };
    assert_eq!(local.leading_comments.len(), 1);
    assert_eq!(local.leading_comments[0].kind, CommentKind::Line);
}

#[test]
fn test_block_comment_on_statement() {
    let cu: CompilationUnit =
        parse_str("class Foo {\nvoid m() {\n/* block */\nint x = 1;\n}\n}").unwrap();
    let class = match &type_decls(&cu)[0] {
        TypeDecl::Class(c) => c,
        _ => panic!("expected class"),
    };
    let method = match &class.body.declarations[0] {
        ClassBodyDecl::Method(m) => m,
        _ => panic!("expected method"),
    };
    let block = method.body.as_ref().unwrap();
    let local = match &block.stmts[0] {
        Stmt::LocalVarDecl(d) => d,
        _ => panic!("expected local var decl"),
    };
    assert_eq!(local.leading_comments.len(), 1);
    assert_eq!(local.leading_comments[0].kind, CommentKind::Block);
}

#[test]
fn test_multiple_doc_comments() {
    let cu: CompilationUnit =
        parse_str("/// Line 1\n/// Line 2\n/** Block doc. */\nclass Foo {}").unwrap();
    let decl = match &type_decls(&cu)[0] {
        TypeDecl::Class(c) => c,
        _ => panic!("expected class"),
    };
    assert_eq!(decl.doc_comment.len(), 3);
    assert_eq!(decl.doc_comment[0].kind, CommentKind::DocLine);
    assert_eq!(decl.doc_comment[1].kind, CommentKind::DocLine);
    assert_eq!(decl.doc_comment[2].kind, CommentKind::DocBlock);
}

#[test]
fn test_regular_comment_skipped_for_items() {
    // Regular comments before items should NOT attach as doc comments
    let cu: CompilationUnit = parse_str("// regular\nclass Foo {}").unwrap();
    let decl = match &type_decls(&cu)[0] {
        TypeDecl::Class(c) => c,
        _ => panic!("expected class"),
    };
    assert!(decl.doc_comment.is_empty());
}

#[test]
fn test_comments_collected_in_compilation_unit() {
    let cu: CompilationUnit = parse_str("class Foo {}\n// trailing").unwrap();
    assert_eq!(cu.comments.len(), 1);
    assert_eq!(cu.comments[0].kind, CommentKind::Line);
}

#[test]
fn test_doc_comment_on_interface() {
    let cu: CompilationUnit = parse_str("/** An interface. */\ninterface Foo {}").unwrap();
    let decl = match &type_decls(&cu)[0] {
        TypeDecl::Interface(i) => i,
        _ => panic!("expected interface"),
    };
    assert_eq!(decl.doc_comment.len(), 1);
    assert_eq!(decl.doc_comment[0].kind, CommentKind::DocBlock);
}

#[test]
fn test_doc_comment_on_enum() {
    let cu: CompilationUnit = parse_str("/** An enum. */\nenum Foo { A, B }").unwrap();
    let decl = match &type_decls(&cu)[0] {
        TypeDecl::Enum(e) => e,
        _ => panic!("expected enum"),
    };
    assert_eq!(decl.doc_comment.len(), 1);
    assert_eq!(decl.doc_comment[0].kind, CommentKind::DocBlock);
}

#[test]
fn test_doc_comment_on_record() {
    let cu: CompilationUnit = parse_str("/** A record. */\nrecord Foo(int x) {}").unwrap();
    let decl = match &type_decls(&cu)[0] {
        TypeDecl::Record(r) => r,
        _ => panic!("expected record"),
    };
    assert_eq!(decl.doc_comment.len(), 1);
    assert_eq!(decl.doc_comment[0].kind, CommentKind::DocBlock);
}

#[test]
fn test_comment_between_items_not_attached() {
    let cu: CompilationUnit =
        parse_str("/** Doc A. */\nclass A {}\n// between\n/** Doc B. */\nclass B {}").unwrap();
    let a = match &type_decls(&cu)[0] {
        TypeDecl::Class(c) => c,
        _ => panic!("expected class A"),
    };
    assert_eq!(a.doc_comment.len(), 1);
    let b = match &type_decls(&cu)[1] {
        TypeDecl::Class(c) => c,
        _ => panic!("expected class B"),
    };
    assert_eq!(b.doc_comment.len(), 1);
}

#[test]
fn test_declaration_spans_point_at_their_own_tokens() {
    let src = "package com.example;\n\n\
import java.util.List;\n\
import java.util.*;\n\
import static java.util.Collections.emptyList;\n\
import static java.util.Arrays.*;\n\n\
public class Outer implements Runnable {\n\
    private int field = 1;\n\n\
    Outer(int count, String... names) {\n\
        super();\n\
        this.field = count;\n\
    }\n\n\
    public void run() {\n\
        int[] xs = new int[3];\n\
        xs[0] = 1;\n\
    }\n\
}\n";
    let cu: CompilationUnit = parse_str(src).unwrap();

    let pkg = package(&cu).unwrap();
    assert_eq!(pkg.package_span.source_text(src), "package");
    assert_eq!(pkg.name.span.source_text(src), "com.example");
    assert_eq!(pkg.semi_span.source_text(src), ";");
    assert_eq!(pkg.span.source_text(src), "package com.example;");

    let ImportDecl::SingleType {
        import_span,
        path,
        semi_span,
    } = &cu.imports[0]
    else {
        panic!("expected a single type import");
    };
    assert_eq!(import_span.source_text(src), "import");
    assert_eq!(path.span.source_text(src), "java.util.List");
    assert_eq!(semi_span.source_text(src), ";");

    let ImportDecl::TypeOnDemand {
        path,
        star_span,
        semi_span,
        ..
    } = &cu.imports[1]
    else {
        panic!("expected a type import on demand");
    };
    assert_eq!(path.span.source_text(src), "java.util");
    assert_eq!(star_span.source_text(src), "*");
    assert_eq!(semi_span.source_text(src), ";");

    let ImportDecl::SingleStatic {
        static_span,
        path,
        member,
        semi_span,
        ..
    } = &cu.imports[2]
    else {
        panic!("expected a single static import");
    };
    assert_eq!(static_span.source_text(src), "static");
    assert_eq!(
        path.span.source_text(src),
        "java.util.Collections.emptyList"
    );
    assert_eq!(member.span.source_text(src), "emptyList");
    assert_eq!(semi_span.source_text(src), ";");

    let ImportDecl::StaticOnDemand {
        static_span,
        path,
        star_span,
        semi_span,
        ..
    } = &cu.imports[3]
    else {
        panic!("expected a static import on demand");
    };
    assert_eq!(static_span.source_text(src), "static");
    assert_eq!(path.span.source_text(src), "java.util.Arrays");
    assert_eq!(star_span.source_text(src), "*");
    assert_eq!(semi_span.source_text(src), ";");

    let TypeDecl::Class(cls) = &type_decls(&cu)[0] else {
        panic!("expected a class");
    };
    assert_eq!(cls.class_span.source_text(src), "class");
    assert_eq!(cls.name.span.source_text(src), "Outer");
    assert_eq!(
        cls.implements_clause
            .as_ref()
            .unwrap()
            .implements_span
            .source_text(src),
        "implements"
    );
    assert_eq!(cls.body.brace_span.0.source_text(src), "{");
    assert_eq!(cls.body.brace_span.1.source_text(src), "}");

    let ClassBodyDecl::Field(field) = &cls.body.declarations[0] else {
        panic!("expected a field");
    };
    assert_eq!(field.modifiers[0].span().source_text(src), "private");
    assert_eq!(field.ty.span().source_text(src), "int");
    assert_eq!(
        field.declarators[0]
            .name
            .as_ref()
            .unwrap()
            .span
            .source_text(src),
        "field"
    );
    assert_eq!(field.semi_span.source_text(src), ";");

    let ClassBodyDecl::Constructor(ctor) = &cls.body.declarations[1] else {
        panic!("expected a constructor");
    };
    assert_eq!(ctor.paren_span.0.source_text(src), "(");
    assert_eq!(ctor.paren_span.1.source_text(src), ")");
    assert_eq!(ctor.body.brace_span.0.source_text(src), "{");
    assert_eq!(ctor.body.brace_span.1.source_text(src), "}");
    let FormalParameter::VarArgs {
        ellipsis_span,
        name,
        ..
    } = &ctor.params[1]
    else {
        panic!("expected a varargs parameter");
    };
    assert_eq!(ellipsis_span.source_text(src), "...");
    assert_eq!(name.as_ref().unwrap().span.source_text(src), "names");

    let ClassBodyDecl::Method(m) = &cls.body.declarations[2] else {
        panic!("expected a method");
    };
    assert_eq!(m.paren_span.0.source_text(src), "(");
    assert_eq!(m.paren_span.1.source_text(src), ")");

    let body = m.body.as_ref().unwrap();
    assert_eq!(body.brace_span.0.source_text(src), "{");
    assert_eq!(body.brace_span.1.source_text(src), "}");
    let Stmt::LocalVarDecl(decl) = &body.stmts[0] else {
        panic!("expected a local variable declaration");
    };
    assert_eq!(decl.semi_span.source_text(src), ";");
    let Stmt::Expr(expr_stmt) = &body.stmts[1] else {
        panic!("expected an expression statement");
    };
    assert_eq!(expr_stmt.semi_span.source_text(src), ";");
}

#[test]
fn test_generic_and_delimiter_spans() {
    let src = "@Deprecated\n\
@SuppressWarnings(value = \"x\")\n\
class G<T extends Number> {\n\
    java.util.List<String> items;\n\n\
    void m(java.util.Map<String, java.util.List<Integer>> map) {\n\
        switch (1) {\n\
            case 1: break;\n\
            default: break;\n\
        }\n\
        try (AutoCloseable c = null) {\n\
            int n = (int) 1L;\n\
        } catch (Throwable t) {\n\
        } finally {\n\
        }\n\
    }\n\
}\n";
    let cu: CompilationUnit = parse_str(src).unwrap();

    let TypeDecl::Class(cls) = &type_decls(&cu)[0] else {
        panic!("expected a class");
    };

    // @Deprecated is a marker; @SuppressWarnings(value = "x") is a normal annotation.
    let Modifier::Annotation(Annotation::Marker { at_token, .. }) = &cls.modifiers[0] else {
        panic!("expected a marker annotation");
    };
    assert_eq!(at_token.source_text(src), "@");
    let Modifier::Annotation(Annotation::Normal {
        paren_span, pairs, ..
    }) = &cls.modifiers[1]
    else {
        panic!("expected a normal annotation");
    };
    assert_eq!(paren_span.0.source_text(src), "(");
    assert_eq!(paren_span.1.source_text(src), ")");
    assert_eq!(pairs[0].eq_span.source_text(src), "=");

    let type_params = cls.type_params.as_ref().unwrap();
    assert_eq!(type_params.lt_span.source_text(src), "<");
    assert_eq!(type_params.gt_spans[0].source_text(src), ">");
    assert_eq!(
        type_params.params[0]
            .bound
            .as_ref()
            .unwrap()
            .extends_span
            .source_text(src),
        "extends"
    );

    // `java.util.List<String>`: the path span must cover the type arguments.
    let ClassBodyDecl::Field(field) = &cls.body.declarations[0] else {
        panic!("expected a field");
    };
    let Type::Reference(ReferenceType::ClassOrInterfaceType(list_ty)) = &field.ty else {
        panic!("expected a class or interface type");
    };
    assert_eq!(list_ty.path.span.source_text(src), "java.util.List<String>");
    let list_args = list_ty.path.segments.last().unwrap().args.as_ref().unwrap();
    assert_eq!(list_args.lt_token.source_text(src), "<");
    assert_eq!(list_args.gt_tokens[0].source_text(src), ">");

    // `Map<String, List<Integer>>` closes with a `>>` that must be split into two
    // separate `>` spans, one per level of nesting.
    let ClassBodyDecl::Method(m) = &cls.body.declarations[1] else {
        panic!("expected a method");
    };
    let FormalParameter::Normal { ty, .. } = &m.params[0] else {
        panic!("expected a normal parameter");
    };
    let Type::Reference(ReferenceType::ClassOrInterfaceType(map_ty)) = ty else {
        panic!("expected a class or interface type");
    };
    assert_eq!(
        map_ty.path.span.source_text(src),
        "java.util.Map<String, java.util.List<Integer>>"
    );
    let map_args = map_ty.path.segments.last().unwrap().args.as_ref().unwrap();
    assert_eq!(map_args.gt_tokens.len(), 1);
    assert_eq!(map_args.gt_tokens[0].source_text(src), ">");
    let TypeArgument::Type(inner) = &map_args.args[1] else {
        panic!("expected a type argument");
    };
    let Type::Reference(ReferenceType::ClassOrInterfaceType(inner_ty)) = inner.as_ref() else {
        panic!("expected a class or interface type");
    };
    assert_eq!(
        inner_ty.path.span.source_text(src),
        "java.util.List<Integer>"
    );
    let inner_args = inner_ty
        .path
        .segments
        .last()
        .unwrap()
        .args
        .as_ref()
        .unwrap();
    assert_eq!(inner_args.gt_tokens[0].source_text(src), ">");
    // The two `>` halves must be adjacent halves of the same `>>` token.
    assert_eq!(
        inner_args.gt_tokens[0].start() + 1,
        map_args.gt_tokens[0].start()
    );

    let body = m.body.as_ref().unwrap();
    let Stmt::Switch(switch_stmt) = &body.stmts[0] else {
        panic!("expected a switch statement");
    };
    assert_eq!(switch_stmt.switch_span.source_text(src), "switch");
    assert_eq!(switch_stmt.paren_span.0.source_text(src), "(");
    assert_eq!(switch_stmt.paren_span.1.source_text(src), ")");
    assert_eq!(switch_stmt.brace_span.0.source_text(src), "{");
    assert_eq!(switch_stmt.brace_span.1.source_text(src), "}");
    assert_eq!(switch_stmt.cases[0].colon_span.source_text(src), ":");

    let Stmt::Try(try_stmt) = &body.stmts[1] else {
        panic!("expected a try statement");
    };
    let TryStmt::TryWithResources {
        paren_span,
        block,
        catches,
        finally_block,
        ..
    } = try_stmt
    else {
        panic!("expected a try-with-resources statement");
    };
    assert_eq!(paren_span.0.source_text(src), "(");
    assert_eq!(paren_span.1.source_text(src), ")");
    assert_eq!(block.brace_span.0.source_text(src), "{");
    assert_eq!(block.brace_span.1.source_text(src), "}");
    assert_eq!(catches[0].paren_span.0.source_text(src), "(");
    assert_eq!(catches[0].paren_span.1.source_text(src), ")");
    assert_eq!(
        finally_block.as_ref().unwrap().0.source_text(src),
        "finally"
    );

    // `int n = (int) 1L;`: the cast keeps its own parentheses.
    let Stmt::LocalVarDecl(decl) = &block.stmts[0] else {
        panic!("expected a local variable declaration");
    };
    let Some(Expr::Cast(cast)) = decl.declarators[0].initializer.as_ref() else {
        panic!("expected a cast expression");
    };
    assert_eq!(cast.paren_span.0.source_text(src), "(");
    assert_eq!(cast.paren_span.1.source_text(src), ")");
}

#[test]
fn test_failed_speculation_does_not_duplicate_comments() {
    // `a /* c */ = 1;` first looks like a local variable declaration, so the
    // parser speculatively parses a type and trips over the comment before it
    // fails. Backtracking must roll that buffered comment back, otherwise the
    // successful parse records it a second time.
    let src = "class A { void m() { a /* c */ = 1; } }";
    let comment_start = src.find("/* c */").unwrap();
    let comment_span = format!(
        "Span({}..{})",
        comment_start,
        comment_start + "/* c */".len()
    );
    let cu: CompilationUnit = parse_str(src).unwrap();
    let debug = format!("{:#?}", cu);
    assert_eq!(
        debug.matches(&comment_span).count(),
        1,
        "the comment was recorded more than once"
    );
}
