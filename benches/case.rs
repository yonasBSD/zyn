#![feature(test)]

extern crate test;

use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
use test::{Bencher, black_box};

const INPUT: &str = "first_name_field";

#[bench]
fn snake_heck(b: &mut Bencher) {
    b.iter(|| black_box(INPUT).to_snake_case())
}

#[bench]
fn snake_zyn(b: &mut Bencher) {
    b.iter(|| zyn_core::case::to_snake(black_box(INPUT)))
}

#[bench]
fn pascal_heck(b: &mut Bencher) {
    b.iter(|| black_box(INPUT).to_pascal_case())
}

#[bench]
fn pascal_zyn(b: &mut Bencher) {
    b.iter(|| zyn_core::case::to_pascal(black_box(INPUT)))
}

#[bench]
fn camel_heck(b: &mut Bencher) {
    b.iter(|| black_box(INPUT).to_lower_camel_case())
}

#[bench]
fn camel_zyn(b: &mut Bencher) {
    b.iter(|| zyn_core::case::to_camel(black_box(INPUT)))
}

#[bench]
fn screaming_heck(b: &mut Bencher) {
    b.iter(|| black_box(INPUT).to_shouty_snake_case())
}

#[bench]
fn screaming_zyn(b: &mut Bencher) {
    b.iter(|| zyn_core::case::to_screaming(black_box(INPUT)))
}

#[bench]
fn kebab_heck(b: &mut Bencher) {
    b.iter(|| black_box(INPUT).to_kebab_case())
}

#[bench]
fn kebab_zyn(b: &mut Bencher) {
    b.iter(|| zyn_core::case::to_kebab(black_box(INPUT)))
}
