/*--------------------------------------------------------------------------

black

The MIT License (MIT)

Copyright (c) 2019 Haydn Paterson (sinclair) <haydn.developer@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

---------------------------------------------------------------------------*/

use super::metadata::StructMeta;

#[allow(dead_code)]
fn impl_new_function(s: &StructMeta) -> quote::Tokens {
    let initializers = s.fields.iter().map(|field| match field.kind.as_ref() {
        "Vec4" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: Vec4::new(0.0, 0.0, 0.0, 0.0)
            }
        }
        "Vec3" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: Vec3::new(0.0, 0.0, 0.0)
            }
        }
        "Vec2" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: Vec2::new(0.0, 0.0)
            }
        }
        "f32" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: 0.0
            }
        }
        _ => panic!(format!("Cannot interpolate type '{}'", field.kind)),
    });
    let name = quote::Ident::from(s.name.clone());
    quote!(
        #[inline(always)]
        fn new() -> #name {
            #name {
                #( #initializers ),*
            }
        }
    )
}

#[allow(dead_code)]
fn impl_correct_function(s: &StructMeta) -> quote::Tokens {
    let initializers = s.fields.iter().map(|field| match field.kind.as_ref() {
        "Vec4" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: Vec4::new(
                    v.#field_name.x / w,
                    v.#field_name.y / w,
                    v.#field_name.z / w,
                    v.#field_name.w / w,
                )
            }
        }
        "Vec3" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: Vec3::new(
                    v.#field_name.x / w,
                    v.#field_name.y / w,
                    v.#field_name.z / w,
                )
            }
        }
        "Vec2" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: Vec2::new(
                    v.#field_name.x / w,
                    v.#field_name.y / w,
                )
            }
        },
        "f32" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: v.#field_name / w
            }
        }
        _ => panic!(format!("Cannot interpolate type '{}'", field.kind)),
    });
    let name = quote::Ident::from(s.name.clone());
    quote!(
        #[inline(always)]
        fn correct(v: &#name, w: &f32)  -> #name {
            #name {
                #( #initializers ),*
            }
        }
    )
}

#[allow(dead_code)]
fn impl_interpolate_function(s: &StructMeta) -> quote::Tokens {
    let initializers = s.fields.iter().map(|field| {
        match field.kind.as_ref() {
            "Vec4" => {
                let field_name = quote::Ident::from(field.name.clone());
                quote! {
                    #field_name: Vec4::new(
                        ((w0 * v0.#field_name.x) + (w1 * v1.#field_name.x) + (w2 * v2.#field_name.x)) / w,
                        ((w0 * v0.#field_name.y) + (w1 * v1.#field_name.y) + (w2 * v2.#field_name.y)) / w,
                        ((w0 * v0.#field_name.z) + (w1 * v1.#field_name.z) + (w2 * v2.#field_name.z)) / w,
                        ((w0 * v0.#field_name.w) + (w1 * v1.#field_name.w) + (w2 * v2.#field_name.w)) / w,
                    )
                }
            },
            "Vec3" => {
                let field_name = quote::Ident::from(field.name.clone());
                quote! {
                    #field_name: Vec3::new(
                        ((w0 * v0.#field_name.x) + (w1 * v1.#field_name.x) + (w2 * v2.#field_name.x)) / w,
                        ((w0 * v0.#field_name.y) + (w1 * v1.#field_name.y) + (w2 * v2.#field_name.y)) / w,
                        ((w0 * v0.#field_name.z) + (w1 * v1.#field_name.z) + (w2 * v2.#field_name.z)) / w,
                    )
                }
            },
            "Vec2" => {
                let field_name = quote::Ident::from(field.name.clone());
                quote! {
                    #field_name: Vec2::new(
                        ((w0 * v0.#field_name.x) + (w1 * v1.#field_name.x) + (w2 * v2.#field_name.x)) / w,
                        ((w0 * v0.#field_name.y) + (w1 * v1.#field_name.y) + (w2 * v2.#field_name.y)) / w,
                    )
                }
            },
            "f32" => {
                let field_name = quote::Ident::from(field.name.clone());
                quote! {
                    #field_name: ((w0 * v0.#field_name) + (w1 * v1.#field_name) + (w2 * v2.#field_name)) / w
                }
            },
            _ => panic!(format!("Cannot interpolate type '{}'", field.kind))
        }
    });
    let name = quote::Ident::from(s.name.clone());
    quote!(
        #[inline(always)]
        fn interpolate(v0: &#name, v1: &#name, v2: &#name, w0: &f32, w1: &f32, w2: &f32, w:  &f32) -> #name {
            #name {
                #( #initializers ),*
            }
        }
    )
}

#[allow(dead_code)]
pub fn impl_interpolate(s: &StructMeta) -> quote::Tokens {
    let kind = quote::Ident::from(s.name.clone());
    let new_function = impl_new_function(&s);
    let correct_function = impl_correct_function(&s);
    let interpolate_function = impl_interpolate_function(&s);
    let implementation = quote! {
        impl Interpolate for #kind {
            #new_function
            #correct_function
            #interpolate_function
        }
    };
    // println!("{}", implementation);
    implementation
}
