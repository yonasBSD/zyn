use zyn::syn;

#[derive(zyn::Attribute)]
enum Mode {
    Fast,
    Slow,
    Custom { speed: i64 },
}

#[test]
fn unit_variant_from_flag() {
    let arg: zyn::meta::Arg = syn::parse_str("fast").unwrap();
    assert!(matches!(Mode::from_arg(&arg).unwrap(), Mode::Fast));
}

#[test]
fn unit_variant_slow_from_flag() {
    let arg: zyn::meta::Arg = syn::parse_str("slow").unwrap();
    assert!(matches!(Mode::from_arg(&arg).unwrap(), Mode::Slow));
}

#[test]
fn struct_variant_from_list() {
    let arg: zyn::meta::Arg = syn::parse_str("custom(speed = 42)").unwrap();
    let v = Mode::from_arg(&arg).unwrap();
    assert!(matches!(v, Mode::Custom { speed: 42 }));
}

#[test]
fn unknown_variant_is_err() {
    let arg: zyn::meta::Arg = syn::parse_str("turbo").unwrap();
    assert!(Mode::from_arg(&arg).is_err());
}

#[test]
fn wrong_arg_shape_is_err() {
    let arg: zyn::meta::Arg = syn::parse_str("x = 1").unwrap();
    assert!(Mode::from_arg(&arg).is_err());
}

#[derive(zyn::Attribute)]
enum Color {
    Red,
    Named(String),
    Rgb(u8, u8, u8),
}

#[test]
fn single_field_tuple_from_expr() {
    let arg: zyn::meta::Arg = syn::parse_str("named = \"blue\"").unwrap();
    let v = Color::from_arg(&arg).unwrap();
    assert!(matches!(v, Color::Named(s) if s == "blue"));
}

#[test]
fn multi_field_tuple_from_list() {
    let arg: zyn::meta::Arg = syn::parse_str("rgb(255, 128, 0)").unwrap();
    let v = Color::from_arg(&arg).unwrap();
    assert!(matches!(v, Color::Rgb(255, 128, 0)));
}

#[test]
fn unit_color_from_flag() {
    let arg: zyn::meta::Arg = syn::parse_str("red").unwrap();
    assert!(matches!(Color::from_arg(&arg).unwrap(), Color::Red));
}

#[derive(zyn::Attribute)]
#[zyn("config")]
struct Config {
    mode: Mode,
}

#[test]
fn enum_as_field_in_attribute_struct() {
    let args: zyn::meta::Args = syn::parse_str("mode(fast)").unwrap();
    let config = Config::from_args(&args).unwrap();
    assert!(matches!(config.mode, Mode::Fast));
}

#[test]
fn no_from_input_on_enum() {
    fn _assert_no_from_input<T: ::zyn::FromArg>() {}
    _assert_no_from_input::<Mode>();
}
