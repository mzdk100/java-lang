//! Parse implementations for all AST types.

use crate::{
    Result,
    ast::*,
    ident::Ident,
    parse::{Parse, ParseStream},
    span::Span,
    token::TokenKind,
};

// ============================================================================
// CompilationUnit
// ============================================================================

impl Parse for CompilationUnit {
    fn parse(input: &ParseStream) -> Result<Self> {
        let mut package = None;
        let mut imports = Vec::new();
        let mut module = None;
        let mut type_decls = Vec::new();

        while !input.is_empty() {
            if input.is(&TokenKind::Semicolon) {
                let sp = input.peek().span;
                input.next();
                type_decls.push(TypeDecl::Empty(sp));
                continue;
            }
            if input.is(&TokenKind::At) || input.is(&TokenKind::Package) {
                // Try parsing package declaration (possibly with annotations)
                let saved = input.cursor();
                let pkg_result = input.try_parse(PackageDecl::parse);
                if let Some(pkg) = pkg_result {
                    package = Some(pkg);
                    continue;
                }
                input.set_cursor(saved);
            }
            if input.is(&TokenKind::Import) {
                imports.push(input.parse()?);
                continue;
            }
            if input.is_ident("module") || input.is(&TokenKind::Open) {
                module = Some(input.parse()?);
                continue;
            }
            type_decls.push(input.parse()?);
        }

        Ok(Self {
            package,
            imports,
            type_decls,
            module,
        })
    }
}

// ============================================================================
// PackageDecl
// ============================================================================

impl Parse for PackageDecl {
    fn parse(input: &ParseStream) -> Result<Self> {
        let start = input.peek().span;
        let mut annotations = Vec::new();
        while input.is(&TokenKind::At) {
            annotations.push(input.parse::<Annotation>()?);
        }
        let package_span = input.peek().span;
        input.expect(TokenKind::Package)?;
        let name = parse_path(input)?;
        input.expect(TokenKind::Semicolon)?;
        let semi_span = input.peek().span;
        Ok(Self {
            annotations,
            package_span,
            name,
            semi_span,
            span: start.join(semi_span),
        })
    }
}

// ============================================================================
// ImportDecl
// ============================================================================

impl Parse for ImportDecl {
    fn parse(input: &ParseStream) -> Result<Self> {
        let import_span = input.peek().span;
        input.expect(TokenKind::Import)?;

        let is_static = input.eat(&TokenKind::Static);
        let static_span = if is_static {
            Some(input.peek().span)
        } else {
            None
        };

        let path = parse_path(input)?;

        if input.eat(&TokenKind::Dot) && input.eat(&TokenKind::Star) {
            let star_span = input.peek().span;
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            if is_static {
                Ok(Self::StaticOnDemand {
                    import_span,
                    static_span: static_span.unwrap(),
                    path,
                    star_span,
                    semi_span,
                })
            } else {
                Ok(Self::TypeOnDemand {
                    import_span,
                    path,
                    star_span,
                    semi_span,
                })
            }
        } else {
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            if is_static {
                let member = path.last_ident().clone();
                Ok(Self::SingleStatic {
                    import_span,
                    static_span: static_span.unwrap(),
                    path,
                    member,
                    semi_span,
                })
            } else {
                Ok(Self::SingleType {
                    import_span,
                    path,
                    semi_span,
                })
            }
        }
    }
}

// ============================================================================
// ModuleDecl
// ============================================================================

impl Parse for ModuleDecl {
    fn parse(input: &ParseStream) -> Result<Self> {
        let mut annotations = Vec::new();
        while input.is(&TokenKind::At) {
            annotations.push(input.parse::<Annotation>()?);
        }
        let open_span = if input.eat(&TokenKind::Open) {
            Some(input.peek().span)
        } else {
            None
        };
        let module_span = input.peek().span;
        if !input.is_ident("module") {
            return Err(crate::error::Error::new(
                input.peek().span,
                "expected 'module'",
            ));
        }
        input.next();
        let name = parse_path(input)?;
        input.expect(TokenKind::LBrace)?;
        let brace_start = input.peek().span;
        let mut directives = Vec::new();
        while !input.is(&TokenKind::RBrace) && !input.is_empty() {
            directives.push(parse_module_directive(input)?);
        }
        input.expect(TokenKind::RBrace)?;
        let brace_end = input.peek().span;
        Ok(Self {
            annotations,
            open_span,
            module_span,
            name,
            brace_span: (brace_start, brace_end),
            directives,
        })
    }
}

fn parse_module_directive(input: &ParseStream) -> Result<ModuleDirective> {
    let span = input.peek().span;
    match &input.peek().kind {
        TokenKind::Requires => {
            input.next();
            let requires_span = span;
            let mut modifiers = Vec::new();
            while input.is(&TokenKind::Transitive) {
                modifiers.push(RequiresModifier::Transitive(input.next().span));
            }
            if input.is(&TokenKind::Static) {
                modifiers.push(RequiresModifier::Static(input.next().span));
            }
            let module = parse_path(input)?;
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            Ok(ModuleDirective::Requires {
                requires_span,
                modifiers,
                module,
                semi_span,
            })
        }
        TokenKind::Exports => {
            input.next();
            let exports_span = span;
            let pkg = parse_path(input)?;
            let to_modules = if input.eat(&TokenKind::To) {
                let to_span = input.peek().span;
                let mut mods = vec![parse_path(input)?];
                while input.eat(&TokenKind::Comma) {
                    mods.push(parse_path(input)?);
                }
                Some((to_span, mods))
            } else {
                None
            };
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            Ok(ModuleDirective::Exports {
                exports_span,
                pkg,
                to_modules,
                semi_span,
            })
        }
        TokenKind::Opens => {
            input.next();
            let opens_span = span;
            let pkg = parse_path(input)?;
            let to_modules = if input.eat(&TokenKind::To) {
                let to_span = input.peek().span;
                let mut mods = vec![parse_path(input)?];
                while input.eat(&TokenKind::Comma) {
                    mods.push(parse_path(input)?);
                }
                Some((to_span, mods))
            } else {
                None
            };
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            Ok(ModuleDirective::Opens {
                opens_span,
                pkg,
                to_modules,
                semi_span,
            })
        }
        TokenKind::Uses => {
            input.next();
            let uses_span = span;
            let ty = parse_type(input)?;
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            Ok(ModuleDirective::Uses {
                uses_span,
                ty,
                semi_span,
            })
        }
        TokenKind::Provides => {
            input.next();
            let provides_span = span;
            let ty = parse_type(input)?;
            input.expect(TokenKind::With)?;
            let with_span = input.peek().span;
            let mut impls = vec![parse_type(input)?];
            while input.eat(&TokenKind::Comma) {
                impls.push(parse_type(input)?);
            }
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            Ok(ModuleDirective::Provides {
                provides_span,
                ty,
                with_span,
                impls,
                semi_span,
            })
        }
        _ => Err(crate::error::Error::new(span, "expected module directive")),
    }
}

// ============================================================================
// TypeDecl
// ============================================================================

impl Parse for TypeDecl {
    fn parse(input: &ParseStream) -> Result<Self> {
        let annotations = parse_annotations(input)?;
        let modifiers = parse_modifiers_after_annotations(&annotations, input);

        match &input.peek().kind {
            TokenKind::Class => {
                let decl: ClassDecl = parse_class_decl(input, modifiers)?;
                Ok(Self::Class(decl))
            }
            TokenKind::Interface => {
                let decl: InterfaceDecl = parse_interface_decl(input, modifiers)?;
                Ok(Self::Interface(decl))
            }
            TokenKind::Enum => {
                let decl: EnumDecl = parse_enum_decl(input, modifiers)?;
                Ok(Self::Enum(decl))
            }
            TokenKind::Record => {
                let decl: RecordDecl = parse_record_decl(input, modifiers)?;
                Ok(Self::Record(decl))
            }
            TokenKind::At => {
                // Check for @interface (annotation type declaration)
                let saved = input.cursor();
                let at_span = input.peek().span;
                input.next(); // consume '@'
                if input.is(&TokenKind::Interface) {
                    input.next(); // consume 'interface'
                    let name = input.parse_ident()?;
                    let body = parse_annotation_type_body(input)?;
                    return Ok(Self::AnnotationType(AnnotationInterfaceDecl {
                        modifiers,
                        at_span,
                        interface_span: name.span(),
                        name,
                        body,
                    }));
                }
                input.set_cursor(saved);

                // Otherwise it's an annotation before a type declaration
                let mut all_mods = modifiers.clone();
                let ann = input.parse::<Annotation>()?;
                all_mods.push(Modifier::Annotation(ann));
                match &input.peek().kind {
                    TokenKind::Class => {
                        let decl = parse_class_decl(input, all_mods)?;
                        Ok(Self::Class(decl))
                    }
                    TokenKind::Interface => {
                        let decl = parse_interface_decl(input, all_mods)?;
                        Ok(Self::Interface(decl))
                    }
                    TokenKind::Enum => {
                        let decl = parse_enum_decl(input, all_mods)?;
                        Ok(Self::Enum(decl))
                    }
                    TokenKind::Record => {
                        let decl = parse_record_decl(input, all_mods)?;
                        Ok(Self::Record(decl))
                    }
                    _ => Err(crate::error::Error::new(
                        input.peek().span,
                        "expected type declaration",
                    )),
                }
            }
            _ => Err(crate::error::Error::new(
                input.peek().span,
                "expected type declaration (class, interface, enum, or record)",
            )),
        }
    }
}

// ============================================================================
// Modifiers and Annotations
// ============================================================================

fn parse_annotations(input: &ParseStream) -> Result<Vec<Annotation>> {
    let mut anns = Vec::new();
    while input.is(&TokenKind::At) {
        // Stop before @interface (annotation type declaration)
        let saved = input.cursor();
        input.next(); // consume '@'
        if input.is(&TokenKind::Interface) {
            input.set_cursor(saved);
            break;
        }
        input.set_cursor(saved);
        anns.push(input.parse::<Annotation>()?);
    }
    Ok(anns)
}

fn is_modifier_keyword(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Public
            | TokenKind::Protected
            | TokenKind::Private
            | TokenKind::Static
            | TokenKind::Abstract
            | TokenKind::Final
            | TokenKind::Synchronized
            | TokenKind::Native
            | TokenKind::Strictfp
            | TokenKind::Transient
            | TokenKind::Volatile
            | TokenKind::Default
            | TokenKind::Sealed
            | TokenKind::NonSealed
    )
}

fn parse_modifier(input: &ParseStream) -> Option<Modifier> {
    let modifier = match &input.peek().kind {
        TokenKind::Public => Modifier::Public(input.next().span),
        TokenKind::Protected => Modifier::Protected(input.next().span),
        TokenKind::Private => Modifier::Private(input.next().span),
        TokenKind::Static => Modifier::Static(input.next().span),
        TokenKind::Abstract => Modifier::Abstract(input.next().span),
        TokenKind::Final => Modifier::Final(input.next().span),
        TokenKind::Synchronized => Modifier::Synchronized(input.next().span),
        TokenKind::Native => Modifier::Native(input.next().span),
        TokenKind::Strictfp => Modifier::Strictfp(input.next().span),
        TokenKind::Transient => Modifier::Transient(input.next().span),
        TokenKind::Volatile => Modifier::Volatile(input.next().span),
        TokenKind::Default => Modifier::Default(input.next().span),
        TokenKind::Sealed => Modifier::Sealed(input.next().span),
        TokenKind::NonSealed => Modifier::NonSealed(input.next().span),
        TokenKind::Ident(s) if s == "non" => {
            // non-sealed is lexed as three tokens: Ident("non"), Minus, Sealed
            let saved = input.cursor();
            let non_span = input.next().span;
            if input.eat(&TokenKind::Minus) && input.is(&TokenKind::Sealed) {
                let sealed_span = input.next().span;
                Modifier::NonSealed(non_span.join(sealed_span))
            } else {
                input.set_cursor(saved);
                return None;
            }
        }
        TokenKind::At => {
            let ann = input.try_parse(Annotation::parse);
            match ann {
                Some(a) => Modifier::Annotation(a),
                None => return None,
            }
        }
        _ => return None,
    };
    Some(modifier)
}

fn parse_modifiers(input: &ParseStream) -> Vec<Modifier> {
    let mut mods = Vec::new();
    while let Some(m) = parse_modifier(input) {
        mods.push(m);
    }
    mods
}

fn parse_modifiers_after_annotations(
    annotations: &[Annotation],
    input: &ParseStream,
) -> Vec<Modifier> {
    let mut mods: Vec<Modifier> = annotations
        .iter()
        .map(|a| Modifier::Annotation(a.clone()))
        .collect();
    while let Some(m) = parse_modifier(input) {
        mods.push(m);
    }
    mods
}

// ============================================================================
// Annotation
// ============================================================================

impl Parse for Annotation {
    fn parse(input: &ParseStream) -> Result<Self> {
        let at_token = input.peek().span;
        input.expect(TokenKind::At)?;
        let name = parse_annotation_path(input)?;

        if input.is(&TokenKind::LParen) {
            input.next();
            let open = input.peek().span;

            if input.is(&TokenKind::RParen) {
                input.next();
                return Ok(Self::Marker { at_token, name });
            }

            // Try to parse as Normal (key=value pairs)
            // Check if first thing is ident followed by '=' or ','
            if input.is_any_ident() {
                let saved = input.cursor();
                let ident = input.parse_ident().ok();
                if let Some(ident) = ident {
                    if input.eat(&TokenKind::Eq) {
                        // Normal annotation
                        let eq_span = input.peek().span;
                        let value = parse_element_value(input)?;
                        let mut pairs = vec![ElementValuePair {
                            key: ident,
                            eq_span,
                            value,
                        }];
                        while input.eat(&TokenKind::Comma) {
                            let key = input.parse_ident()?;
                            input.expect(TokenKind::Eq)?;
                            let eq_span = input.peek().span;
                            let value = parse_element_value(input)?;
                            pairs.push(ElementValuePair {
                                key,
                                eq_span,
                                value,
                            });
                        }
                        input.expect(TokenKind::RParen)?;
                        let close = input.peek().span;
                        return Ok(Self::Normal {
                            at_token,
                            name,
                            paren_span: (open, close),
                            pairs,
                        });
                    } else {
                        // It was just a value, backtrack
                        input.set_cursor(saved);
                    }
                } else {
                    input.set_cursor(saved);
                }
            }

            // Single element annotation
            let value = parse_element_value(input)?;
            input.expect(TokenKind::RParen)?;
            let eq_token = input.peek().span;
            Ok(Self::SingleElement {
                at_token,
                name,
                eq_token,
                value: Box::new(value),
            })
        } else {
            Ok(Self::Marker { at_token, name })
        }
    }
}

