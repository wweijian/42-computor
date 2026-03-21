use computor_v1::parse;
use computor_v1::parse::{ERR_BAD_DEG, ERR_DIV_ZERO, ERR_EQN, ERR_HANG, ERR_MAL};
use std::collections::HashMap;

/// Returns (deg0, deg1, deg2) from the map, defaulting absent keys to 0.0.
fn coeffs(map: &HashMap<i32, f64>) -> (f64, f64, f64) {
    (
        map.get(&0).copied().unwrap_or(0.0),
        map.get(&1).copied().unwrap_or(0.0),
        map.get(&2).copied().unwrap_or(0.0),
    )
}

// --- success cases ---

#[test]
fn parse_x_equals_zero() {
    let map = parse::populate_map("X = 0").unwrap();
    assert_eq!(coeffs(&map), (0.0, 1.0, 0.0));
}

#[test]
fn parse_x_squared_equals_zero() {
    let map = parse::populate_map("X^2 = 0").unwrap();
    assert_eq!(coeffs(&map), (0.0, 0.0, 1.0));
}

#[test]
fn parse_full_quadratic() {
    // 2 * X^2 + 3 * X + 1 = 0
    let map = parse::populate_map("2 * X^2 + 3 * X + 1 = 0").unwrap();
    assert_eq!(coeffs(&map), (1.0, 3.0, 2.0));
}

#[test]
fn parse_negative_coeff() {
    // X^2 - 2 * X + 1 = 0
    let map = parse::populate_map("X^2 - 2 * X + 1 = 0").unwrap();
    assert_eq!(coeffs(&map), (1.0, -2.0, 1.0));
}

#[test]
fn parse_terms_cancel_across_equals() {
    // 5 * X^0 = 5 * X^0  =>  all terms cancel to 0
    let map = parse::populate_map("5 * X^0 = 5 * X^0").unwrap();
    assert_eq!(coeffs(&map), (0.0, 0.0, 0.0));
}

#[test]
fn parse_x_equals_x() {
    let map = parse::populate_map("X = X").unwrap();
    assert_eq!(coeffs(&map), (0.0, 0.0, 0.0));
}

#[test]
fn parse_negative_leading_term() {
    // -X + 1 = 0
    let map = parse::populate_map("-X + 1 = 0").unwrap();
    assert_eq!(coeffs(&map), (1.0, -1.0, 0.0));
}

#[test]
fn parse_decimal_coefficient() {
    let map = parse::populate_map("1.5 * X = 0").unwrap();
    assert_eq!(coeffs(&map), (0.0, 1.5, 0.0));
}

#[test]
fn parse_leading_plus_as_polarity() {
    // '+' at the start is just a positive sign
    let map = parse::populate_map("+ X = 0").unwrap();
    assert_eq!(coeffs(&map), (0.0, 1.0, 0.0));
}

// --- whitespace ---

#[test]
fn parse_no_spaces() {
    let map = parse::populate_map("X=0").unwrap();
    assert_eq!(coeffs(&map), (0.0, 1.0, 0.0));
}

#[test]
fn parse_extra_spaces_around_equals() {
    let map = parse::populate_map("X  =  0").unwrap();
    assert_eq!(coeffs(&map), (0.0, 1.0, 0.0));
}

#[test]
fn parse_extra_spaces_around_operators() {
    let map = parse::populate_map("X^2  +  3 * X  +  2 = 0").unwrap();
    assert_eq!(coeffs(&map), (2.0, 3.0, 1.0));
}

#[test]
fn parse_spaces_inside_coefficient() {
    // space between coefficient and X
    let map = parse::populate_map("2 * X^2 = 0").unwrap();
    assert_eq!(coeffs(&map), (0.0, 0.0, 2.0));
}

#[test]
fn parse_no_spaces_compact_quadratic() {
    let map = parse::populate_map("X^2+X+1=0").unwrap();
    assert_eq!(coeffs(&map), (1.0, 1.0, 1.0));
}

// --- failure cases ---

#[test]
fn parse_degree_too_high() {
    let err = parse::populate_map("X^3 = 0").unwrap_err();
    assert!(err.contains(ERR_BAD_DEG), "got: {err}");
}

#[test]
fn parse_unexpected_character() {
    let err = parse::populate_map("@ = 0").unwrap_err();
    assert!(err.contains(ERR_EQN), "got: {err}");
}

#[test]
fn parse_hanging_operator_mid_expression() {
    // operator with nothing after it before '='
    let err = parse::populate_map("X + = 0").unwrap_err();
    assert!(err.contains(ERR_HANG), "got: {err}");
}

#[test]
fn parse_malformed_double_operator() {
    // two operators in a row
    let err = parse::populate_map("X * + 1 = 0").unwrap_err();
    assert!(err.contains(ERR_MAL), "got: {err}");
}

#[test]
fn parse_division_by_zero() {
    let err = parse::populate_map("X / 0 = 0").unwrap_err();
    assert!(err.contains(ERR_DIV_ZERO), "got: {err}");
}
