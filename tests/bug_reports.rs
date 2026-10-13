//! Characterization tests for the known parser failures in java-lang 0.3.2.
//! Each test embeds a reduced Java example and links directly to its upstream
//! occurrences. Source links are pinned to commits to preserve line numbers.
//! These tests pass when the reported Java input is rejected. When fixing a bug,
//! convert its assertion to expect successful parsing.

use java_lang::{CompilationUnit, parse_str};

#[allow(dead_code)]
fn rejects(source: &str) {
    let error = parse_str::<CompilationUnit>(source)
        .expect_err("reported bug did not reproduce: Java example parsed successfully");
    println!("{error}");
}

fn accepts(source: &str) {
    parse_str::<CompilationUnit>(source).expect("positive control failed to parse");
}

/// Type-use annotations before varargs are rejected.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/google/guava/blob/6b3a8d7498a7fde122967ba37a023ba3dcb47792/guava/src/com/google/common/base/Preconditions.java#L162>.
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/junit-platform-commons/src/main/java/org/junit/platform/commons/util/ClassUtils.java#L68>.
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/junit-platform-commons/src/main/java/org/junit/platform/commons/util/ReflectionUtils.java#L848>.
/// Source: <https://github.com/spring-projects/spring-boot/blob/bd0300809876f7aeec5e3638a487c8c46a3219ec/core/spring-boot-autoconfigure/src/main/java/org/springframework/boot/autoconfigure/condition/ConditionMessage.java#L348>.
/// Source: <https://github.com/spring-projects/spring-boot/blob/bd0300809876f7aeec5e3638a487c8c46a3219ec/core/spring-boot/src/main/java/org/springframework/boot/context/properties/bind/Bindable.java#L183>.
#[test]
fn bug_01_varargs_annotation() {
    accepts(
        r#"class Foo {
    static String bar(Object @Nullable ... args) {
        return null;
    }
}
"#,
    );
}

/// Varargs record components are rejected.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/platform-tests/src/test/java/org/junit/platform/commons/util/CollectionUtilsTests.java#L362>.
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/platform-tests/src/test/java/org/junit/platform/console/ConsoleDetailsTests.java#L197>.
/// Source: <https://github.com/spring-projects/spring-boot/blob/bd0300809876f7aeec5e3638a487c8c46a3219ec/buildSrc/src/main/java/org/springframework/boot/build/antora/Extensions.java#L77>.
/// Source: <https://github.com/spring-projects/spring-boot/blob/bd0300809876f7aeec5e3638a487c8c46a3219ec/loader/spring-boot-loader/src/main/java/org/springframework/boot/loader/launch/PropertiesLauncher.java#L597>.
#[test]
fn bug_02_record_varargs() {
    accepts(
        r#"record Foo(Object... elements) {
}
"#,
    );
}

/// Module requires modifiers fail when static precedes transitive.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/junit-platform-engine/src/main/java/module-info.java#L21>.
#[test]
fn bug_03_requires_order() {
    accepts(
        r#"module org.example {
    requires static transitive org.apiguardian.api;
}
"#,
    );
}

/// Annotations on module declarations are rejected.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/junit-vintage-engine/src/main/java/module-info.java#L19>.
#[test]
fn bug_04_module_annotation() {
    accepts(
        r#"@SuppressWarnings("deprecation")
module org.example {
}
"#,
    );
}

/// Unnamed lambda parameters are rejected.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/jupiter-tests/src/test/java/org/junit/jupiter/engine/extension/CloseablePathTests.java#L134>.
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/jupiter-tests/src/test/java/org/junit/jupiter/engine/extension/TempDirectoryTests.java#L1261>.
#[test]
fn bug_05_unnamed_lambdas() {
    accepts(
        r#"class Foo {
    void bar() {
        Runnable r = () -> {};
        java.util.function.BiConsumer<String, String> c = (_, _) -> {};
        java.util.function.Function<String, Integer> f = _ -> 0;
    }
}
"#,
    );
}

/// Three levels of generic nesting in an explicit type witness fail.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/google/guava/blob/6b3a8d7498a7fde122967ba37a023ba3dcb47792/guava/src/com/google/common/collect/TreeRangeSet.java#L796>.
/// Source: <https://github.com/google/guava/blob/6b3a8d7498a7fde122967ba37a023ba3dcb47792/guava-tests/test/com/google/common/collect/ImmutableBiMapFloodingTest.java#L42>.
/// Source: <https://github.com/spring-projects/spring-boot/blob/bd0300809876f7aeec5e3638a487c8c46a3219ec/core/spring-boot/src/test/java/org/springframework/boot/context/properties/bind/ValueObjectBinderTests.java#L351>.
/// Source: <https://github.com/spring-projects/spring-boot/blob/bd0300809876f7aeec5e3638a487c8c46a3219ec/core/spring-boot/src/test/java/org/springframework/boot/context/properties/bind/MapBinderTests.java#L143>.
#[test]
fn bug_06_nested_type_witness() {
    accepts(
        r#"class Foo {
    void bar() {
        a.<B<C<D>>>m();
    }
}
"#,
    );
}

