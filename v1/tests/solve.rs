mod run;

// --- constant equations ---

#[test]
fn solve_zero_equals_zero_infinite() {
    // 0 = 0  =>  all X satisfy it
    run::stdout_contains("0 = 0", "infinite");
}

#[test]
fn solve_equal_constants_infinite() {
    // 5 * X^0 = 5 * X^0  =>  same as 0 = 0
    run::stdout_contains("5 * X^0 = 5 * X^0", "infinite");
}

#[test]
fn solve_nonzero_constant_no_solution() {
    // 1 = 0  =>  impossible
    run::stdout_contains("1 = 0", "no solutions");
}

// --- linear equations ---

#[test]
fn solve_x_equals_zero() {
    // X = 0  =>  X = 0
    run::stdout_contains("X = 0", "one solution");
    run::stdout_contains("X = 0", "0");
}

#[test]
fn solve_x_equals_positive() {
    // X = 3  =>  X = 3
    run::stdout_contains("X = 3", "one solution");
    run::stdout_contains("X = 3", "3");
}

#[test]
fn solve_x_equals_negative() {
    // X + 5 = 0  =>  X = -5
    run::stdout_contains("X + 5 = 0", "one solution");
    run::stdout_contains("X + 5 = 0", "-5");
}

#[test]
fn solve_linear_with_coeff() {
    // 2 * X = 4  =>  X = 2
    run::stdout_contains("2 * X = 4", "one solution");
    run::stdout_contains("2 * X = 4", "2");
}

#[test]
fn solve_linear_negative_coeff() {
    // -3 * X = 6  =>  X = -2
    run::stdout_contains("-3 * X = 6", "one solution");
    run::stdout_contains("-3 * X = 6", "-2");
}

// --- quadratic equations ---

#[test]
fn solve_x_squared_double_root_zero() {
    // X^2 = 0  =>  X = 0  (double root)
    run::stdout_contains("X^2 = 0", "one solution");
    run::stdout_contains("X^2 = 0", "0");
}

#[test]
fn solve_quadratic_double_root_nonzero() {
    // X^2 - 2 * X + 1 = 0  =>  X = 1  (double root)
    run::stdout_contains("X^2 - 2 * X + 1 = 0", "one solution");
    run::stdout_contains("X^2 - 2 * X + 1 = 0", "1");
}

#[test]
fn solve_quadratic_two_roots() {
    // X^2 - 5 * X + 6 = 0  =>  X = 2 and X = 3
    run::stdout_contains("X^2 - 5 * X + 6 = 0", "two solution");
    run::stdout_contains("X^2 - 5 * X + 6 = 0", "2");
    run::stdout_contains("X^2 - 5 * X + 6 = 0", "3");
}

#[test]
fn solve_quadratic_two_roots_one_negative() {
    // X^2 - X - 6 = 0  =>  X = 3 and X = -2
    run::stdout_contains("X^2 - X - 6 = 0", "two solution");
    run::stdout_contains("X^2 - X - 6 = 0", "3");
    run::stdout_contains("X^2 - X - 6 = 0", "-2");
}

#[test]
fn solve_difference_of_squares() {
    // X^2 - 1 = 0  =>  X = 1 and X = -1
    run::stdout_contains("X^2 - 1 = 0", "two solution");
    run::stdout_contains("X^2 - 1 = 0", "1");
    run::stdout_contains("X^2 - 1 = 0", "-1");
}

#[test]
fn solve_quadratic_no_real_roots() {
    // X^2 + 1 = 0  =>  no real solutions
    run::stdout_contains("X^2 + 1 = 0", "no real");
}

#[test]
fn solve_quadratic_no_real_roots_larger() {
    // X^2 + X + 5 = 0  =>  disc = 1 - 20 < 0
    run::stdout_contains("X^2 + X + 5 = 0", "no real");
}

// --- failure cases (non-zero exit) ---

#[test]
fn solve_fail_degree_too_high() {
    run::fail("X^3 = 0");
}

#[test]
fn solve_fail_invalid_character() {
    run::fail("@ = 0");
}

#[test]
fn solve_fail_division_by_zero() {
    run::fail("X / 0 = 0");
}

#[test]
fn solve_fail_hanging_operator() {
    run::fail("X + = 0");
}
