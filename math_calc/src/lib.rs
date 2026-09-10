// based on section 1.6 in the No Bullshit Guide to Math & Physics
// this will be the main source for all my mathy shit in this library
// because its fun...
pub fn completing_the_square(a: i32, b: i32, c: i32) -> (f32, f32) {
    let split = (b as f32) / (2.0 * a as f32);
    let constant = (c as f32 / a as f32) - split.powf(2.0);
    let constant = -constant;
    let constant = constant.sqrt();

    (-split + constant, -split - constant)
}

// Based on section 1.7 of NBG2M&P
pub fn solve_quadratic(a: f32, b: f32, c: f32) -> (f32, f32) {
    let discriminant = b.powf(2.0) - 4.0 * a * c; 
    let res1 = (-b + (discriminant.sqrt())) / (2.0 * a);
    let res2 = (-b - (discriminant.sqrt())) / (2.0 * a);

    (res1, res2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_a_3_and_minus5() {
        let (res1, res2) = completing_the_square(1, 2, -15);
        assert_eq!(res1, 3.0);
        assert_eq!(res2, -5.0);
    }

    #[test]
    fn it_should_return_a_neg2minsqrt3_and_neg2plussqrt3() {
        let (res1, res2) = completing_the_square(1, 4, 1);

        let exp1: f32= 3.0;
        let exp2: f32= 3.0;
        assert_eq!(res1, -2.0 + exp1.sqrt());
        assert_eq!(res2, -2.0 - exp2.sqrt());
    }

    #[test]
    fn it_testing_the_quadratic_solver() {
        let (res1, res2) = solve_quadratic(2.0, -1.0, -3.0);

        assert_eq!(res1, 1.5);
        assert_eq!(res2, -1.0);
    }

    #[test]
    fn it_testing_the_quadratic_solver_2() {
        let (res1, res2) = solve_quadratic(1.0, -4.0, 4.0);

        assert_eq!(res1, 2.0);
        assert_eq!(res2, 2.0);
    }
}
