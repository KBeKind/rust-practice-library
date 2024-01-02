
use rust_proj_5::colors;

// TEST ANNOTATION NEEDED FOR EACH TEST
#[test]
// COLORS TESTS
fn test_red_coloring(){
    let red: String = colors::red("Red color string");

    let correct_color_string: String = format!("\x1b[31m{}\x1b[0m", "Red color string");
    //let wrong_color_string: String = "Red color string".to_string();
    assert_eq!(correct_color_string, red);
}


// SAMPLE TESTS
#[test]
fn test_simple() {
    assert!(true);
}

#[test]
fn test_simple2() {
    let false_boolean = false;
    assert!(false_boolean != true);
}