fn parse_element_value(input: &ParseStream) -> Result<ElementValue> {
    if input.is(&TokenKind::At) {
        let ann = input.parse::<Annotation>()?;
        Ok(ElementValue::Annotation(ann))
    } else if input.is(&TokenKind::LBrace) {
        let open = input.peek().span;
        input.next();
        let mut values = Vec::new();
        let mut trailing_comma = false;
        while !input.is(&TokenKind::RBrace) && !input.is_empty() {
            values.push(parse_element_value(input)?);
            if input.eat(&TokenKind::Comma) {
                trailing_comma = true;
            } else {
                trailing_comma = false;
                break;
            }
        }
        input.expect(TokenKind::RBrace)?;
        let close = input.peek().span;
        Ok(ElementValue::Array {
            brace_span: (open, close),
            values,
            trailing_comma,
        })
    } else {
        let expr = parse_expression(input)?;
        Ok(ElementValue::Expr(expr))
    }
}

fn parse_implements_clause(input: &ParseStream) -> Option<ImplementsClause> {
    if input.eat(&TokenKind::Implements) {
        let implements_span = input.peek().span;
        let supertypes = parse_type_list(input);
        Some(ImplementsClause {
            implements_span,
            supertypes,
        })
    } else {
        None
    }
}

fn parse_array_init(input: &ParseStream) -> Result<ArrayInitExpr> {
    let open = input.peek().span;
    input.next();
    let mut elements = Vec::new();
    let mut trailing_comma = false;
    while !input.is(&TokenKind::RBrace) && !input.is_empty() {
        elements.push(parse_expression(input)?);
        if input.eat(&TokenKind::Comma) {
            trailing_comma = true;
        } else {
            trailing_comma = false;
            break;
        }
    }
    input.expect(TokenKind::RBrace)?;
    let close = input.peek().span;
    Ok(ArrayInitExpr {
        brace_span: (open, close),
        elements,
        trailing_comma,
    })
}

// ============================================================================
// ClassDecl
// ============================================================================

fn parse_class_decl(input: &ParseStream, modifiers: Vec<Modifier>) -> Result<ClassDecl> {
    let class_span = input.peek().span;
    input.expect(TokenKind::Class)?;
    let name = input.parse_ident()?;
    let type_params = parse_optional_type_params(input);
    let extends_clause = if input.eat(&TokenKind::Extends) {
        let extends_span = input.peek().span;
        let supertype = parse_type(input)?;
        Some(ExtendsClause {
            extends_span,
            supertype,
        })
    } else {
        None
    };
    let implements_clause = parse_implements_clause(input);
    let permits_clause = if input.eat(&TokenKind::Permits) {
        let permits_span = input.peek().span;
        let types = parse_type_list(input);
        Some(PermitsClause {
            permits_span,
            types,
        })
    } else {
        None
    };
    let body = parse_class_body_decl_list(input)?;
    Ok(ClassDecl {
        modifiers,
        class_span,
        name,
        type_params,
        extends_clause,
        implements_clause,
        permits_clause,
        body,
    })
}

fn parse_class_body_decl_list(input: &ParseStream) -> Result<ClassBodyDeclList> {
    input.expect(TokenKind::LBrace)?;
    let open = input.peek().span;
    let mut declarations = Vec::new();
    while !input.is(&TokenKind::RBrace) && !input.is_empty() {
        if input.is(&TokenKind::Semicolon) {
            let sp = input.next().span;
            declarations.push(ClassBodyDecl::Empty(sp));
            continue;
        }
        declarations.push(parse_class_body_decl(input)?);
    }
    input.expect(TokenKind::RBrace)?;
    let close = input.peek().span;
    Ok(ClassBodyDeclList {
        brace_span: (open, close),
        declarations,
    })
}

fn parse_class_body_decl(input: &ParseStream) -> Result<ClassBodyDecl> {
    let modifiers = parse_modifiers(input);

    match &input.peek().kind {
        TokenKind::LBrace => {
            let block = parse_block(input)?;
            Ok(ClassBodyDecl::InstanceInit(InstanceInit { block }))
        }
        TokenKind::Static => {
            // Could be static initializer or static modifier for method/field
            let static_span = input.peek().span;
            input.next();
            if input.is(&TokenKind::LBrace) {
                let block = parse_block(input)?;
                Ok(ClassBodyDecl::StaticInit(StaticInit { static_span, block }))
            } else {
                let mut rest_mods = modifiers;
                rest_mods.push(Modifier::Static(static_span));
                let rest = parse_modifiers(input);
                rest_mods.extend(rest);
                parse_class_member(input, rest_mods)
            }
        }
        TokenKind::Class => {
            let decl = parse_class_decl(input, modifiers)?;
            Ok(ClassBodyDecl::Class(decl))
        }
        TokenKind::Interface => {
            let decl = parse_interface_decl(input, modifiers)?;
            Ok(ClassBodyDecl::Interface(decl))
        }
        TokenKind::Enum => {
            let decl = parse_enum_decl(input, modifiers)?;
            Ok(ClassBodyDecl::Enum(decl))
        }
        TokenKind::Record => {
            let decl = parse_record_decl(input, modifiers)?;
            Ok(ClassBodyDecl::Record(decl))
        }
        TokenKind::At => {
            // Could be nested @interface
            let saved = input.cursor();
            let at_span = input.peek().span;
            input.next(); // consume '@'
            if input.is(&TokenKind::Interface) {
                input.next(); // consume 'interface'
                let name = input.parse_ident()?;
                let body = parse_annotation_type_body(input)?;
                Ok(ClassBodyDecl::AnnotationType(AnnotationInterfaceDecl {
                    modifiers,
                    at_span,
                    interface_span: name.span(),
                    name,
                    body,
                }))
            } else {
                input.set_cursor(saved);
                parse_class_member(input, modifiers)
            }
        }
        _ => parse_class_member(input, modifiers),
    }
}

fn parse_class_member(input: &ParseStream, modifiers: Vec<Modifier>) -> Result<ClassBodyDecl> {
    let type_params = parse_optional_type_params(input);

    // Parse the return type (or constructor name)
    let ty = parse_type(input)?;

    if input.is(&TokenKind::LParen) {
        // Constructor: ClassName(params) { ... }
        let name = match &ty {
            Type::Reference(ReferenceType::ClassOrInterfaceType(cit)) => cit.name().clone(),
            _ => {
                return Err(crate::error::Error::new(
                    input.peek().span,
                    "expected constructor name",
                ));
            }
        };
        input.expect(TokenKind::LParen)?;
        let open = input.peek().span;
        let params = parse_formal_params_after_lparen(input)?;
        let throws_clause = parse_throws_clause(input)?;
        let body = parse_constructor_body(input)?;
        return Ok(ClassBodyDecl::Constructor(ConstructorDecl {
            modifiers,
            type_params,
            name,
            receiver_param: None,
            params,
            paren_span: (open, input.peek().span),
            throws_clause,
            body,
        }));
    }

    if input.is_any_ident() {
        let name = input.parse_ident()?;

        if input.is(&TokenKind::LParen) {
            // Method
            input.expect(TokenKind::LParen)?;
            let open = input.peek().span;
            let params = parse_formal_params_after_lparen(input)?;
            // Handle C-style array dimensions after params (e.g., int foo()[])
            let _trailing_dims = parse_array_dims(input)?;
            let throws_clause = parse_throws_clause(input)?;
            let body = if input.is(&TokenKind::LBrace) {
                Some(parse_block(input)?)
            } else {
                input.expect(TokenKind::Semicolon)?;
                None
            };
            return Ok(ClassBodyDecl::Method(MethodDecl {
                modifiers,
                type_params,
                return_type: MethodReturnType::Type(ty),
                name,
                receiver_param: None,
                params,
                paren_span: (open, input.peek().span),
                throws_clause,
                body,
            }));
        }

        // Field
        let dims = parse_array_dims(input)?;
        let mut declarators = Vec::new();
        let initializer = if input.eat(&TokenKind::Eq) {
            Some(parse_expression(input)?)
        } else {
            None
        };
        declarators.push(VariableDeclarator {
            name: Some(name),
            dims,
            initializer,
        });
        while input.eat(&TokenKind::Comma) {
            let name = input.parse_ident()?;
            let dims = parse_array_dims(input)?;
            let initializer = if input.eat(&TokenKind::Eq) {
                Some(parse_expression(input)?)
            } else {
                None
            };
            declarators.push(VariableDeclarator {
                name: Some(name),
                dims,
                initializer,
            });
        }
        input.expect(TokenKind::Semicolon)?;
        let semi_span = input.peek().span;
        return Ok(ClassBodyDecl::Field(FieldDecl {
            modifiers,
            ty,
            declarators,
            semi_span,
        }));
    }

    Err(crate::error::Error::new(
        input.peek().span,
        "expected method or field declaration",
    ))
}

fn parse_formal_params_after_lparen(input: &ParseStream) -> Result<Vec<FormalParameter>> {
    let mut params = Vec::new();
    while !input.is(&TokenKind::RParen) && !input.is_empty() {
        let modifiers = parse_modifiers(input);
        let ty = parse_type(input)?;
        // Accept regular identifiers or `this` (for receiver parameters)
        let name = if input.is(&TokenKind::This) {
            let sp = input.next().span;
            Some(Ident::new("this".to_string(), sp))
        } else {
            input.parse_ident().ok()
        };

        // Handle C-style array parameters: e.g. `char str[]` → treat as `char[] str`
        let ty = if name.is_some() && input.is(&TokenKind::LBracket) {
            let trailing_dims = parse_array_dims(input)?;
            if trailing_dims.is_empty() {
                ty
            } else {
                let start = ty.span();
                let end = trailing_dims.last().unwrap().bracket_span.1;
                let span = start.join(end);
                let mut all_dims = trailing_dims;
                // Merge with any existing array dims from the type
                if let Type::Reference(ReferenceType::Array(arr)) = &ty {
                    let mut merged = arr.dims.clone();
                    merged.extend(all_dims);
                    all_dims = merged;
                }
                Type::Reference(ReferenceType::Array(ArrayType {
                    elem_type: Box::new(match &ty {
                        Type::Reference(ReferenceType::Array(arr)) => *arr.elem_type.clone(),
                        other => other.clone(),
                    }),
                    dims: all_dims,
                    span,
                }))
            }
        } else {
            ty
        };

        if input.eat(&TokenKind::Ellipsis) {
            let ellipsis_span = input.peek().span;
            let name = input.parse_ident().ok();
            params.push(FormalParameter::VarArgs {
                modifiers,
                ty,
                ellipsis_span,
                name,
            });
        } else {
            params.push(FormalParameter::Normal {
                modifiers,
                ty,
                name,
            });
        }
        if !input.eat(&TokenKind::Comma) {
            break;
        }
    }
    input.expect(TokenKind::RParen)?;
    Ok(params)
}

fn parse_throws_clause(input: &ParseStream) -> Result<Option<ThrowsClause>> {
    if input.eat(&TokenKind::Throws) {
        let throws_span = input.peek().span;
        let types = parse_type_list(input);
        Ok(Some(ThrowsClause { throws_span, types }))
    } else {
        Ok(None)
    }
}

// ============================================================================
// InterfaceDecl
// ============================================================================

fn parse_interface_decl(input: &ParseStream, modifiers: Vec<Modifier>) -> Result<InterfaceDecl> {
    let interface_span = input.peek().span;
    input.expect(TokenKind::Interface)?;
    let name = input.parse_ident()?;
    let type_params = parse_optional_type_params(input);
    let extends_clause = if input.eat(&TokenKind::Extends) {
        let extends_span = input.peek().span;
        let supertypes = parse_type_list(input);
        Some(InterfaceExtendsClause {
            extends_span,
            supertypes,
        })
    } else {
        None
    };
    let permits_clause = if input.eat(&TokenKind::Permits) {
        let permits_span = input.peek().span;
        let types = parse_type_list(input);
        Some(PermitsClause {
            permits_span,
            types,
        })
    } else {
        None
    };

    // Check for @interface (annotation type)
    // body
    input.expect(TokenKind::LBrace)?;
    let open = input.peek().span;
    let mut members = Vec::new();
    while !input.is(&TokenKind::RBrace) && !input.is_empty() {
        if input.is(&TokenKind::Semicolon) {
            let sp = input.next().span;
            members.push(InterfaceMemberDecl::Empty(sp));
            continue;
        }
        members.push(parse_interface_member_decl(input)?);
    }
    input.expect(TokenKind::RBrace)?;
    let close = input.peek().span;

    Ok(InterfaceDecl {
        modifiers,
        interface_span,
        name,
        type_params,
        extends_clause,
        permits_clause,
        body: InterfaceBody {
            brace_span: (open, close),
            members,
        },
    })
}

fn parse_interface_member_decl(input: &ParseStream) -> Result<InterfaceMemberDecl> {
    let modifiers = parse_modifiers(input);

    match &input.peek().kind {
        TokenKind::Class => {
            let decl = parse_class_decl(input, modifiers)?;
            Ok(InterfaceMemberDecl::Class(decl))
        }
        TokenKind::Interface => {
            let decl = parse_interface_decl(input, modifiers)?;
            Ok(InterfaceMemberDecl::Interface(decl))
        }
        TokenKind::Enum => {
            let decl = parse_enum_decl(input, modifiers)?;
            Ok(InterfaceMemberDecl::Enum(decl))
        }
        TokenKind::Record => {
            let decl = parse_record_decl(input, modifiers)?;
            Ok(InterfaceMemberDecl::Record(decl))
        }
        TokenKind::At => {
            // Could be nested @interface
            let saved = input.cursor();
            let at_span = input.peek().span;
            input.next(); // consume '@'
            if input.is(&TokenKind::Interface) {
                input.next(); // consume 'interface'
                let name = input.parse_ident()?;
                let body = parse_annotation_type_body(input)?;
                Ok(InterfaceMemberDecl::AnnotationInterface(
                    AnnotationInterfaceDecl {
                        modifiers,
                        at_span,
                        interface_span: name.span(),
                        name,
                        body,
                    },
                ))
            } else {
                input.set_cursor(saved);
                parse_interface_field_or_method(input, modifiers)
            }
        }
        _ => parse_interface_field_or_method(input, modifiers),
    }
}