/// Module import declarations are rejected.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/platform-tooling-support-tests/projects/junit-start/modular/p/MultiplicationTests.java#L13>.
#[test]
fn bug_07_module_import() {
    accepts(
        r#"import module org.junit.jupiter.api;

class Foo {
}
"#,
    );
}

/// Top-level methods in implicitly declared classes are rejected.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/platform-tooling-support-tests/projects/junit-start/compact/JUnitRun.java#L13>.
/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/platform-tooling-support-tests/projects/junit-start/compact/JUnitRunClass.java#L13>.
#[test]
fn bug_08_implicit_class() {
    accepts(
        r#"void main() {
    System.out.println("hi");
}
"#,
    );
}

/// Type-use annotations before wildcard type arguments are rejected.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/apache/calcite/blob/8f27e24a02e7b76fc40b6cfd9b1d2cc96bc8b1b6/core/src/main/java/org/apache/calcite/sql/type/CompositeOperandTypeChecker.java#L93>.
#[test]
fn bug_09_annotated_wildcard() {
    accepts(
        r#"class Foo {
    ImmutableList<@UnknownKeyFor ? extends Bar> x;
}
"#,
    );
}

/// Qualified superclass constructor invocations are rejected.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/google/guava/blob/6b3a8d7498a7fde122967ba37a023ba3dcb47792/guava-tests/test/com/google/common/reflect/TypeTokenResolutionTest.java#L311>.
#[test]
fn bug_10_qualified_super() {
    accepts(
        r#"class Outer {
    class Inner {
    }
}

class Sub extends Outer.Inner {
    Sub(Outer outer) {
        outer.super();
    }
}
"#,
    );
}

/// Generic constructor references fail on qualified paths.
///
/// Minimal example adapted from the reported occurrences.
///
/// Source: <https://github.com/google/guava/blob/6b3a8d7498a7fde122967ba37a023ba3dcb47792/guava/src/com/google/common/collect/CollectCollectors.java#L192>.
#[test]
fn bug_11_qualified_constructor_reference() {
    accepts(
        r#"class Foo {
    void bar() {
        x = Builder<K, V>::new;             // OK: single-segment path
        y = ImmutableMap.Builder<K, V>::new; // fails: multi-segment path
    }
}
"#,
    );
}

/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/jupiter-tests/src/test/java/org/junit/jupiter/engine/extension/TempDirectoryTests.java#L1261>.
#[test]
fn bug_05_single_unnamed_lambda() {
    accepts(
        "class Foo { void bar() { java.util.function.Function<String, Integer> f = _ -> 0; } }",
    );
}

/// Source: <https://github.com/junit-team/junit5/blob/1ef55a6a9a2430ae274165bfcc4251d5e147b803/jupiter-tests/src/test/java/org/junit/jupiter/engine/extension/CloseablePathTests.java#L134>.
#[test]
fn bug_05_multiple_unnamed_lambda() {
    accepts(
        "class Foo { void bar() { java.util.function.BiConsumer<String, String> c = (_, _) -> {}; } }",
    );
}

/// Two levels and three levels of generic nesting both parse successfully.
///
/// Source: <https://github.com/google/guava/blob/6b3a8d7498a7fde122967ba37a023ba3dcb47792/guava/src/com/google/common/collect/TreeRangeSet.java#L796>.
/// Source: <https://github.com/spring-projects/spring-boot/blob/bd0300809876f7aeec5e3638a487c8c46a3219ec/core/spring-boot/src/test/java/org/springframework/boot/context/properties/bind/ValueObjectBinderTests.java#L351>.
#[test]
fn bug_06_additional_examples() {
    accepts(
        r#"class Foo {
    void bar() {
        a.<B<C>>m(); // parses correctly
    }
}
"#,
    );
    accepts(
        r#"class Foo {
    void bar() {
        a.<B<C<D>>>m(); // also parses correctly
    }
}
"#,
    );
}

/// Generic method references fail on qualified paths, too.
///
/// Source: <https://github.com/apache/calcite/blob/8f27e24a02e7b76fc40b6cfd9b1d2cc96bc8b1b6/core/src/main/java/org/apache/calcite/plan/volcano/RelSubset.java#L671>.
#[test]
fn bug_11_qualified_method_reference() {
    accepts(
        r#"class Foo {
    void bar() {
        z = Map.Entry<String, Long>::getValue; // fails the same way
    }
}
"#,
    );
}
