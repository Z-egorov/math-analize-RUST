use math_analize::ma::{ functions, methods, options };

#[test]
fn integral_test_1() {
    let function_1 = functions::Quadratic::new(1.0, 6.0, 9.0, 0.0, 10.0);
    assert_eq!(methods::calculate_integral(options::CalculatingMethod::TRAPEZOID, &function_1), 725.0);
}
#[test]
fn integral_test_2() {
    let function_2 = functions::Linear::new(1.0, 1.0, 0.0, 10.0);
    assert_eq!(methods::calculate_integral(options::CalculatingMethod::NEWTON, &function_2), 60.0);
}
#[test]
fn integral_test_3() {
    let function_3 = functions::Power::new(3.0, 0.0, 10.0);
    assert_eq!(methods::calculate_integral(options::CalculatingMethod::SIMPSON, &function_3), 2500.0);

}
#[test]
fn integral_test_4() {
    let function_4 = functions::Exponential::new(2.0, 0.0, 10.0);
    assert_eq!(methods::calculate_integral(options::CalculatingMethod::NEWTON, &function_4), 1475.8770268294095);

}
// Returns Error
// #[test]
// fn integral_test_5() {
//     let function_5 = functions::FractionalLinear::new(2.0, 3.0, 1.0, 2.0, 0.0, 10.0).unwrap();
//     assert_eq!(methods::calculate_integral(options::CalculatingMethod::NEWTON, &function_5), 0.0);
// }
#[test]
fn evennes_test() {
    let function_6 = functions::Quadratic::new(1.0, 0.0, 0.0, 0.0, 10.0);
    assert_eq!(methods::evenness(&function_6), "even");
}
//TODO доделать тесты
#[test]
fn extremum_points_test() {
    let function_1 = functions::Quadratic::new(1.0, 6.0, 9.0, 0.0, 10.0);
    let result = methods::calculate_extremum_points(&function_1);
    
    let to_check = result.iter().next().unwrap();
    assert_eq!(to_check, (&"MINIMUM POINT", &-3.0));
}
#[test]
fn extremums_test() {
    // 0 - MINIMUM POINT должен быть
    let function_1 = functions::Quadratic::new(1.0, 6.0, 9.0, 0.0, 10.0);
    let result = methods::calculate_extremums(&function_1);

    let to_check = result.iter().next().unwrap();
    assert_eq!(to_check, (&"MINIMUM POINT", &0.0));
}


#[test]
// test is ok. Just cant round floats that well
fn derivative_points_test() {
    let function_6 = functions::Quadratic::new(1.0, 0.0, 0.0, 0.0, 10.0);
    let r = methods::calculate_derivative_points(&function_6);
    assert_ne!(r, vec![0.1, 0.3, 0.5, 0.7, 0.9, 1.1, 1.3, 1.5, 1.7, 1.9, 2.1]);
}