fn parse_interface_field_or_method(
    input: &ParseStream,
    modifiers: Vec<Modifier>,
) -> Result<InterfaceMemberDecl> {
    let type_params = parse_optional_type_params(input);
    let ty = parse_type(input)?;
    if input.is_any_ident() {
        let name = input.parse_ident()?;
        if input.is(&TokenKind::LParen) {
            // Method
            input.expect(TokenKind::LParen)?;
            let open = input.peek().span;
            let params = parse_formal_params_after_lparen(input)?;
            let throws_clause = parse_throws_clause(input)?;
            let body = if input.is(&TokenKind::LBrace) {
                Some(parse_block(input)?)
            } else {
                input.expect(TokenKind::Semicolon)?;
                None
            };
            Ok(InterfaceMemberDecl::Method(MethodDecl {
                modifiers,
                type_params,
                return_type: MethodReturnType::Type(ty),
                name,
                receiver_param: None,
                params,
                paren_span: (open, input.peek().span),
                throws_clause,
                body,
            }))
        } else {
            // Constant field
            let dims = parse_array_dims(input)?;
            let mut declarators = Vec::new();
            let initializer = if input.eat(&TokenKind::Eq) {
                Some(parse_expression(input)?)
            } else {
                None
            };
            declarators.push(VariableDeclarator {
                name: Some(name),
                dims,
                initializer,
            });
            while input.eat(&TokenKind::Comma) {
                let name = input.parse_ident()?;
                let dims = parse_array_dims(input)?;
                let initializer = if input.eat(&TokenKind::Eq) {
                    Some(parse_expression(input)?)
                } else {
                    None
                };
                declarators.push(VariableDeclarator {
                    name: Some(name),
                    dims,
                    initializer,
                });
            }
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            Ok(InterfaceMemberDecl::Field(FieldDecl {
                modifiers,
                ty,
                declarators,
                semi_span,
            }))
        }
    } else {
        Err(crate::error::Error::new(
            input.peek().span,
            "expected interface member",
        ))
    }
}

// ============================================================================
// EnumDecl
// ============================================================================

fn parse_enum_decl(input: &ParseStream, modifiers: Vec<Modifier>) -> Result<EnumDecl> {
    let enum_span = input.peek().span;
    input.expect(TokenKind::Enum)?;
    let name = input.parse_ident()?;
    let implements_clause = parse_implements_clause(input);

    input.expect(TokenKind::LBrace)?;
    let open = input.peek().span;
    let mut constants = Vec::new();
    let mut comma_span = None;

    while !input.is(&TokenKind::RBrace) && !input.is(&TokenKind::Semicolon) && !input.is_empty() {
        let annotations = parse_annotations(input)?;
        let name = input.parse_ident()?;
        let (paren_span, args) = if input.is(&TokenKind::LParen) {
            let open_p = input.peek().span;
            input.next();
            let args = input.parse_terminated(parse_expression)?;
            input.expect(TokenKind::RParen)?;
            let close_p = input.peek().span;
            (Some((open_p, close_p)), args)
        } else {
            (None, Vec::new())
        };
        let body = if input.is(&TokenKind::LBrace) {
            Some(parse_class_body_decl_list(input)?)
        } else {
            None
        };
        constants.push(EnumConstant {
            annotations,
            name,
            paren_span,
            args,
            body,
        });
        if input.eat(&TokenKind::Comma) {
            comma_span = Some(input.peek().span);
        } else {
            break;
        }
    }

    let members = if input.eat(&TokenKind::Semicolon) {
        let mut members = Vec::new();
        while !input.is(&TokenKind::RBrace) && !input.is_empty() {
            if input.is(&TokenKind::Semicolon) {
                let sp = input.next().span;
                members.push(ClassBodyDecl::Empty(sp));
                continue;
            }
            members.push(parse_class_body_decl(input)?);
        }
        members
    } else {
        Vec::new()
    };

    input.expect(TokenKind::RBrace)?;
    let close = input.peek().span;

    Ok(EnumDecl {
        modifiers,
        enum_span,
        name,
        implements_clause,
        body: EnumBody {
            brace_span: (open, close),
            constants,
            comma_span,
            members,
        },
    })
}

// ============================================================================
// RecordDecl
// ============================================================================

fn parse_record_decl(input: &ParseStream, modifiers: Vec<Modifier>) -> Result<RecordDecl> {
    let record_span = input.peek().span;
    input.expect(TokenKind::Record)?;
    let name = input.parse_ident()?;
    let type_params = parse_optional_type_params(input);

    // Parse record components
    input.expect(TokenKind::LParen)?;
    let open_p = input.peek().span;
    let mut components = Vec::new();
    while !input.is(&TokenKind::RParen) && !input.is_empty() {
        let annotations = parse_annotations(input)?;
        let ty = parse_type(input)?;
        let name = input.parse_ident()?;
        if input.eat(&TokenKind::Ellipsis) {
            let ellipsis_span = input.peek().span;
            components.push(RecordComponent::VarArgs {
                annotations,
                ty,
                ellipsis_span,
                name,
            });
        } else {
            components.push(RecordComponent::Normal {
                annotations,
                ty,
                name,
            });
        }
        if !input.eat(&TokenKind::Comma) {
            break;
        }
    }
    input.expect(TokenKind::RParen)?;
    let close_p = input.peek().span;

    let implements_clause = parse_implements_clause(input);

    // Record body
    input.expect(TokenKind::LBrace)?;
    let open = input.peek().span;
    let mut members = Vec::new();
    while !input.is(&TokenKind::RBrace) && !input.is_empty() {
        if input.is(&TokenKind::Semicolon) {
            let sp = input.next().span;
            members.push(RecordBodyDecl::Empty(sp));
            continue;
        }
        members.push(parse_record_body_decl(input)?);
    }
    input.expect(TokenKind::RBrace)?;
    let close = input.peek().span;

    Ok(RecordDecl {
        modifiers,
        record_span,
        name,
        type_params,
        components: RecordComponents {
            paren_span: (open_p, close_p),
            components,
        },
        implements_clause,
        body: RecordBody {
            brace_span: (open, close),
            members,
        },
    })
}

fn parse_record_body_decl(input: &ParseStream) -> Result<RecordBodyDecl> {
    let modifiers = parse_modifiers(input);

    // Check for compact constructor
    if input.is_any_ident() {
        let saved = input.cursor();
        let name = input.parse_ident().ok();
        if let Some(name) = name {
            if input.is(&TokenKind::LBrace) {
                let body = parse_constructor_body(input)?;
                return Ok(RecordBodyDecl::CompactConstructor(CompactConstructorDecl {
                    modifiers,
                    name,
                    body,
                }));
            }
            input.set_cursor(saved);
        } else {
            input.set_cursor(saved);
        }
    }

    match &input.peek().kind {
        TokenKind::LBrace => {
            let block = parse_block(input)?;
            Ok(RecordBodyDecl::InstanceInit(InstanceInit { block }))
        }
        TokenKind::Static => {
            let static_span = input.peek().span;
            input.next();
            if input.is(&TokenKind::LBrace) {
                let block = parse_block(input)?;
                Ok(RecordBodyDecl::StaticInit(StaticInit {
                    static_span,
                    block,
                }))
            } else {
                let mut rest_mods = modifiers;
                rest_mods.push(Modifier::Static(static_span));
                let rest = parse_modifiers(input);
                rest_mods.extend(rest);
                // Re-parse as field/method
                parse_record_body_decl_with_mods(input, rest_mods)
            }
        }
        TokenKind::Class => {
            let decl = parse_class_decl(input, modifiers)?;
            Ok(RecordBodyDecl::Class(decl))
        }
        TokenKind::Interface => {
            let decl = parse_interface_decl(input, modifiers)?;
            Ok(RecordBodyDecl::Interface(decl))
        }
        TokenKind::Enum => {
            let decl = parse_enum_decl(input, modifiers)?;
            Ok(RecordBodyDecl::Enum(decl))
        }
        TokenKind::Record => {
            let decl = parse_record_decl(input, modifiers)?;
            Ok(RecordBodyDecl::Record(decl))
        }
        _ => parse_record_body_decl_with_mods(input, modifiers),
    }
}

fn parse_record_body_decl_with_mods(
    input: &ParseStream,
    modifiers: Vec<Modifier>,
) -> Result<RecordBodyDecl> {
    let type_params = parse_optional_type_params(input);
    let ty = parse_type(input)?;
    if input.is_any_ident() {
        let name = input.parse_ident()?;
        if input.is(&TokenKind::LParen) {
            input.expect(TokenKind::LParen)?;
            let open = input.peek().span;
            let params = parse_formal_params_after_lparen(input)?;
            let throws_clause = parse_throws_clause(input)?;
            let body = if input.is(&TokenKind::LBrace) {
                Some(parse_block(input)?)
            } else {
                input.expect(TokenKind::Semicolon)?;
                None
            };
            return Ok(RecordBodyDecl::Method(MethodDecl {
                modifiers,
                type_params,
                return_type: MethodReturnType::Type(ty),
                name,
                receiver_param: None,
                params,
                paren_span: (open, input.peek().span),
                throws_clause,
                body,
            }));
        }
        let mut declarators = Vec::new();
        let dims = parse_array_dims(input)?;
        let initializer = if input.eat(&TokenKind::Eq) {
            Some(parse_expression(input)?)
        } else {
            None
        };
        declarators.push(VariableDeclarator {
            name: Some(name),
            dims,
            initializer,
        });
        while input.eat(&TokenKind::Comma) {
            let name = input.parse_ident()?;
            let dims = parse_array_dims(input)?;
            let initializer = if input.eat(&TokenKind::Eq) {
                Some(parse_expression(input)?)
            } else {
                None
            };
            declarators.push(VariableDeclarator {
                name: Some(name),
                dims,
                initializer,
            });
        }
        input.expect(TokenKind::Semicolon)?;
        let semi_span = input.peek().span;
        Ok(RecordBodyDecl::Field(FieldDecl {
            modifiers,
            ty,
            declarators,
            semi_span,
        }))
    } else {
        Err(crate::error::Error::new(
            input.peek().span,
            "expected record body member",
        ))
    }
}

fn parse_constructor_body(input: &ParseStream) -> Result<ConstructorBody> {
    input.expect(TokenKind::LBrace)?;
    let open = input.peek().span;
    let mut explicit_constructor_call = None;
    let mut stmts = Vec::new();

    if input.is(&TokenKind::This) || input.is(&TokenKind::Super) {
        let saved = input.cursor();
        let call = input.try_parse(parse_explicit_constructor_call);
        if let Some(call) = call {
            explicit_constructor_call = Some(call);
        } else {
            input.set_cursor(saved);
        }
    }

    while !input.is(&TokenKind::RBrace) && !input.is_empty() {
        stmts.push(parse_statement(input)?);
    }
    input.expect(TokenKind::RBrace)?;
    let close = input.peek().span;
    Ok(ConstructorBody {
        brace_span: (open, close),
        explicit_constructor_call,
        stmts,
    })
}

fn parse_explicit_constructor_call(input: &ParseStream) -> Result<ExplicitConstructorCall> {
    let type_args = parse_optional_type_arguments(input);
    match &input.peek().kind {
        TokenKind::This => {
            let this_span = input.peek().span;
            input.next();
            input.expect(TokenKind::LParen)?;
            let open = input.peek().span;
            let args = input.parse_terminated(parse_expression)?;
            input.expect(TokenKind::RParen)?;
            let close = input.peek().span;
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            Ok(ExplicitConstructorCall::This {
                type_args,
                this_span,
                paren_span: (open, close),
                args,
                semi_span,
            })
        }
        TokenKind::Super => {
            let super_span = input.peek().span;
            input.next();
            input.expect(TokenKind::LParen)?;
            let open = input.peek().span;
            let args = input.parse_terminated(parse_expression)?;
            input.expect(TokenKind::RParen)?;
            let close = input.peek().span;
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            Ok(ExplicitConstructorCall::Super {
                type_args,
                super_span,
                paren_span: (open, close),
                args,
                semi_span,
            })
        }
        _ => Err(crate::error::Error::new(
            input.peek().span,
            "expected this or super",
        )),
    }
}

// ============================================================================
// Path
// ============================================================================

/// Parse a path for annotations - no type arguments allowed since annotations can't be parameterized
fn parse_annotation_path(input: &ParseStream) -> Result<Path> {
    let start = input.peek().span;
    let ident = input.parse_ident()?;
    let mut segments = vec![PathSegment { ident, args: None }];

    while input.is(&TokenKind::Dot) {
        let saved = input.cursor();
        input.next();
        if input.is_any_ident() || is_contextual_type_keyword(&input.peek().kind) {
            let ident = input.parse_ident()?;
            segments.push(PathSegment { ident, args: None });
        } else {
            input.set_cursor(saved);
            break;
        }
    }

    let end = if input.cursor() > 0 {
        input.look_ahead(0).span
    } else {
        start
    };

    Ok(Path {
        segments,
        span: start.join(end),
    })
}

fn parse_path(input: &ParseStream) -> Result<Path> {
    let start = input.peek().span;
    let ident = input.parse_ident()?;
    let args = parse_optional_type_arguments(input);
    let mut segments = vec![PathSegment { ident, args }];

    while input.is(&TokenKind::Dot) {
        // Speculative check: only consume the dot if followed by an identifier
        let saved = input.cursor();
        input.next(); // consume '.'
        // Consume any type-use annotations after . (e.g., OuterType.@Annotation InnerType)
        let _annotations = parse_type_annotations(input);
        if input.is_any_ident() || is_contextual_type_keyword(&input.peek().kind) {
            let ident = input.parse_ident()?;
            let args = parse_optional_type_arguments(input);
            segments.push(PathSegment { ident, args });
        } else {
            input.set_cursor(saved);
            break;
        }
    }

    let end = if input.cursor() > 0 {
        input.look_ahead(0).span
    } else {
        start
    };
    Ok(Path {
        segments,
        span: start.join(end),
    })
}

fn is_contextual_type_keyword(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Record | TokenKind::Sealed | TokenKind::Var | TokenKind::Yield | TokenKind::Open
    )
}

// ============================================================================
// Type
// ============================================================================

fn parse_type(input: &ParseStream) -> Result<Type> {
    let annotations = parse_type_annotations(input);
    let start = if let Some(ann) = annotations.first() {
        ann.span()
    } else {
        input.peek().span
    };

    if input.is(&TokenKind::Void) {
        input.next();
        return Ok(Type::Void(input.peek().span));
    }

    let base = if is_primitive_type_token(&input.peek().kind) {
        let prim = parse_primitive_type(input)?;
        Type::Primitive(prim)
    } else if input.is_any_ident() || is_contextual_type_keyword(&input.peek().kind) {
        let path = parse_path(input)?;
        Type::Reference(ReferenceType::ClassOrInterfaceType(ClassOrInterfaceType {
            path,
            annotations_prefix: annotations,
        }))
    } else {
        return Err(crate::error::Error::new(input.peek().span, "expected type"));
    };

    let dims = parse_array_dims(input)?;
    if dims.is_empty() {
        Ok(base)
    } else {
        let span = start.join(dims.last().unwrap().bracket_span.1);
        Ok(Type::Reference(ReferenceType::Array(ArrayType {
            elem_type: Box::new(base),
            dims,
            span,
        })))
    }
}

