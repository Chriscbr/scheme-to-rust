use im_rc::vector;
use scheme_to_rust::{tc_with_env, type_check, Env, Type};

#[test]
fn test_typecheck_prims() {
    let exp = lexpr::from_str("3").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("-497").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("#t").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);

    let exp = lexpr::from_str("#f").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);

    let exp = lexpr::from_str("true").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);

    let exp = lexpr::from_str("false").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);

    let exp = lexpr::from_str("\"true\"").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Str);

    let exp = lexpr::from_str("\"foo\"").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Str);

    let exp = lexpr::from_str("\"\"").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Str);
}

#[test]
fn test_typecheck_binops_happy() {
    let exp = lexpr::from_str("(+ 3 5)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("(* 3 5)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("(- 3 5)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("(/ 3 5)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("(+ (* 4 5) (- 5 2))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str(r#"(concat "hello " "world")"#).unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Str);

    let exp = lexpr::from_str("(and true false)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);

    let exp = lexpr::from_str("(or true false)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);
}

#[test]
fn test_typecheck_binops_sad() {
    let exp = lexpr::from_str("(+ 3 true)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    let exp = lexpr::from_str("(* 3 true)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    let exp = lexpr::from_str(r#"(- false "hello")"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    let exp = lexpr::from_str(r#"(/ "foo" 3)"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    let exp = lexpr::from_str(r#"(concat 3 "world")"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    let exp = lexpr::from_str(r#"(and 3 6)"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    let exp = lexpr::from_str(r#"(or "hello" "world")"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);
}

#[test]
fn test_typecheck_lists_happy() {
    let exp = lexpr::from_str("(null int)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::List(Box::from(Type::Int)));

    let exp = lexpr::from_str("(null bool)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::List(Box::from(Type::Bool)));

    let exp = lexpr::from_str("(cons 3 (null int))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::List(Box::from(Type::Int)));

    let exp = lexpr::from_str("(cons 3 (cons 4 (null int)))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::List(Box::from(Type::Int)));

    let exp = lexpr::from_str(r#"(cons "foo" (cons "bar" (null string)))"#).unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::List(Box::from(Type::Str)));

    // NOTE: these two cases probably looks weird, but it is the simplest solution
    // we can just assume car/cdr of an empty list is the type of the list's items
    // or the type of the list respectively (or that the program just panics?)
    let exp = lexpr::from_str("(car (null int))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("(cdr (null int))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::List(Box::from(Type::Int)));

    let exp = lexpr::from_str("(car (cons 3 (null int)))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("(cdr (cons 3 (null int)))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::List(Box::from(Type::Int)));

    let exp = lexpr::from_str("(null? (null int))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);
}

#[test]
fn test_typecheck_lists_sad() {
    // missing type
    let exp = lexpr::from_str("(null)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // not valid in our language
    let exp = lexpr::from_str("null").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // too many types
    let exp = lexpr::from_str("(null int bool)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // type of car does not match type of cdr
    let exp = lexpr::from_str(r#"(cons "hey" (null int))"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // invalid argument to car
    let exp = lexpr::from_str("(car 3)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // invalid argument to cdr
    let exp = lexpr::from_str("(cdr 3)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);
}

#[test]
fn test_typecheck_let_happy() {
    let exp = lexpr::from_str("(let ((x 23)) (+ x 24))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);
}

#[test]
fn test_typecheck_let_sad() {
    // one variable missing
    let exp = lexpr::from_str("(let ([x 23]) (+ x y))").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);
}

#[test]
fn test_typecheck_sideeffects_happy() {
    let exp = lexpr::from_str("(begin (+ 3 5))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("(begin (+ 3 5) (- 4 1))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("(let ((x 3)) (set! x 7))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);
}

#[test]
fn test_typecheck_sideeffects_sad() {
    // begin expression is missing arguments
    let exp = lexpr::from_str(r#"(begin)"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // intermediary expression in begin is not valid
    let exp = lexpr::from_str(r#"(begin (+ 3 "hello") (- 4 1))"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // last expression in begin is not valid
    let exp = lexpr::from_str(r#"(begin (+ 3 4) (- 4 "hello"))"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // using set! before variable is defined
    let exp = lexpr::from_str("(set! x 7)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);
}

#[test]
fn test_typecheck_local_scoping() {
    // local variable overrides outer variable
    let exp = lexpr::from_str(
        r#"(let ((x "hello"))
                (let ((x 23))
                    (+ x 24)))"#,
    )
    .unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    // binding from let lasts past its scope
    let exp = lexpr::from_str("(+ (let ((x 5)) (+ x 3)) x)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // binding from lambda lasts past its scope
    let exp = lexpr::from_str("(+ ((lambda ((x : int)) : int x) 3) x)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // let bindings with same names of conflicting types
    let exp = lexpr::from_str("(and (let ((x 5)) (< x 3)) (let ((x false)) (or x true)))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);

    // lambda bindings with same names of conflicting types
    let exp =
        lexpr::from_str("(and ((lambda ((x : int)) : bool (< x 3)) 5) ((lambda ((x : bool)) : bool (and x true)) false))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);

    // using set! after variable goes out of scope
    let exp = lexpr::from_str("(begin (let ((x 3)) (+ x 5)) (set! x 7))").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);
}

#[test]
fn test_test_typecheck_if_happy() {
    let exp = lexpr::from_str(r#"(if (< 3 4) 1 -1)"#).unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);
}

#[test]
fn test_typecheck_if_sad() {
    // missing alternate
    let exp = lexpr::from_str(r#"(if (< 3 4) 4)"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // invalid predicate
    let exp = lexpr::from_str(r#"(if 3 4 5)"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // consequent and alternate do not match
    let exp = lexpr::from_str(r#"(if (< 3 4) "hello" 5)"#).unwrap();
    assert_eq!(type_check(&exp).is_err(), true);
}

#[test]
fn test_typecheck_lambda_happy() {
    let exp = lexpr::from_str("(lambda () : int 3)").unwrap();
    assert_eq!(
        type_check(&exp).unwrap(),
        Type::Func(vector![], Box::from(Type::Int))
    );

    let exp = lexpr::from_str("(lambda ((x : int)) : bool (< x 5))").unwrap();
    assert_eq!(
        type_check(&exp).unwrap(),
        Type::Func(vector![Type::Int], Box::from(Type::Bool))
    );

    let exp = lexpr::from_str("(lambda ((x : int) (y : int)) : int (* x y))").unwrap();
    assert_eq!(
        type_check(&exp).unwrap(),
        Type::Func(vector![Type::Int, Type::Int], Box::from(Type::Int))
    );

    let exp =
        lexpr::from_str("(lambda ((fn : (-> int int bool)) (x : int) (y : int)) : bool (fn x y))")
            .unwrap();
    assert_eq!(
        type_check(&exp).unwrap(),
        Type::Func(
            vector![
                Type::Func(vector![Type::Int, Type::Int], Box::from(Type::Bool)),
                Type::Int,
                Type::Int
            ],
            Box::from(Type::Bool)
        )
    );
}

#[test]
fn test_typecheck_lambda_sad() {
    // mismatched return type
    let exp = lexpr::from_str("(lambda () : bool 3)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // input types do not work in body
    let exp = lexpr::from_str("(lambda ((x : bool) (y : bool)) : int (+ x y))").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);
}

#[test]
fn test_type_check_apply_happy() {
    let exp = lexpr::from_str("((lambda () : int 3))").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("((lambda ((x : int)) : bool (< x 5)) 3)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);

    let exp = lexpr::from_str("((lambda ((x : int) (y : int)) : int (* x y)) 5 6)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);

    let exp = lexpr::from_str("((lambda ((x : int) (y : int)) : int (* x y)) 5 6)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Int);
}

#[test]
fn test_type_check_apply_hof_happy() {
    // Note: this is basically (apply (lambda equivalent to <) 3 5)
    let exp = lexpr::from_str("((lambda ((fn : (-> int int bool)) (x : int) (y : int)) : bool (fn x y)) (lambda ((a : int) (b : int)) : bool (< a b)) 3 5)").unwrap();
    assert_eq!(type_check(&exp).unwrap(), Type::Bool);

    // "map" is recursive, so without implementing type-checking for define,
    // (which would create the binding from "map" to its type signature)
    // we must add the name to the environment for the unit test
    let exp = lexpr::from_str(
        "(lambda ((f : (-> int int)) (lst : (list int))) : (list int)
            (if (null? lst)
                (null int)
                (cons (f (car lst)) (map f (cdr lst)))))",
    )
    .unwrap();
    let map_type = Type::Func(
        vector![
            Type::Func(vector![Type::Int], Box::from(Type::Int)),
            Type::List(Box::from(Type::Int)),
        ],
        Box::from(Type::List(Box::from(Type::Int))),
    ); // (-> (-> int int) (list int) (list int))
    let mut env = Env::new();
    env = env.add_binding((String::from("map"), map_type.clone()));
    assert_eq!(tc_with_env(&exp, &mut env).unwrap(), map_type);
}

#[test]
fn test_type_check_apply_sad() {
    // missing parameters
    let exp = lexpr::from_str("((lambda ((x : int)) : bool (< x 5)))").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // too many parameters
    let exp = lexpr::from_str("((lambda ((x : int)) : bool (< x 5)) 3 5)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);

    // arg type does not match param type
    let exp = lexpr::from_str("((lambda ((x : int)) : bool (< x 5)) true)").unwrap();
    assert_eq!(type_check(&exp).is_err(), true);
}