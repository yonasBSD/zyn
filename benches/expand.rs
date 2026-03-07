#![feature(test)]

extern crate test;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use test::{Bencher, black_box};
use zyn_core::{Fields, FromInput, Input, Pipe, pipes};

fn make_input() -> TokenStream {
    quote! {
        pub struct UserRecord {
            pub user_id: u64,
            pub first_name: String,
            pub last_name: String,
            pub is_active: bool,
            pub created_at: u64,
        }
    }
}

#[bench]
fn parse_vanilla(b: &mut Bencher) {
    let ts = make_input();
    b.iter(|| syn::parse2::<syn::DeriveInput>(black_box(ts.clone())).unwrap())
}

#[bench]
fn parse_zyn(b: &mut Bencher) {
    let ts = make_input();
    b.iter(|| syn::parse2::<Input>(black_box(ts.clone())).unwrap())
}

#[bench]
fn extract_vanilla(b: &mut Bencher) {
    let ts = make_input();
    let derive: syn::DeriveInput = syn::parse2(ts).unwrap();
    b.iter(|| {
        let syn::Data::Struct(ref s) = black_box(&derive).data else {
            panic!()
        };
        let syn::Fields::Named(ref n) = s.fields else {
            panic!()
        };
        black_box(n.clone())
    })
}

#[bench]
fn extract_zyn(b: &mut Bencher) {
    let ts = make_input();
    let input: Input = syn::parse2(ts).unwrap();
    b.iter(|| black_box(Fields::<syn::FieldsNamed>::from_input(black_box(&input)).unwrap()))
}

#[bench]
fn codegen_vanilla(b: &mut Bencher) {
    let ts = make_input();
    let derive: syn::DeriveInput = syn::parse2(ts).unwrap();
    let syn::Data::Struct(ref data) = derive.data else {
        panic!()
    };
    let syn::Fields::Named(ref named) = data.fields else {
        panic!()
    };
    let named = named.clone();
    b.iter(|| {
        let methods = named.named.iter().map(|f| {
            let fname = f.ident.as_ref().unwrap();
            let getter = format_ident!("get_{}", fname);
            let ty = &f.ty;
            quote! { pub fn #getter(&self) -> &#ty { &self.#fname } }
        });
        black_box(quote! { impl UserRecord { #(#methods)* } })
    })
}

#[bench]
fn codegen_zyn(b: &mut Bencher) {
    let ts = make_input();
    let input: Input = syn::parse2(ts).unwrap();
    let fields = Fields::<syn::FieldsNamed>::from_input(&input).unwrap();
    b.iter(|| {
        let methods = fields.named.iter().map(|f| {
            let fname = f.ident.as_ref().unwrap();
            let getter = pipes::Ident("get_{}").pipe(fname.to_string());
            let ty = &f.ty;
            quote! { pub fn #getter(&self) -> &#ty { &self.#fname } }
        });
        black_box(quote! { impl UserRecord { #(#methods)* } })
    })
}

#[bench]
fn full_vanilla(b: &mut Bencher) {
    let ts = make_input();
    b.iter(|| {
        let derive: syn::DeriveInput = syn::parse2(ts.clone()).unwrap();
        let name = &derive.ident;
        let syn::Data::Struct(ref data) = derive.data else {
            panic!()
        };
        let syn::Fields::Named(ref named) = data.fields else {
            panic!()
        };
        let methods = named.named.iter().map(|f| {
            let fname = f.ident.as_ref().unwrap();
            let getter = format_ident!("get_{}", fname);
            let ty = &f.ty;
            quote! { pub fn #getter(&self) -> &#ty { &self.#fname } }
        });
        black_box(quote! { impl #name { #(#methods)* } })
    })
}

#[bench]
fn full_zyn(b: &mut Bencher) {
    let ts = make_input();
    b.iter(|| {
        let input: Input = syn::parse2(ts.clone()).unwrap();
        let name = input.ident();
        let fields = Fields::<syn::FieldsNamed>::from_input(&input).unwrap();
        let methods = fields.named.iter().map(|f| {
            let fname = f.ident.as_ref().unwrap();
            let getter = pipes::Ident("get_{}").pipe(fname.to_string());
            let ty = &f.ty;
            quote! { pub fn #getter(&self) -> &#ty { &self.#fname } }
        });
        black_box(quote! { impl #name { #(#methods)* } })
    })
}