fn parse_type_annotations(input: &ParseStream) -> Vec<Annotation> {
    let mut anns = Vec::new();
    while input.is(&TokenKind::At) {
        // Stop before @interface
        let saved = input.cursor();
        input.next(); // consume '@'
        if input.is(&TokenKind::Interface) {
            input.set_cursor(saved);
            break;
        }
        input.set_cursor(saved);
        if let Some(ann) = input.try_parse(|i| i.parse::<Annotation>()) {
            anns.push(ann);
        } else {
            break;
        }
    }
    anns
}

fn parse_annotation_type_body(input: &ParseStream) -> Result<AnnotationInterfaceBody> {
    input.expect(TokenKind::LBrace)?;
    let open = input.peek().span;
    let mut members = Vec::new();
    while !input.is(&TokenKind::RBrace) && !input.is_empty() {
        if input.eat(&TokenKind::Semicolon) {
            members.push(AnnotationInterfaceMember::Empty(input.peek().span));
            continue;
        }
        let modifiers = parse_modifiers(input);
        let annotations = parse_annotations(input)?;
        let mut all_mods: Vec<Modifier> = annotations
            .iter()
            .map(|a| Modifier::Annotation(a.clone()))
            .collect();
        all_mods.extend(modifiers);

        if input.is(&TokenKind::At) {
            // Nested @interface
            let saved = input.cursor();
            let at_span = input.peek().span;
            input.next();
            if input.is(&TokenKind::Interface) {
                input.next();
                let name = input.parse_ident()?;
                let body = parse_annotation_type_body(input)?;
                members.push(AnnotationInterfaceMember::AnnotationInterface(
                    AnnotationInterfaceDecl {
                        modifiers: all_mods,
                        at_span,
                        interface_span: name.span(),
                        name,
                        body,
                    },
                ));
                continue;
            }
            input.set_cursor(saved);
        }

        // Try class/interface/enum/record
        if input.is(&TokenKind::Class) {
            let decl = parse_class_decl(input, all_mods)?;
            members.push(AnnotationInterfaceMember::Class(decl));
            continue;
        }
        if input.is(&TokenKind::Interface) {
            let decl = parse_interface_decl(input, all_mods)?;
            members.push(AnnotationInterfaceMember::Interface(decl));
            continue;
        }
        if input.is(&TokenKind::Enum) {
            let decl = parse_enum_decl(input, all_mods)?;
            members.push(AnnotationInterfaceMember::Enum(decl));
            continue;
        }
        if input.is(&TokenKind::Record) {
            let decl: RecordDecl = parse_record_decl(input, all_mods)?;
            members.push(AnnotationInterfaceMember::Record(decl));
            continue;
        }
        // Annotation element or field
        let ty = parse_type(input)?;
        let dims = parse_array_dims(input)?;
        let name = input.parse_ident()?;

        if input.is(&TokenKind::LParen) {
            // Annotation element: Type name() [default value];
            let open_paren = input.peek().span;
            input.next();
            input.expect(TokenKind::RParen)?;
            let close_paren = input.peek().span;
            let default_value = if input.eat(&TokenKind::Default) {
                let eq_span = input.peek().span;
                let val = parse_element_value(input)?;
                Some((eq_span, val))
            } else {
                None
            };
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            members.push(AnnotationInterfaceMember::Element(AnnotationElement {
                modifiers: all_mods,
                ty,
                name,
                paren_span: (open_paren, close_paren),
                dims,
                default_value,
                semi_span,
            }));
        } else {
            // Field declaration: Type name [= value];
            let mut declarators = Vec::new();
            let initializer = if input.eat(&TokenKind::Eq) {
                Some(parse_expression(input)?)
            } else {
                None
            };
            declarators.push(VariableDeclarator {
                name: Some(name),
                dims,
                initializer,
            });
            while input.eat(&TokenKind::Comma) {
                let name = input.parse_ident()?;
                let dims = parse_array_dims(input)?;
                let initializer = if input.eat(&TokenKind::Eq) {
                    Some(parse_expression(input)?)
                } else {
                    None
                };
                declarators.push(VariableDeclarator {
                    name: Some(name),
                    dims,
                    initializer,
                });
            }
            input.expect(TokenKind::Semicolon)?;
            let semi_span = input.peek().span;
            members.push(AnnotationInterfaceMember::Field(FieldDecl {
                modifiers: all_mods,
                ty,
                declarators,
                semi_span,
            }));
        }
    }
    input.expect(TokenKind::RBrace)?;
    let close = input.peek().span;
    Ok(AnnotationInterfaceBody {
        brace_span: (open, close),
        members,
    })
}

fn is_primitive_type_token(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Byte
            | TokenKind::Short
            | TokenKind::Int
            | TokenKind::Long
            | TokenKind::Char
            | TokenKind::Float
            | TokenKind::Double
            | TokenKind::Boolean
    )
}

fn parse_primitive_type(input: &ParseStream) -> Result<PrimitiveType> {
    let prim = match &input.peek().kind {
        TokenKind::Byte => PrimitiveType::Byte,
        TokenKind::Short => PrimitiveType::Short,
        TokenKind::Int => PrimitiveType::Int,
        TokenKind::Long => PrimitiveType::Long,
        TokenKind::Char => PrimitiveType::Char,
        TokenKind::Float => PrimitiveType::Float,
        TokenKind::Double => PrimitiveType::Double,
        TokenKind::Boolean => PrimitiveType::Boolean,
        _ => {
            return Err(crate::error::Error::new(
                input.peek().span,
                "expected primitive type",
            ));
        }
    };
    input.next();
    Ok(prim)
}
fn parse_array_dims(input: &ParseStream) -> Result<Vec<ArrayDim>> {
    let mut dims = Vec::new();
    loop {
        // Check for annotations before [ (JSR 308 type-use annotations on array dimensions)
        // We need to be careful: annotations before [ should only be consumed if [ follows.
        let saved = input.cursor();
        let _pre_annotations = parse_type_annotations(input);
        if !input.is(&TokenKind::LBracket) {
            // No bracket found - restore cursor to before annotations
            input.set_cursor(saved);
            break;
        }
        let open = input.peek().span;
        input.next();
        let annotations = parse_type_annotations(input);
        input.expect(TokenKind::RBracket)?;
        let close = input.peek().span;
        dims.push(ArrayDim {
            bracket_span: (open, close),
            annotations,
        });
    }
    Ok(dims)
}
fn parse_type_list(input: &ParseStream) -> Vec<Type> {
    let mut types = Vec::new();
    while let Ok(ty) = parse_type(input) {
        types.push(ty);
        if !input.eat(&TokenKind::Comma) {
            break;
        }
    }
    types
}
// Generics
// ============================================================================

fn parse_optional_type_params(input: &ParseStream) -> Option<TypeParameters> {
    if input.is(&TokenKind::Lt) {
        parse_type_params(input).ok()
    } else {
        None
    }
}

fn parse_type_params(input: &ParseStream) -> Result<TypeParameters> {
    let lt_span = input.peek().span;
    input.expect(TokenKind::Lt)?;
    let mut gt_spans = Vec::new();
    let mut params = Vec::new();
    loop {
        let annotations = parse_annotations(input)?;
        let name = input.parse_ident()?;
        let bound = if input.eat(&TokenKind::Extends) {
            let extends_span = input.peek().span;
            // Consume any type-use annotations before the bound type
            let _annotations = parse_type_annotations(input);
            let first = parse_path(input)?;
            let mut additional = Vec::new();
            while input.eat(&TokenKind::Amp) {
                let _annotations = parse_type_annotations(input);
                additional.push(parse_path(input)?);
            }
            Some(TypeBound {
                first,
                additional,
                extends_span,
            })
        } else {
            None
        };
        let span = annotations
            .first()
            .map(|a| a.span())
            .unwrap_or(name.span())
            .join(
                bound
                    .as_ref()
                    .map(|b| {
                        b.additional
                            .last()
                            .map(|p| p.span)
                            .unwrap_or_else(|| b.first.span)
                    })
                    .unwrap_or(name.span()),
            );
        params.push(TypeParameter {
            annotations,
            name,
            bound,
            span,
        });
        if input.eat(&TokenKind::Gt) {
            gt_spans.push(input.peek().span);
            break;
        }
        if input.is(&TokenKind::GtGt) {
            // Split >> into one > for us and one > for the outer parser
            input.split_gt();
            gt_spans.push(input.peek().span);
            break;
        }
        if input.is(&TokenKind::GtGtGt) {
            // Split >>> into one > for us and two > for the outer parser
            input.split_gt();
            gt_spans.push(input.peek().span);
            break;
        }
        input.expect(TokenKind::Comma)?;
    }
    Ok(TypeParameters {
        params,
        lt_span,
        gt_spans,
    })
}

fn parse_optional_type_arguments(input: &ParseStream) -> Option<TypeArguments> {
    if input.is(&TokenKind::Lt) {
        parse_type_arguments(input).ok()
    } else {
        None
    }
}

fn parse_type_arguments(input: &ParseStream) -> Result<TypeArguments> {
    let lt_span = input.peek().span;
    input.expect(TokenKind::Lt)?;
    // Handle diamond operator <>: empty type argument list
    if input.eat(&TokenKind::Gt) {
        let gt_spans = vec![input.peek().span];
        return Ok(TypeArguments {
            args: vec![],
            lt_token: lt_span,
            gt_tokens: gt_spans,
        });
    }
    let mut gt_spans = Vec::new();
    let mut args = Vec::new();
    loop {
        if input.eat(&TokenKind::Question) {
            let wildcard_span = input.peek().span;
            let bound = if input.eat(&TokenKind::Extends) {
                Some(WildcardBound::Extends(Box::new(parse_type(input)?)))
            } else if input.eat(&TokenKind::Super) {
                Some(WildcardBound::Super(Box::new(parse_type(input)?)))
            } else {
                None
            };
            args.push(TypeArgument::Wildcard(Wildcard {
                bound,
                span: wildcard_span,
            }));
        } else {
            args.push(TypeArgument::Type(Box::new(parse_type(input)?)));
        }
        if input.eat(&TokenKind::Gt) {
            gt_spans.push(input.peek().span);
            break;
        }
        if input.is(&TokenKind::GtGt) {
            // Split >> into one > for us and one > for the outer parser
            input.split_gt();
            gt_spans.push(input.peek().span);
            break;
        }
        if input.is(&TokenKind::GtGtGt) {
            // Split >>> into one > for us and two > for the outer parser
            input.split_gt();
            gt_spans.push(input.peek().span);
            break;
        }
        input.expect(TokenKind::Comma)?;
    }
    Ok(TypeArguments {
        args,
        lt_token: lt_span,
        gt_tokens: gt_spans,
    })
}

// ============================================================================
// Statements
// ============================================================================

fn parse_block(input: &ParseStream) -> Result<Block> {
    let open = input.peek().span;
    input.expect(TokenKind::LBrace)?;
    let mut stmts = Vec::new();
    while !input.is(&TokenKind::RBrace) && !input.is_empty() {
        stmts.push(parse_statement(input)?);
    }
    input.expect(TokenKind::RBrace)?;
    let close = input.peek().span;
    Ok(Block {
        brace_span: (open, close),
        stmts,
    })
}

fn parse_statement(input: &ParseStream) -> Result<Stmt> {
    if input.is(&TokenKind::LBrace) {
        let block = parse_block(input)?;
        return Ok(Stmt::Block(block));
    }
    if input.is(&TokenKind::Semicolon) {
        let sp = input.next().span;
        return Ok(Stmt::Empty(sp));
    }
    if input.is(&TokenKind::If) {
        return parse_if_stmt(input);
    }
    if input.is(&TokenKind::Assert) {
        return parse_assert_stmt(input);
    }
    if input.is(&TokenKind::Switch) {
        return parse_switch_stmt(input);
    }
    if input.is(&TokenKind::While) {
        return parse_while_stmt(input);
    }
    if input.is(&TokenKind::Do) {
        return parse_do_while_stmt(input);
    }
    if input.is(&TokenKind::For) {
        return parse_for_stmt(input);
    }
    if input.is(&TokenKind::Break) {
        return parse_jump_stmt(input, TokenKind::Break, Stmt::Break);
    }
    if input.is(&TokenKind::Continue) {
        return parse_jump_stmt(input, TokenKind::Continue, Stmt::Continue);
    }
    if input.is(&TokenKind::Return) {
        return parse_return_stmt(input);
    }
    if input.is(&TokenKind::Throw) {
        return parse_throw_stmt(input);
    }
    if input.is(&TokenKind::Synchronized) {
        return parse_synchronized_stmt(input);
    }
    if input.is(&TokenKind::Try) {
        return parse_try_stmt(input);
    }
    if input.is(&TokenKind::Yield) {
        return parse_yield_stmt(input);
    }

    // Check for labeled statement: Ident ':'
    if input.is_any_ident() {
        let saved = input.cursor();
        let ident = input.parse_ident().ok();
        if let Some(ident) = ident {
            if input.is(&TokenKind::Colon) {
                let colon_span = input.peek().span;
                input.next();
                let stmt = parse_statement(input)?;
                return Ok(Stmt::Labeled(LabeledStmt {
                    label: ident,
                    colon_span,
                    stmt: Box::new(stmt),
                }));
            }
            input.set_cursor(saved);
        } else {
            input.set_cursor(saved);
        }
    }

    // Check for local class/interface/enum/record declaration
    // Also check for modifiers/annotations that may precede the type keyword
    if input.is(&TokenKind::Class)
        || input.is(&TokenKind::Interface)
        || input.is(&TokenKind::Enum)
        || input.is(&TokenKind::Record)
        || input.is(&TokenKind::At)
        || is_modifier_keyword(&input.peek().kind)
    {
        let saved = input.cursor();
        match input.try_parse(|i| {
            // Only parse as local class if modifiers + keyword + ident pattern
            // e.g. "class Foo" or "abstract class Foo" but not "record.method()"
            let _modifiers = parse_modifiers(i);
            if !matches!(
                i.peek().kind,
                TokenKind::Class | TokenKind::Interface | TokenKind::Enum | TokenKind::Record
            ) {
                return Err(crate::error::Error::new(i.peek().span, "not a local class"));
            }
            TypeDecl::parse(i)
        }) {
            Some(decl) => return Ok(Stmt::ClassDecl(Box::new(decl))),
            None => {
                input.set_cursor(saved);
            }
        }
    }

    // Check for local variable declaration or expression statement
    // Local var decl starts with type, possibly with var
    let saved = input.cursor();
    if let Some(stmt) = try_parse_local_var_decl_or_expr_stmt(input) {
        return Ok(stmt);
    }
    input.set_cursor(saved);

    // Expression statement
    let expr = parse_expression(input)?;
    input.expect(TokenKind::Semicolon)?;
    let semi_span = input.peek().span;
    Ok(Stmt::Expr(ExprStmt { expr, semi_span }))
}

fn try_parse_local_var_decl_or_expr_stmt(input: &ParseStream) -> Option<Stmt> {
    let modifiers = parse_modifiers(input);
    let is_var = input.is(&TokenKind::Var);
    let can_start_type = crate::token::can_start_type(&input.peek().kind);

    if is_var || can_start_type {
        let saved = input.cursor();
        let ty_result = if is_var {
            let var_span = input.peek().span;
            input.next();
            Ok(LocalVarType::Var(var_span))
        } else {
            parse_type(input).map(LocalVarType::Type)
        };

        if let Ok(ty) = ty_result {
            // Now check if next token is an identifier (local var decl) or something else
            if input.is_any_ident() {
                let name = input.parse_ident().ok()?;
                let dims = parse_array_dims(input).ok().unwrap_or_default();
                let initializer = if input.eat(&TokenKind::Eq) {
                    parse_expression(input).ok()
                } else {
                    None
                };
                let mut declarators = vec![VariableDeclarator {
                    name: Some(name),
                    dims,
                    initializer,
                }];
                while input.eat(&TokenKind::Comma) {
                    let name = input.parse_ident().ok()?;
                    let dims = parse_array_dims(input).ok().unwrap_or_default();
                    let init = if input.eat(&TokenKind::Eq) {
                        parse_expression(input).ok()
                    } else {
                        None
                    };
                    declarators.push(VariableDeclarator {
                        name: Some(name),
                        dims,
                        initializer: init,
                    });
                }
                if input.eat(&TokenKind::Semicolon) {
                    let semi_span = input.peek().span;
                    return Some(Stmt::LocalVarDecl(LocalVarDeclStmt {
                        modifiers,
                        ty,
                        declarators,
                        semi_span,
                    }));
                }
            }
        }
        input.set_cursor(saved);
    }
    None
}

fn parse_if_stmt(input: &ParseStream) -> Result<Stmt> {
    let if_span = input.peek().span;
    input.expect(TokenKind::If)?;
    input.expect(TokenKind::LParen)?;
    let open = input.peek().span;
    let cond = parse_expression(input)?;
    input.expect(TokenKind::RParen)?;
    let close = input.peek().span;
    let then_stmt = parse_statement(input)?;
    let else_clause = if input.eat(&TokenKind::Else) {
        let else_span = input.peek().span;
        let else_stmt = parse_statement(input)?;
        Some((else_span, Box::new(else_stmt)))
    } else {
        None
    };
    Ok(Stmt::If(IfStmt {
        if_span,
        paren_span: (open, close),
        cond,
        then_stmt: Box::new(then_stmt),
        else_clause,
    }))
}

fn parse_assert_stmt(input: &ParseStream) -> Result<Stmt> {
    let assert_span = input.peek().span;
    input.expect(TokenKind::Assert)?;
    let cond = parse_expression(input)?;
    let detail = if input.eat(&TokenKind::Colon) {
        let colon_span = input.peek().span;
        let detail = parse_expression(input)?;
        Some((colon_span, detail))
    } else {
        None
    };
    input.expect(TokenKind::Semicolon)?;
    let semi_span = input.peek().span;
    Ok(Stmt::Assert(AssertStmt {
        assert_span,
        cond,
        detail,
        semi_span,
    }))
}

fn parse_switch_stmt(input: &ParseStream) -> Result<Stmt> {
    let switch_span = input.peek().span;
    input.expect(TokenKind::Switch)?;
    input.expect(TokenKind::LParen)?;
    let open = input.peek().span;
    let selector = parse_expression(input)?;
    input.expect(TokenKind::RParen)?;
    let close = input.peek().span;
    input.expect(TokenKind::LBrace)?;
    let brace_open = input.peek().span;
    let mut cases = Vec::new();
    while !input.is(&TokenKind::RBrace) && !input.is_empty() {
        let labels = parse_switch_labels(input)?;
        let colon_span = if input.eat(&TokenKind::Colon) || input.eat(&TokenKind::Arrow) {
            input.peek().span
        } else {
            break;
        };
        let mut stmts = Vec::new();
        while !is_switch_label_start(input) && !input.is(&TokenKind::RBrace) && !input.is_empty() {
            stmts.push(parse_statement(input)?);
        }
        cases.push(SwitchCaseGroup {
            labels,
            colon_span,
            stmts,
        });
    }
    input.expect(TokenKind::RBrace)?;
    let brace_close = input.peek().span;
    Ok(Stmt::Switch(SwitchStmt {
        switch_span,
        paren_span: (open, close),
        selector,
        brace_span: (brace_open, brace_close),
        cases,
    }))
}

fn is_any_ident_kind(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Ident(_)
            | TokenKind::Record
            | TokenKind::Sealed
            | TokenKind::Var
            | TokenKind::Yield
            | TokenKind::Open
            | TokenKind::Provides
            | TokenKind::Requires
            | TokenKind::Uses
            | TokenKind::With
            | TokenKind::When
            | TokenKind::To
            | TokenKind::Exports
            | TokenKind::Opens
            | TokenKind::Transitive
            | TokenKind::Permits
            | TokenKind::NonSealed
            | TokenKind::Module
            | TokenKind::Byte
            | TokenKind::Short
            | TokenKind::Int
            | TokenKind::Long
            | TokenKind::Char
            | TokenKind::Float
            | TokenKind::Double
            | TokenKind::Boolean
            | TokenKind::Void
    )
}

fn is_switch_label_start(input: &ParseStream) -> bool {
    input.is(&TokenKind::Case) || input.is(&TokenKind::Default)
}

/// Parse a case value expression, but don't allow `IDENTIFIER ->` to be parsed as a lambda.
fn parse_case_value(input: &ParseStream) -> Result<Expr> {
    // If we see IDENTIFIER followed by `->`, it's a case label arrow, not a lambda.
    // Check for this pattern before calling the full expression parser.
    if input.is_any_ident() && input.look_ahead(1).kind == TokenKind::Arrow {
        let ident = input.parse_ident()?;
        return Ok(Expr::Ident(ident));
    }
    // If we see `(` followed by a type and `)` then IDENTIFIER `->`, it's a cast case value
    // like `case (int) FOO ->`. The expression parser would treat `FOO ->` as a lambda
    // inside the cast, which is wrong — `->` here is the case arrow.
    // Detect this pattern by scanning ahead.
    if input.is(&TokenKind::LParen) {
        let mut depth = 1usize;
        let mut offset = 1usize;
        loop {
            let tok = input.look_ahead(offset);
            match &tok.kind {
                TokenKind::LParen => depth += 1,
                TokenKind::RParen => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                TokenKind::Eof => break,
                _ => {}
            }
            offset += 1;
        }
        // offset now points to matching `)`. Check if `IDENTIFIER ->` follows.
        let after_rparen = &input.look_ahead(offset + 1).kind;
        let after_that = &input.look_ahead(offset + 2).kind;
        if is_any_ident_kind(after_rparen) && matches!(after_that, TokenKind::Arrow) {
            // Parse as cast: (type) IDENTIFIER, leaving `->` for the switch parser.
            let open_span = input.peek().span;
            input.next(); // consume `(`
            let saved = input.cursor();
            if let Ok(ty) = parse_type(input)
                && input.eat(&TokenKind::RParen)
            {
                let close_span = input.peek().span;
                let ident = input.parse_ident()?;
                return Ok(Expr::Cast(CastExpr {
                    paren_span: (open_span, close_span),
                    target_type: ty,
                    expr: Box::new(Expr::Ident(ident)),
                }));
            }
            input.set_cursor(saved);
            // If the targeted parse failed, fall through to general expression parsing
        }
    }
    parse_expression(input)
}

fn parse_switch_labels(input: &ParseStream) -> Result<Vec<SwitchCase>> {
    let mut labels = Vec::new();
    loop {
        if input.is(&TokenKind::Case) {
            let case_span = input.peek().span;
            input.next();
            if input.is(&TokenKind::NullLit) {
                let null_span = input.peek().span;
                input.next();
                if input.eat(&TokenKind::Comma) {
                    // case null, default ->
                    if input.is(&TokenKind::Default) {
                        let default_span = input.peek().span;
                        input.next();
                        labels.push(SwitchCase::CaseNullDefault {
                            case_span,
                            null_span,
                            default_span,
                        });
                        break;
                    }
                } else {
                    labels.push(SwitchCase::CaseNull {
                        case_span,
                        null_span,
                    });
                    break;
                }
            } else if input.is(&TokenKind::Default) {
                let default_span = input.peek().span;
                input.next();
                labels.push(SwitchCase::Default { default_span });
                break;
            } else {
                let mut values = Vec::new();
                values.push(parse_case_value(input)?);
                while input.eat(&TokenKind::Comma) {
                    if input.is(&TokenKind::Default) {
                        let default_span = input.peek().span;
                        input.next();
                        // Add remaining values as a case
                        labels.push(SwitchCase::CaseValues {
                            case_span,
                            values: std::mem::take(&mut values),
                        });
                        labels.push(SwitchCase::Default { default_span });
                        break;
                    }
                    values.push(parse_case_value(input)?);
                }
                if !values.is_empty() {
                    labels.push(SwitchCase::CaseValues { case_span, values });
                }
                break;
            }
        } else if input.is(&TokenKind::Default) {
            let default_span = input.peek().span;
            input.next();
            labels.push(SwitchCase::Default { default_span });
            break;
        } else {
            break;
        }
    }
    Ok(labels)
}

fn parse_while_stmt(input: &ParseStream) -> Result<Stmt> {
    let while_span = input.peek().span;
    input.expect(TokenKind::While)?;
    input.expect(TokenKind::LParen)?;
    let open = input.peek().span;
    let cond = parse_expression(input)?;
    input.expect(TokenKind::RParen)?;
    let close = input.peek().span;
    let body = parse_statement(input)?;
    Ok(Stmt::While(WhileStmt {
        while_span,
        paren_span: (open, close),
        cond,
        body: Box::new(body),
    }))
}

fn parse_do_while_stmt(input: &ParseStream) -> Result<Stmt> {
    let do_span = input.peek().span;
    input.expect(TokenKind::Do)?;
    let body = parse_statement(input)?;
    let while_span = input.peek().span;
    input.expect(TokenKind::While)?;
    input.expect(TokenKind::LParen)?;
    let open = input.peek().span;
    let cond = parse_expression(input)?;
    input.expect(TokenKind::RParen)?;
    let close = input.peek().span;
    input.expect(TokenKind::Semicolon)?;
    let semi_span = input.peek().span;
    Ok(Stmt::DoWhile(DoWhileStmt {
        do_span,
        body: Box::new(body),
        while_span,
        paren_span: (open, close),
        cond,
        semi_span,
    }))
}

fn parse_for_stmt(input: &ParseStream) -> Result<Stmt> {
    let for_span = input.peek().span;
    input.expect(TokenKind::For)?;
    input.expect(TokenKind::LParen)?;
    let open = input.peek().span;

    // Check for enhanced for: Type ident : expr
    // Or regular for: init; cond; update
    let saved = input.cursor();

    // Try enhanced for
    if let Some(stmt) = try_parse_enhanced_for(for_span, open, input) {
        return Ok(stmt);
    }
    input.set_cursor(saved);

    // Regular for
    let init = if input.is(&TokenKind::Semicolon) {
        input.next();
        ForInit::Exprs(Vec::new())
    } else {
        // Could be local var decl or expression list
        let inner_saved = input.cursor();
        if let Some(var_decl) = try_parse_local_var_decl(input) {
            ForInit::LocalVarDecl(var_decl)
        } else {
            input.set_cursor(inner_saved);
            let mut exprs = Vec::new();
            exprs.push(parse_expression(input)?);
            while input.eat(&TokenKind::Comma) {
                exprs.push(parse_expression(input)?);
            }
            input.expect(TokenKind::Semicolon)?;
            ForInit::Exprs(exprs)
        }
    };

    let cond = if input.is(&TokenKind::Semicolon) {
        input.next();
        None
    } else {
        let semi1 = Some(input.peek().span);
        let cond = parse_expression(input)?;
        input.expect(TokenKind::Semicolon)?;
        let _ = semi1;
        Some(cond)
    };

    let mut update = Vec::new();
    while !input.is(&TokenKind::RParen) && !input.is_empty() {
        update.push(parse_expression(input)?);
        input.eat(&TokenKind::Comma);
    }
    input.expect(TokenKind::RParen)?;
    let close = input.peek().span;
    let body = parse_statement(input)?;

    Ok(Stmt::For(ForStmt {
        for_span,
        paren_span: (open, close),
        init,
        cond,
        semi2_span: None,
        update,
        body: Box::new(body),
    }))
}

fn try_parse_local_var_decl(input: &ParseStream) -> Option<LocalVarDeclStmt> {
    let modifiers = parse_modifiers(input);
    let is_var = input.is(&TokenKind::Var);
    let can_start = crate::token::can_start_type(&input.peek().kind);

    if is_var || can_start {
        let saved = input.cursor();
        let ty_result = if is_var {
            let var_span = input.peek().span;
            input.next();
            Ok(LocalVarType::Var(var_span))
        } else {
            parse_type(input).map(LocalVarType::Type)
        };

        if let Ok(ty) = ty_result
            && input.is_any_ident()
        {
            let name = input.parse_ident().ok()?;
            let dims = parse_array_dims(input).ok().unwrap_or_default();
            let initializer = if input.eat(&TokenKind::Eq) {
                parse_expression(input).ok()
            } else {
                None
            };
            let mut declarators = vec![VariableDeclarator {
                name: Some(name),
                dims,
                initializer,
            }];
            while input.eat(&TokenKind::Comma) {
                let name = input.parse_ident().ok()?;
                let dims = parse_array_dims(input).ok().unwrap_or_default();
                let init = if input.eat(&TokenKind::Eq) {
                    parse_expression(input).ok()
                } else {
                    None
                };
                declarators.push(VariableDeclarator {
                    name: Some(name),
                    dims,
                    initializer: init,
                });
            }
            if input.eat(&TokenKind::Semicolon) {
                let semi_span = input.peek().span;
                return Some(LocalVarDeclStmt {
                    modifiers,
                    ty,
                    declarators,
                    semi_span,
                });
            }
        }
        input.set_cursor(saved);
    }
    None
}

fn try_parse_enhanced_for(for_span: Span, open: Span, input: &ParseStream) -> Option<Stmt> {
    let modifiers = parse_modifiers(input);
    let is_var = input.is(&TokenKind::Var);
    let can_start = crate::token::can_start_type(&input.peek().kind);

    if !is_var && !can_start {
        input.set_cursor(input.cursor()); // already advanced by parse_modifiers
        return None;
    }

    let saved = input.cursor();
    let ty_result = if is_var {
        let var_span = input.peek().span;
        input.next();
        Ok(LocalVarType::Var(var_span))
    } else {
        parse_type(input).map(LocalVarType::Type)
    };

    if let Ok(ty) = ty_result
        && input.is_any_ident()
    {
        let name = input.parse_ident().ok()?;
        let dims = parse_array_dims(input).ok().unwrap_or_default();
        if input.eat(&TokenKind::Colon) {
            let colon_span = input.peek().span;
            let iterable = parse_expression(input).ok()?;
            input.expect(TokenKind::RParen).ok()?;
            let close = input.peek().span;
            let body = parse_statement(input).ok()?;
            return Some(Stmt::EnhancedFor(EnhancedForStmt {
                for_span,
                paren_span: (open, close),
                var_decl: LocalVarDeclStmt {
                    modifiers,
                    ty,
                    declarators: vec![VariableDeclarator {
                        name: Some(name),
                        dims,
                        initializer: None,
                    }],
                    semi_span: Span::call_site(),
                },
                colon_span,
                iterable,
                body: Box::new(body),
            }));
        }
    }
    input.set_cursor(saved);
    None
}

fn parse_jump_stmt(
    input: &ParseStream,
    keyword: TokenKind,
    make_stmt: fn(JumpStmt) -> Stmt,
) -> Result<Stmt> {
    let keyword_span = input.peek().span;
    input.expect(keyword)?;
    let label = if input.is_any_ident() && !input.is(&TokenKind::Semicolon) {
        let saved = input.cursor();
        let ident = input.parse_ident().ok();
        if ident.is_some() && (input.is(&TokenKind::Semicolon) || input.is_empty()) {
            ident
        } else {
            input.set_cursor(saved);
            None
        }
    } else {
        None
    };
    input.expect(TokenKind::Semicolon)?;
    let semi_span = input.peek().span;
    Ok(make_stmt(JumpStmt {
        keyword_span,
        label,
        semi_span,
    }))
}

fn parse_return_stmt(input: &ParseStream) -> Result<Stmt> {
    let return_span = input.peek().span;
    input.expect(TokenKind::Return)?;
    let value = if input.is(&TokenKind::Semicolon) {
        None
    } else {
        Some(parse_expression(input)?)
    };
    input.expect(TokenKind::Semicolon)?;
    let semi_span = input.peek().span;
    Ok(Stmt::Return(ReturnStmt {
        return_span,
        value,
        semi_span,
    }))
}

fn parse_throw_stmt(input: &ParseStream) -> Result<Stmt> {
    let throw_span = input.peek().span;
    input.expect(TokenKind::Throw)?;
    let expr = parse_expression(input)?;
    input.expect(TokenKind::Semicolon)?;
    let semi_span = input.peek().span;
    Ok(Stmt::Throw(ThrowStmt {
        throw_span,
        expr,
        semi_span,
    }))
}

fn parse_synchronized_stmt(input: &ParseStream) -> Result<Stmt> {
    let synchronized_span = input.peek().span;
    input.expect(TokenKind::Synchronized)?;
    input.expect(TokenKind::LParen)?;
    let open = input.peek().span;
    let lock = parse_expression(input)?;
    input.expect(TokenKind::RParen)?;
    let close = input.peek().span;
    let body = parse_block(input)?;
    Ok(Stmt::Synchronized(SynchronizedStmt {
        synchronized_span,
        paren_span: (open, close),
        lock,
        body,
    }))
}

fn parse_try_stmt(input: &ParseStream) -> Result<Stmt> {
    let try_span = input.peek().span;
    input.expect(TokenKind::Try)?;

    if input.is(&TokenKind::LParen) {
        // Try-with-resources
        input.expect(TokenKind::LParen)?;
        let open = input.peek().span;
        let mut resources = Vec::new();
        loop {
            let saved = input.cursor();
            // Try parsing as a local var decl (without requiring semicolon)
            let modifiers = parse_modifiers(input);
            let is_var = input.is(&TokenKind::Var);
            let can_start = crate::token::can_start_type(&input.peek().kind);
            if is_var || can_start {
                let ty_result = if is_var {
                    let var_span = input.peek().span;
                    input.next();
                    Ok(LocalVarType::Var(var_span))
                } else {
                    parse_type(input).map(LocalVarType::Type)
                };
                if let Ok(ty) = ty_result
                    && input.is_any_ident()
                    && let Ok(name) = input.parse_ident()
                {
                    let dims = parse_array_dims(input).unwrap_or_default();
                    let initializer = if input.eat(&TokenKind::Eq) {
                        parse_expression(input).ok()
                    } else {
                        None
                    };
                    let declarators = vec![VariableDeclarator {
                        name: Some(name),
                        dims,
                        initializer,
                    }];
                    resources.push(TryResource::Decl(LocalVarDeclStmt {
                        modifiers,
                        ty,
                        declarators,
                        semi_span: Span::call_site(),
                    }));
                    if !input.eat(&TokenKind::Semicolon) {
                        break;
                    }
                    if input.is(&TokenKind::RParen) {
                        break;
                    }
                    continue;
                }
            }
            input.set_cursor(saved);
            let ident = input.parse_ident()?;
            resources.push(TryResource::VarRef(ident));
            if !input.eat(&TokenKind::Semicolon) {
                break;
            }
            if input.is(&TokenKind::RParen) {
                break;
            }
        }
        input.expect(TokenKind::RParen)?;
        let close = input.peek().span;
        let block = parse_block(input)?;
        let catches = parse_catch_clauses(input)?;
        let finally_block = parse_finally_clause(input);
        Ok(Stmt::Try(TryStmt::TryWithResources {
            try_span,
            paren_span: (open, close),
            resources,
            block,
            catches,
            finally_block,
        }))
    } else {
        let block = parse_block(input)?;
        let catches = parse_catch_clauses(input)?;
        let finally_block = parse_finally_clause(input);
        Ok(Stmt::Try(TryStmt::Basic {
            try_span,
            block,
            catches,
            finally_block,
        }))
    }
}

fn parse_catch_clauses(input: &ParseStream) -> Result<Vec<CatchClause>> {
    let mut clauses = Vec::new();
    while input.is(&TokenKind::Catch) {
        let catch_span = input.peek().span;
        input.next();
        input.expect(TokenKind::LParen).ok();
        let open = input.peek().span;
        let modifiers = parse_modifiers(input);
        let mut types = Vec::new();
        types.push(parse_type(input)?);
        while input.eat(&TokenKind::Pipe) {
            types.push(parse_type(input)?);
        }
        let name = input.parse_ident()?;
        input.expect(TokenKind::RParen).ok();
        let close = input.peek().span;
        let block = parse_block(input)?;
        clauses.push(CatchClause {
            catch_span,
            paren_span: (open, close),
            param: CatchParam {
                modifiers,
                ty: CatchType { types },
                name,
            },
            block,
        });
    }
    Ok(clauses)
}

fn parse_finally_clause(input: &ParseStream) -> Option<(Span, Block)> {
    if input.eat(&TokenKind::Finally) {
        let finally_span = input.peek().span;
        let block = parse_block(input).ok()?;
        Some((finally_span, block))
    } else {
        None
    }
}

fn parse_yield_stmt(input: &ParseStream) -> Result<Stmt> {
    let yield_span = input.peek().span;
    input.expect(TokenKind::Yield)?;
    let value = parse_expression(input)?;
    input.expect(TokenKind::Semicolon)?;
    let semi_span = input.peek().span;
    Ok(Stmt::Yield(YieldStmt {
        yield_span,
        value,
        semi_span,
    }))
}

// ============================================================================
// Expressions (Pratt parser)
// ============================================================================

fn parse_expression(input: &ParseStream) -> Result<Expr> {
    parse_assignment_expr(input)
}

fn parse_assignment_expr(input: &ParseStream) -> Result<Expr> {
    let left = parse_conditional_expr(input)?;

    if is_assignment_op(&input.peek().kind) {
        let op = parse_assign_op(input)?;
        let value = parse_assignment_expr(input)?;
        let target = expr_to_assign_target(left)?;
        return Ok(Expr::Assign(AssignExpr {
            target,
            op,
            value: Box::new(value),
        }));
    }

    Ok(left)
}

fn is_assignment_op(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Eq
            | TokenKind::PlusEq
            | TokenKind::MinusEq
            | TokenKind::StarEq
            | TokenKind::SlashEq
            | TokenKind::AmpEq
            | TokenKind::PipeEq
            | TokenKind::CaretEq
            | TokenKind::PercentEq
            | TokenKind::LtLtEq
            | TokenKind::GtGtEq
            | TokenKind::GtGtGtEq
    )
}

fn parse_assign_op(input: &ParseStream) -> Result<AssignOpToken> {
    let span = input.peek().span;
    let op = match &input.peek().kind {
        TokenKind::Eq => AssignOp::Assign,
        TokenKind::PlusEq => AssignOp::AddAssign,
        TokenKind::MinusEq => AssignOp::SubAssign,
        TokenKind::StarEq => AssignOp::MulAssign,
        TokenKind::SlashEq => AssignOp::DivAssign,
        TokenKind::AmpEq => AssignOp::AndAssign,
        TokenKind::PipeEq => AssignOp::OrAssign,
        TokenKind::CaretEq => AssignOp::XorAssign,
        TokenKind::PercentEq => AssignOp::RemAssign,
        TokenKind::LtLtEq => AssignOp::LShiftAssign,
        TokenKind::GtGtEq => AssignOp::RShiftAssign,
        TokenKind::GtGtGtEq => AssignOp::URShiftAssign,
        _ => {
            return Err(crate::error::Error::new(
                span,
                "expected assignment operator",
            ));
        }
    };
    input.next();
    Ok(AssignOpToken { op, span })
}

fn expr_to_assign_target(expr: Expr) -> Result<AssignTarget> {
    match expr {
        Expr::Ident(i) => Ok(AssignTarget::Ident(i)),
        Expr::FieldAccess(f) => Ok(AssignTarget::FieldAccess(f)),
        Expr::ArrayAccess(a) => Ok(AssignTarget::ArrayAccess(a)),
        _ => Err(crate::error::Error::new(
            expr.span(),
            "invalid assignment target",
        )),
    }
}

fn parse_conditional_expr(input: &ParseStream) -> Result<Expr> {
    let expr = parse_binary_expr(input, 0)?;
    if input.eat(&TokenKind::Question) {
        let question_span = input.peek().span;
        let then_expr = parse_expression(input)?;
        input.expect(TokenKind::Colon)?;
        let colon_span = input.peek().span;
        let else_expr = parse_conditional_expr(input)?;
        return Ok(Expr::Conditional(ConditionalExpr {
            cond: Box::new(expr),
            question_span,
            then_expr: Box::new(then_expr),
            colon_span,
            else_expr: Box::new(else_expr),
        }));
    }
    Ok(expr)
}

fn get_precedence(kind: &TokenKind) -> u8 {
    match kind {
        TokenKind::PipePipe => 1,
        TokenKind::AmpAmp => 2,
        TokenKind::Pipe => 3,
        TokenKind::Caret => 4,
        TokenKind::Amp => 5,
        TokenKind::EqEq | TokenKind::BangEq => 6,
        TokenKind::Lt
        | TokenKind::Gt
        | TokenKind::LtEq
        | TokenKind::GtEq
        | TokenKind::Instanceof => 7,
        TokenKind::LtLt | TokenKind::GtGt | TokenKind::GtGtGt => 8,
        TokenKind::Plus | TokenKind::Minus => 9,
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => 10,
        _ => 0,
    }
}

fn token_to_bin_op(kind: &TokenKind) -> Option<crate::ast::op::BinOp> {
    use crate::ast::op::BinOp::*;
    Some(match kind {
        TokenKind::Plus => Add,
        TokenKind::Minus => Sub,
        TokenKind::Star => Mul,
        TokenKind::Slash => Div,
        TokenKind::Percent => Rem,
        TokenKind::Amp => And,
        TokenKind::Pipe => Or,
        TokenKind::Caret => Xor,
        TokenKind::LtLt => LShift,
        TokenKind::GtGt => RShift,
        TokenKind::GtGtGt => URShift,
        TokenKind::EqEq => Eq,
        TokenKind::BangEq => Ne,
        TokenKind::Lt => Lt,
        TokenKind::Gt => Gt,
        TokenKind::LtEq => Le,
        TokenKind::GtEq => Ge,
        TokenKind::AmpAmp => LAnd,
        TokenKind::PipePipe => LOr,
        _ => return None,
    })
}

fn parse_binary_expr(input: &ParseStream, min_prec: u8) -> Result<Expr> {
    let mut left = parse_unary_expr(input)?;

    loop {
        let prec = get_precedence(&input.peek().kind);
        if prec == 0 || prec < min_prec {
            break;
        }

        if input.is(&TokenKind::Instanceof) {
            let instanceof_span = input.peek().span;
            input.next();

            // Handle 'final' modifier in patterns (Java 16+)
            let _has_final = input.eat(&TokenKind::Final);

            let ty = parse_type(input)?;
            let ty_span = ty.span();

            if input.is(&TokenKind::LParen) {
                // Record pattern: instanceof Point(int x, int y)
                input.next(); // consume '('
                let mut components = Vec::new();
                while !input.is(&TokenKind::RParen) && !input.is_empty() {
                    components.push(parse_pattern(input)?);
                    input.eat(&TokenKind::Comma);
                }
                input.expect(TokenKind::RParen)?;
                let end = input.peek().span;
                left = Expr::Instanceof(InstanceofExpr {
                    expr: Box::new(left),
                    instanceof_span,
                    pattern: InstanceofPattern::Pattern(Pattern::RecordPattern(RecordPattern {
                        record_type: ty,
                        components,
                        span: ty_span.join(end),
                    })),
                });
                continue;
            } else if input.is_any_ident() {
                // Type pattern: instanceof String s
                let name = input.parse_ident()?;
                left = Expr::Instanceof(InstanceofExpr {
                    expr: Box::new(left),
                    instanceof_span,
                    pattern: InstanceofPattern::Pattern(Pattern::TypePattern(TypePattern {
                        annotations: vec![],
                        ty,
                        name: Some(name),
                        span: ty_span.join(input.peek().span),
                    })),
                });
                continue;
            } else {
                left = Expr::Instanceof(InstanceofExpr {
                    expr: Box::new(left),
                    instanceof_span,
                    pattern: InstanceofPattern::Type(ty),
                });
                continue;
            }
        }

        let op = match token_to_bin_op(&input.peek().kind) {
            Some(op) => op,
            None => break,
        };
        let op_span = input.peek().span;
        input.next();

        let right = parse_binary_expr(input, prec + 1)?;
        left = Expr::Binary(BinaryExpr {
            left: Box::new(left),
            op: BinOpToken { op, span: op_span },
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn can_start_unary_expr(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Bang
            | TokenKind::Tilde
            | TokenKind::PlusPlus
            | TokenKind::MinusMinus
            | TokenKind::LParen
            | TokenKind::New
            | TokenKind::Super
            | TokenKind::This
            | TokenKind::IntegerLit(_)
            | TokenKind::FloatLit(_)
            | TokenKind::StringLit(_)
            | TokenKind::CharLit(_)
            | TokenKind::BoolLit(_)
            | TokenKind::NullLit
            | TokenKind::Ident(_)
            | TokenKind::Record
            | TokenKind::Sealed
            | TokenKind::Var
            | TokenKind::Yield
            | TokenKind::Open
            | TokenKind::Provides
            | TokenKind::Requires
            | TokenKind::Uses
            | TokenKind::With
            | TokenKind::When
            | TokenKind::To
            | TokenKind::Exports
            | TokenKind::Opens
            | TokenKind::Transitive
            | TokenKind::Byte
            | TokenKind::Short
            | TokenKind::Int
            | TokenKind::Long
            | TokenKind::Char
            | TokenKind::Float
            | TokenKind::Double
            | TokenKind::Boolean
            | TokenKind::Switch
    )
}

fn token_to_unary_op(kind: &TokenKind) -> Option<UnaryOp> {
    match kind {
        TokenKind::Minus => Some(UnaryOp::Neg),
        TokenKind::Bang => Some(UnaryOp::Not),
        TokenKind::Tilde => Some(UnaryOp::BitNot),
        TokenKind::PlusPlus => Some(UnaryOp::PreInc),
        TokenKind::MinusMinus => Some(UnaryOp::PreDec),
        _ => None,
    }
}

fn parse_unary_expr(input: &ParseStream) -> Result<Expr> {
    let span = input.peek().span;
    if input.is(&TokenKind::Plus) {
        input.next();
        return parse_unary_expr(input);
    }
    if let Some(op) = token_to_unary_op(&input.peek().kind) {
        input.next();
        let expr = parse_unary_expr(input)?;
        return Ok(Expr::Unary(UnaryExpr {
            op,
            op_span: span,
            expr: Box::new(expr),
            is_postfix: false,
        }));
    }
    parse_postfix_expr(input)
}

fn parse_postfix_expr(input: &ParseStream) -> Result<Expr> {
    let mut expr = parse_primary_expr(input)?;

    loop {
        match &input.peek().kind {
            TokenKind::Dot => {
                let dot_span = input.peek().span;
                input.next();
                if input.is(&TokenKind::New) {
                    // Inner class creation: Outer.new InnerType(...)
                    let new_span = input.peek().span;
                    input.next();
                    let type_args = parse_optional_type_arguments(input);
                    let class_type = parse_path(input)?;
                    input.expect(TokenKind::LParen)?;
                    let open = input.peek().span;
                    let args = input.parse_terminated(parse_expression)?;
                    input.expect(TokenKind::RParen)?;
                    let close = input.peek().span;
                    let body = if input.is(&TokenKind::LBrace) {
                        let dl = parse_class_body_decl_list(input)?;
                        Some(dl)
                    } else {
                        None
                    };
                    expr = Expr::NewClass(NewClassExpr {
                        new_span,
                        type_args,
                        class_type,
                        paren_span: (open, close),
                        args,
                        body,
                    });
                } else if input.is(&TokenKind::This) {
                    let this_span = input.peek().span;
                    input.next();
                    expr = Expr::FieldAccess(FieldAccessExpr {
                        target: Box::new(expr),
                        dot_span,
                        field: Ident::new("this".to_string(), this_span),
                    });
                } else if input.is(&TokenKind::Class) {
                    let class_span = input.peek().span;
                    input.next();
                    let ty = expr_to_type(&expr)?;
                    expr = Expr::ClassLit {
                        type_expr: Box::new(ty),
                        dot_span,
                        class_span,
                    };
                } else if input.is(&TokenKind::Super) {
                    let super_span = input.peek().span;
                    input.next();
                    if input.is(&TokenKind::ColonColon) {
                        let colon_colon_span = input.peek().span;
                        input.next();
                        let type_args = parse_optional_type_arguments(input);
                        let method_name = input.parse_ident()?;
                        expr = Expr::MethodRef(MethodRefExpr {
                            target: MethodRefTarget::SuperFromType {
                                type_name: expr_to_path(expr)?,
                                dot_span,
                                super_span,
                            },
                            colon_colon_span,
                            type_args,
                            method_name,
                        });
                    } else if input.is(&TokenKind::Dot) {
                        // Qualified super method call: Outer.super.method(args)
                        // Treat Outer.super as a special expression and continue parsing
                        // the .method() part in the next iteration
                        expr = Expr::FieldAccess(FieldAccessExpr {
                            target: Box::new(expr),
                            dot_span,
                            field: Ident::new("super".to_string(), super_span),
                        });
                    } else {
                        return Err(crate::error::Error::new(super_span, "unexpected super"));
                    }
                } else if input.is(&TokenKind::New) {
                    let _ = input.next();
                    // Inner class creation
                    continue;
                } else if input.is(&TokenKind::Lt) {
                    // Explicit type arguments: obj.<Type>method(args)
                    let type_args = parse_type_arguments(input)?;
                    let method = input.parse_ident()?;
                    input.expect(TokenKind::LParen)?;
                    let open = input.peek().span;
                    let args = input.parse_terminated(parse_expression)?;
                    input.expect(TokenKind::RParen)?;
                    let close = input.peek().span;
                    expr = Expr::MethodCall(MethodCallExpr {
                        receiver: Some(Box::new(expr)),
                        type_args: Some(type_args),
                        method,
                        paren_span: (open, close),
                        args,
                    });
                } else {
                    let field = input.parse_ident().unwrap_or_else(|_| {
                        let sp = input.peek().span;
                        let name = format!("{}", input.peek().kind);
                        input.next();
                        Ident::new(name, sp)
                    });
                    expr = Expr::FieldAccess(FieldAccessExpr {
                        target: Box::new(expr),
                        dot_span,
                        field,
                    });
                }
            }
            TokenKind::LParen => {
                // Method call or cast
                // If current expr is a simple ident, it's a method call
                let open = input.peek().span;
                input.next();
                let args = input.parse_terminated(parse_expression)?;
                input.expect(TokenKind::RParen)?;
                let close = input.peek().span;

                // Check if this was actually a cast: (Type) expr
                // Cast is handled in primary, so this is always a method call
                let (method, receiver) = match &expr {
                    Expr::Ident(name) => (name.clone(), None),
                    Expr::FieldAccess(f) => {
                        (f.field.clone(), Some(Box::new(f.target.as_ref().clone())))
                    }
                    _ => return Err(crate::error::Error::new(open, "expected method name")),
                };

                // Check for type arguments before the method call
                // Actually type args are before the name, e.g., name.<Type>method()
                // This should be handled differently...

                expr = Expr::MethodCall(MethodCallExpr {
                    receiver,
                    type_args: None,
                    method,
                    paren_span: (open, close),
                    args,
                });
            }
            TokenKind::LBracket => {
                let open = input.peek().span;
                input.next();
                // Check for empty brackets `[]` — could be array type suffix
                // before `.class` or `::` (e.g., `float[].class`, `String[]::new`)
                if input.is(&TokenKind::RBracket) {
                    input.next();
                    let close = input.peek().span;
                    // Collect more `[]` pairs
                    let mut dims = vec![crate::ast::ArrayDim {
                        bracket_span: (open, close),
                        annotations: vec![],
                    }];
                    while input.is(&TokenKind::LBracket) {
                        let d_open = input.peek().span;
                        input.next();
                        input.expect(TokenKind::RBracket)?;
                        let d_close = input.peek().span;
                        dims.push(crate::ast::ArrayDim {
                            bracket_span: (d_open, d_close),
                            annotations: vec![],
                        });
                    }
                    // Now check if `.class` or `::` follows — if so, this is a type suffix
                    if input.is(&TokenKind::Dot) && input.look_ahead(1).kind == TokenKind::Class {
                        // Type suffix: convert expr to type, then handle .class
                        let base_ty = expr_to_type(&expr)?;
                        let span = base_ty.span().join(dims.last().unwrap().bracket_span.1);
                        let ty = Type::Reference(ReferenceType::Array(ArrayType {
                            elem_type: Box::new(base_ty),
                            dims,
                            span,
                        }));
                        let dot_span = input.peek().span;
                        input.next(); // consume '.'
                        let class_span = input.peek().span;
                        input.next(); // consume 'class'
                        expr = Expr::ClassLit {
                            type_expr: Box::new(ty),
                            dot_span,
                            class_span,
                        };
                    } else if input.is(&TokenKind::ColonColon) {
                        // Method reference with array type: e.g., `ComponentName[]::new`
                        // Skip the `[]` — the `::` handler below will handle the rest
                        // The expression remains as the base identifier, which will be
                        // converted to a path by the `::` handler. Not perfect for array
                        // types but better than failing.
                        // Leave expr as-is (the base identifier)
                    } else {
                        // Standalone empty `[]` — probably a syntax error in context,
                        // but recover gracefully by treating as array type expression
                        expr =
                            Expr::Ident(crate::ident::Ident::new("_array_type_".to_string(), open));
                    }
                } else {
                    let index = parse_expression(input)?;
                    input.expect(TokenKind::RBracket)?;
                    let close = input.peek().span;
                    expr = Expr::ArrayAccess(ArrayAccessExpr {
                        array: Box::new(expr),
                        index: Box::new(index),
                        bracket_span: (open, close),
                    });
                }
            }
            TokenKind::PlusPlus => {
                let op_span = input.peek().span;
                input.next();
                expr = Expr::Unary(UnaryExpr {
                    op: UnaryOp::PostInc,
                    op_span,
                    expr: Box::new(expr),
                    is_postfix: true,
                });
            }
            TokenKind::MinusMinus => {
                let op_span = input.peek().span;
                input.next();
                expr = Expr::Unary(UnaryExpr {
                    op: UnaryOp::PostDec,
                    op_span,
                    expr: Box::new(expr),
                    is_postfix: true,
                });
            }
            TokenKind::ColonColon => {
                let colon_colon_span = input.peek().span;
                input.next();
                let type_args = parse_optional_type_arguments(input);
                if input.is(&TokenKind::New) {
                    // Constructor reference
                    let method_name = Ident::new("new".to_string(), input.peek().span);
                    input.next();
                    expr = Expr::MethodRef(MethodRefExpr {
                        target: MethodRefTarget::Type(expr_to_path(expr)?),
                        colon_colon_span,
                        type_args,
                        method_name,
                    });
                } else {
                    let method_name = input.parse_ident()?;
                    match &expr {
                        Expr::Ident(_) => {
                            expr = Expr::MethodRef(MethodRefExpr {
                                target: MethodRefTarget::Type(expr_to_path(expr)?),
                                colon_colon_span,
                                type_args,
                                method_name,
                            });
                        }
                        _ => {
                            expr = Expr::MethodRef(MethodRefExpr {
                                target: MethodRefTarget::Expr(Box::new(expr)),
                                colon_colon_span,
                                type_args,
                                method_name,
                            });
                        }
                    }
                }
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn expr_to_type(expr: &Expr) -> Result<Type> {
    match expr {
        Expr::Ident(i) => match i.name.as_str() {
            "byte" => Ok(Type::Primitive(PrimitiveType::Byte)),
            "short" => Ok(Type::Primitive(PrimitiveType::Short)),
            "int" => Ok(Type::Primitive(PrimitiveType::Int)),
            "long" => Ok(Type::Primitive(PrimitiveType::Long)),
            "char" => Ok(Type::Primitive(PrimitiveType::Char)),
            "float" => Ok(Type::Primitive(PrimitiveType::Float)),
            "double" => Ok(Type::Primitive(PrimitiveType::Double)),
            "boolean" => Ok(Type::Primitive(PrimitiveType::Boolean)),
            "void" => Ok(Type::Void(i.span())),
            _ => Ok(Type::Reference(ReferenceType::ClassOrInterfaceType(
                ClassOrInterfaceType {
                    path: Path::from_ident(i.clone()),
                    annotations_prefix: vec![],
                },
            ))),
        },
        Expr::FieldAccess(_) => {
            let path = expr_to_path(expr.clone())?;
            Ok(Type::Reference(ReferenceType::ClassOrInterfaceType(
                ClassOrInterfaceType {
                    path,
                    annotations_prefix: vec![],
                },
            )))
        }
        _ => Err(crate::error::Error::new(expr.span(), "expected type")),
    }
}

fn expr_to_path(expr: Expr) -> Result<Path> {
    match expr {
        Expr::Ident(i) => Ok(Path::from_ident(i)),
        Expr::FieldAccess(f) => {
            let mut target_path = expr_to_path(*f.target)?;
            target_path.segments.push(PathSegment {
                ident: f.field,
                args: None,
            });
            Ok(target_path)
        }
        _ => Err(crate::error::Error::new(expr.span(), "expected type name")),
    }
}

fn parse_primary_expr(input: &ParseStream) -> Result<Expr> {
    let span = input.peek().span;

    // Parenthesized expression, cast, or lambda
    if input.is(&TokenKind::LParen) {
        let open = span;
        input.next();

        // Try to determine if this is a cast, lambda, or parenthesized expression
        let saved = input.cursor();

        // Try parsing as type + ')' for cast
        if let Ok(ty) = parse_type(input) {
            // Handle intersection types: (Type & Type & ...) expr
            let mut intersection_types = Vec::new();
            while input.eat(&TokenKind::Amp) {
                if let Ok(ty) = parse_type(input) {
                    intersection_types.push(ty);
                } else {
                    break;
                }
            }

            if input.is(&TokenKind::RParen) {
                // Check if what follows ')' can start a cast operand (unary expression).
                // If not (e.g., '?', ')', ';'), this is likely a parenthesized expression, not a cast.
                let next_after_rparen = input.look_ahead(1);
                let can_be_cast = can_start_unary_expr(&next_after_rparen.kind);

                if can_be_cast {
                    input.next();
                    let close = input.peek().span;
                    // Could be cast or lambda with typed params
                    // If next is '->', it's a lambda
                    if input.is(&TokenKind::Arrow) {
                        let arrow_span = input.peek().span;
                        input.next();
                        let body = parse_lambda_body(input)?;
                        return Ok(Expr::Lambda(LambdaExpr {
                            params: LambdaParams::List {
                                paren_span: (open, close),
                                params: vec![LambdaParam {
                                    modifiers: vec![],
                                    ty,
                                    name: crate::ident::Ident::new("_".to_string(), span),
                                }],
                            },
                            arrow_span,
                            body,
                            span: open.join(input.peek().span),
                        }));
                    }
                    let expr = parse_unary_expr(input)?;
                    // For intersection types, we create a special cast expression
                    // For now, we just use the first type (could be enhanced later)
                    return Ok(Expr::Cast(CastExpr {
                        paren_span: (open, close),
                        target_type: ty,
                        expr: Box::new(expr),
                    }));
                }
            }
        }

        input.set_cursor(saved);

        // Check for lambda: () -> ... or (params) -> ...
        // Cursor is at ')' (the first token after the consumed '(').
        // Scan forward: look_ahead(0) is ')', which matches our '('.
        {
            let lambda_saved = input.cursor();
            let mut depth = 1;
            let mut offset = 0;
            loop {
                let tok = input.look_ahead(offset);
                match &tok.kind {
                    TokenKind::LParen => depth += 1,
                    TokenKind::RParen => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    TokenKind::Eof => break,
                    _ => {}
                }
                offset += 1;
            }
            // offset points to ')' position relative to current cursor
            // Check if token AFTER ')' is '->'
            if input.look_ahead(offset + 1).kind == TokenKind::Arrow {
                // Parse lambda params (cursor is after '('; parse_lambda_params_after_lparen
                // handles parsing until ')')
                let params = parse_lambda_params_after_lparen(input)?;
                let arrow_span = input.peek().span;
                input.next();
                let body = parse_lambda_body(input)?;
                return Ok(Expr::Lambda(LambdaExpr {
                    params,
                    arrow_span,
                    body,
                    span: open.join(input.peek().span),
                }));
            }
            input.set_cursor(lambda_saved);
        }

        // Parenthesized expression
        let expr = parse_expression(input)?;
        input.expect(TokenKind::RParen)?;
        let close = input.peek().span;
        return Ok(Expr::Paren {
            paren_span: (open, close),
            expr: Box::new(expr),
        });
    }

    match &input.peek().kind {
        TokenKind::IntegerLit(v) => {
            let value = v.clone();
            let sp = input.peek().span;
            input.next();
            Ok(Expr::Literal(Lit::Int(IntLit { value, span: sp })))
        }
        TokenKind::FloatLit(v) => {
            let value = v.clone();
            let sp = input.peek().span;
            input.next();
            Ok(Expr::Literal(Lit::Float(FloatLit { value, span: sp })))
        }
        TokenKind::BoolLit(b) => {
            let value = *b;
            let sp = input.peek().span;
            input.next();
            Ok(Expr::Literal(Lit::Bool(BoolLit { value, span: sp })))
        }
        TokenKind::CharLit(v) => {
            let value = v.clone();
            let sp = input.peek().span;
            input.next();
            Ok(Expr::Literal(Lit::Char(CharLit { value, span: sp })))
        }
        TokenKind::StringLit(v) => {
            let value = v.clone();
            let sp = input.peek().span;
            input.next();
            Ok(Expr::Literal(Lit::Str(StrLit { value, span: sp })))
        }
        TokenKind::NullLit => {
            let sp = input.peek().span;
            input.next();
            Ok(Expr::Literal(Lit::Null(NullLit { span: sp })))
        }
        TokenKind::This => {
            let sp = input.peek().span;
            input.next();
            Ok(Expr::This(sp))
        }
        TokenKind::Super => {
            let sp = input.peek().span;
            input.next();
            Ok(Expr::Super(sp))
        }
        TokenKind::New => parse_new_expr(input),
        TokenKind::Switch => parse_switch_expr(input),
        TokenKind::LBrace => Ok(Expr::ArrayInit(parse_array_init(input)?)),
        _ => {
            // Check for identifier (possibly with method reference)
            // Also allow primitive type keywords and void as identifiers for .class expressions
            if input.is_any_ident()
                || is_contextual_type_keyword(&input.peek().kind)
                || is_primitive_type_token(&input.peek().kind)
                || input.is(&TokenKind::Void)
            {
                let ident = input.parse_ident()?;
                // Check for single-param lambda: `x -> expr`
                if input.is(&TokenKind::Arrow) {
                    let ident_span = ident.span;
                    let arrow_span = input.peek().span;
                    input.next();
                    let body = parse_lambda_body(input)?;
                    let end = body.span();
                    return Ok(Expr::Lambda(LambdaExpr {
                        params: LambdaParams::Single(ident),
                        arrow_span,
                        body,
                        span: ident_span.join(end),
                    }));
                }
                // Check for type arguments followed by :: (method reference with type args)
                // e.g., Collection<T>::add, Class<?>[]::new
                if input.is(&TokenKind::Lt) {
                    let saved = input.save_state();
                    if let Ok(type_args) = parse_type_arguments(input) {
                        // Skip optional array dims [] between type args and ::
                        // e.g., Class<?>[]::new has [] between > and ::
                        while input.is(&TokenKind::LBracket)
                            && input.look_ahead(1).kind == TokenKind::RBracket
                        {
                            input.next(); // [
                            input.next(); // ]
                        }
                        if input.is(&TokenKind::ColonColon) {
                            let colon_colon_span = input.peek().span;
                            input.next();
                            let type_args2 = parse_optional_type_arguments(input);
                            let method_name = if input.is(&TokenKind::New) {
                                let name = Ident::new("new".to_string(), input.peek().span);
                                input.next();
                                name
                            } else {
                                input.parse_ident()?
                            };
                            let path = Path {
                                segments: vec![PathSegment {
                                    ident,
                                    args: Some(type_args),
                                }],
                                span: span.join(input.peek().span),
                            };
                            return Ok(Expr::MethodRef(MethodRefExpr {
                                target: MethodRefTarget::Type(path),
                                colon_colon_span,
                                type_args: type_args2,
                                method_name,
                            }));
                        }
                    }
                    input.restore_state(saved);
                }
                Ok(Expr::Ident(ident))
            } else {
                Err(crate::error::Error::new(span, "expected expression"))
            }
        }
    }
}

fn parse_lambda_params_after_lparen(input: &ParseStream) -> Result<LambdaParams> {
    let open = input.peek().span;
    let mut params = Vec::new();
    let mut idents = Vec::new();

    while !input.is(&TokenKind::RParen) && !input.is_empty() {
        let saved = input.cursor();
        // Try to parse as typed parameter: Type name
        if let Ok(ty) = parse_type(input) {
            if input.is_any_ident() {
                let name = input.parse_ident()?;
                params.push(LambdaParam {
                    modifiers: vec![],
                    ty,
                    name,
                });
                if !input.eat(&TokenKind::Comma) {
                    break;
                }
                continue;
            }
            input.set_cursor(saved);
        } else {
            input.set_cursor(saved);
        }
        // Parse as inferred parameter: just an ident
        let ident = input.parse_ident()?;
        idents.push(ident);
        if !input.eat(&TokenKind::Comma) {
            break;
        }
    }

    input.expect(TokenKind::RParen)?;
    let close = input.peek().span;

    if !params.is_empty() && idents.is_empty() {
        Ok(LambdaParams::List {
            paren_span: (open, close),
            params,
        })
    } else if params.is_empty() && !idents.is_empty() {
        Ok(LambdaParams::IdentList {
            paren_span: (open, close),
            idents,
        })
    } else if params.is_empty() && idents.is_empty() {
        Ok(LambdaParams::IdentList {
            paren_span: (open, close),
            idents: vec![],
        })
    } else {
        Ok(LambdaParams::IdentList {
            paren_span: (open, close),
            idents,
        })
    }
}

fn parse_lambda_body(input: &ParseStream) -> Result<LambdaBody> {
    if input.is(&TokenKind::LBrace) {
        Ok(LambdaBody::Block(parse_block(input)?))
    } else {
        Ok(LambdaBody::Expr(Box::new(parse_expression(input)?)))
    }
}

fn parse_new_expr(input: &ParseStream) -> Result<Expr> {
    let new_span = input.peek().span;
    input.expect(TokenKind::New)?;
    let type_args = parse_optional_type_arguments(input);

    // Parse the element/base type WITHOUT consuming array dimensions
    let annotations = parse_type_annotations(input);
    let base_type = if is_primitive_type_token(&input.peek().kind) {
        let prim = parse_primitive_type(input)?;
        Type::Primitive(prim)
    } else if input.is_any_ident() || is_contextual_type_keyword(&input.peek().kind) {
        let path = parse_path(input)?;
        Type::Reference(ReferenceType::ClassOrInterfaceType(ClassOrInterfaceType {
            path,
            annotations_prefix: annotations,
        }))
    } else {
        return Err(crate::error::Error::new(
            new_span,
            "expected type after new",
        ));
    };

    // Check for array creation: new Type[expr] or new Type[]
    if input.is(&TokenKind::LBracket) {
        let mut dim_exprs = Vec::new();
        let mut dims = Vec::new();

        while input.is(&TokenKind::LBracket) {
            let open = input.peek().span;
            input.next();
            if input.is(&TokenKind::RBracket) {
                input.next();
                let close = input.peek().span;
                dims.push(ArrayDim {
                    bracket_span: (open, close),
                    annotations: Vec::new(),
                });
            } else {
                let anns = parse_type_annotations(input);
                let expr = parse_expression(input)?;
                input.expect(TokenKind::RBracket)?;
                let close = input.peek().span;
                dim_exprs.push(ArrayDimExpr {
                    annotations: anns,
                    bracket_span: (open, close),
                    expr: Box::new(expr),
                });
            }
        }

        let initializer = if input.is(&TokenKind::LBrace) {
            Some(parse_array_init(input)?)
        } else {
            None
        };

        return Ok(Expr::ArrayNew(ArrayNewExpr {
            new_span,
            elem_type: base_type,
            dim_exprs,
            dims,
            initializer,
        }));
    }

    // Class instantiation: new Type(args) or new Type { ... }
    if input.is(&TokenKind::LParen) || input.is(&TokenKind::LBrace) {
        let class_type = match &base_type {
            Type::Reference(ReferenceType::ClassOrInterfaceType(cit)) => cit.path.clone(),
            _ => {
                return Err(crate::error::Error::new(
                    input.peek().span,
                    "expected class type",
                ));
            }
        };

        if input.is(&TokenKind::LParen) {
            input.expect(TokenKind::LParen)?;
            let open = input.peek().span;
            let args = input.parse_terminated(parse_expression)?;
            input.expect(TokenKind::RParen)?;
            let close = input.peek().span;
            let body = if input.is(&TokenKind::LBrace) {
                let dl = parse_class_body_decl_list(input)?;
                Some(dl)
            } else {
                None
            };
            return Ok(Expr::NewClass(NewClassExpr {
                new_span,
                type_args,
                class_type,
                paren_span: (open, close),
                args,
                body,
            }));
        }

        // Anonymous class with no args
        let dl = parse_class_body_decl_list(input)?;
        let body = dl;
        return Ok(Expr::NewClass(NewClassExpr {
            new_span,
            type_args,
            class_type,
            paren_span: (Span::call_site(), Span::call_site()),
            args: Vec::new(),
            body: Some(body),
        }));
    }

    Err(crate::error::Error::new(
        new_span,
        "expected type after new",
    ))
}

fn parse_switch_expr(input: &ParseStream) -> Result<Expr> {
    let switch_span = input.peek().span;
    input.expect(TokenKind::Switch)?;
    input.expect(TokenKind::LParen)?;
    let open = input.peek().span;
    let selector = parse_expression(input)?;
    input.expect(TokenKind::RParen)?;
    let close = input.peek().span;
    input.expect(TokenKind::LBrace)?;
    let brace_open = input.peek().span;
    let mut cases = Vec::new();
    while !input.is(&TokenKind::RBrace) && !input.is_empty() {
        cases.push(parse_switch_arm(input)?);
    }
    input.expect(TokenKind::RBrace)?;
    let brace_close = input.peek().span;
    Ok(Expr::Switch(SwitchExpr {
        switch_span,
        paren_span: (open, close),
        selector: Box::new(selector),
        brace_span: (brace_open, brace_close),
        cases,
    }))
}

fn parse_switch_arm(input: &ParseStream) -> Result<SwitchArm> {
    let labels = parse_switch_labels(input)?;
    let label = labels
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::Error::new(input.peek().span, "expected case or default"))?;

    if input.eat(&TokenKind::Arrow) {
        let arrow = input.peek().span;
        if input.is(&TokenKind::Throw) {
            let _throw_span = input.peek().span;
            input.next();
            let expr = parse_expression(input)?;
            input.expect(TokenKind::Semicolon)?;
            Ok(SwitchArm::Throw(label, arrow, expr))
        } else if input.is(&TokenKind::LBrace) {
            let block = parse_block(input)?;
            Ok(SwitchArm::Block(label, arrow, block))
        } else {
            let expr = parse_expression(input)?;
            input.expect(TokenKind::Semicolon)?;
            Ok(SwitchArm::Expr(label, arrow, expr))
        }
    } else {
        input.expect(TokenKind::Colon)?;
        let mut stmts = Vec::new();
        while !is_switch_label_start(input) && !input.is(&TokenKind::RBrace) && !input.is_empty() {
            stmts.push(parse_statement(input)?);
        }
        Ok(SwitchArm::Colon(label, stmts))
    }
}

// ============================================================================
// Pattern
// ============================================================================

fn parse_pattern(input: &ParseStream) -> Result<Pattern> {
    let annotations = parse_annotations(input)?;

    // Try record pattern: Type(Component1, Component2)
    // or type pattern: Type name
    let ty = parse_type(input)?;

    if input.is(&TokenKind::LParen) {
        // Record pattern
        input.next();
        let mut components = Vec::new();
        while !input.is(&TokenKind::RParen) && !input.is_empty() {
            components.push(parse_pattern(input)?);
            input.eat(&TokenKind::Comma);
        }
        input.expect(TokenKind::RParen)?;
        let start = annotations.first().map(|a| a.span()).unwrap_or(ty.span());
        let end = input.peek().span;
        Ok(Pattern::RecordPattern(RecordPattern {
            record_type: ty,
            components,
            span: start.join(end),
        }))
    } else {
        // Type pattern
        let name = input.parse_ident().ok();
        let start = annotations.first().map(|a| a.span()).unwrap_or(ty.span());
        let end = name.as_ref().map(|n| n.span()).unwrap_or(ty.span());
        Ok(Pattern::TypePattern(TypePattern {
            annotations,
            ty,
            name,
            span: start.join(end),
        }))
    }
}
