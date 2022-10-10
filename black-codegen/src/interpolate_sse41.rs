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

// --------------------------------------------------------------------------
//
// This is a explicit implementation of SSE intrinsics.
//
// Note: Rust will auto-vectorize for SSE on its own so this implementation 
// is fairly redundant. Would be open to insights in how best to leverage
// AVX to help accellerate some of this and other code in the rasterizer.
//
// Some optimizations are possible with _mm_dp_ps which is available in
// sse4.1. To get Rust inlining these intrinsics, run the compiler with.
//  
// $ RUSTFLAGS="-C target-feature=+sse4.1"
// $ cargo run --release
//
// ------------------------------------------------------------------------

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
        _ => panic!("Cannot interpolate type '{}'", field.kind),
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
                #field_name: unsafe {
                    let d0 = _mm_set_ps1(*w);
                    let d1 = _mm_set_ps(
                        v.#field_name.x,
                        v.#field_name.y,
                        v.#field_name.z,
                        v.#field_name.w
                    );
                    let d2 = _mm_div_ps(d1, d0);
                    let re:[f32; 4] = std::mem::transmute(d2);
                    Vec4::new(re[3], re[2], re[1], re[0])
                }
            }
        }
        "Vec3" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: unsafe {
                    let d0 = _mm_set_ps1(*w);
                    let d1 = _mm_set_ps(
                        v.#field_name.x,
                        v.#field_name.y,
                        v.#field_name.z,
                        0.0
                    );
                    let d2 = _mm_div_ps(d1, d0);
                    let re:[f32; 4] = std::mem::transmute(d2);
                    Vec3::new(re[3], re[2], re[1])
                }
            }
        }
        "Vec2" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: unsafe {
                    let d0 = _mm_set_ps1(*w);
                    let d1 = _mm_set_ps(
                        v.#field_name.x,
                        v.#field_name.y,
                        0.0,
                        0.0
                    );
                    let d2 = _mm_div_ps(d1, d0);
                    let re:[f32; 4] = std::mem::transmute(d2);
                    Vec2::new(re[3], re[2])
                }
            }
        }
        _ => panic!("Cannot interpolate type '{}'", field.kind),
    });
    let name = quote::Ident::from(s.name.clone());
    quote!(
        #[inline(always)]
        fn correct(v: &#name, w: &f32)  -> #name {
            #[cfg(target_arch = "x86")]
            use std::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64::*;
            
            #name {
                #( #initializers ),*
            }
        }
    )
}

#[allow(dead_code)]
fn impl_interpolate_function(s: &StructMeta) -> quote::Tokens {
    let initializers = s.fields.iter().map(|field| match field.kind.as_ref() {
        "Vec4" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: unsafe {
                    let r0 = _mm_set_ps(*w0, *w1, *w2, 0.0);
                    let r1 = _mm_set_ps(v0.#field_name.x, v1.#field_name.x, v2.#field_name.x, 0.0);
                    let r2 = _mm_set_ps(v0.#field_name.y, v1.#field_name.y, v2.#field_name.y, 0.0);
                    let r3 = _mm_set_ps(v0.#field_name.z, v1.#field_name.z, v2.#field_name.z, 0.0);
                    let r4 = _mm_set_ps(v0.#field_name.w, v1.#field_name.w, v2.#field_name.w, 0.0);

                    let m0 = _mm_mul_ps(r0, r1);
                    let m0 = _mm_hadd_ps(m0, m0);
                    let m0 = _mm_hadd_ps(m0, m0);

                    let m1 = _mm_mul_ps(r0, r2);
                    let m1 = _mm_hadd_ps(m1, m1);
                    let m1 = _mm_hadd_ps(m1, m1);

                    let m2 = _mm_mul_ps(r0, r3);
                    let m2 = _mm_hadd_ps(m2, m2);
                    let m2 = _mm_hadd_ps(m2, m2);

                    let m3 = _mm_mul_ps(r0, r4);
                    let m3 = _mm_hadd_ps(m3, m3);
                    let m3 = _mm_hadd_ps(m3, m3);

                    let u0 = _mm_cvtss_f32(m0);
                    let u1 = _mm_cvtss_f32(m1);
                    let u2 = _mm_cvtss_f32(m2);
                    let u3 = _mm_cvtss_f32(m3);

                    let d0 = _mm_set_ps1(*w);
                    let d1 = _mm_set_ps(u0, u1, u2, u3);
                    let d2 = _mm_div_ps(d1, d0);

                    let re:[f32; 4] = std::mem::transmute(d2);
                    Vec4::new(re[3], re[2], re[1], re[0])
                }
            }
        }
        "Vec3" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: unsafe {
                    let r0 = _mm_set_ps(*w0, *w1, *w2, 0.0);
                    let r1 = _mm_set_ps(v0.#field_name.x, v1.#field_name.x, v2.#field_name.x, 0.0);
                    let r2 = _mm_set_ps(v0.#field_name.y, v1.#field_name.y, v2.#field_name.y, 0.0);
                    let r3 = _mm_set_ps(v0.#field_name.z, v1.#field_name.z, v2.#field_name.z, 0.0);

                    let m0 = _mm_mul_ps(r0, r1);
                    let m0 = _mm_hadd_ps(m0, m0);
                    let m0 = _mm_hadd_ps(m0, m0);

                    let m1 = _mm_mul_ps(r0, r2);
                    let m1 = _mm_hadd_ps(m1, m1);
                    let m1 = _mm_hadd_ps(m1, m1);

                    let m2 = _mm_mul_ps(r0, r3);
                    let m2 = _mm_hadd_ps(m2, m2);
                    let m2 = _mm_hadd_ps(m2, m2);

                    let u0 = _mm_cvtss_f32(m0);
                    let u1 = _mm_cvtss_f32(m1);
                    let u2 = _mm_cvtss_f32(m2);

                    let d0 = _mm_set_ps1(*w);
                    let d1 = _mm_set_ps(u0, u1, u2, 0.0);
                    let d2 = _mm_div_ps(d1, d0);

                    let re:[f32; 4] = std::mem::transmute(d2);
                    Vec3::new(re[3], re[2], re[1])
                }
            }
        }
        "Vec2" => {
            let field_name = quote::Ident::from(field.name.clone());
            quote! {
                #field_name: unsafe {
                    let r0 = _mm_set_ps(*w0, *w1, *w2, 0.0);
                    let r1 = _mm_set_ps(v0.#field_name.x, v1.#field_name.x, v2.#field_name.x, 0.0);
                    let r2 = _mm_set_ps(v0.#field_name.y, v1.#field_name.y, v2.#field_name.y, 0.0);

                    let m0 = _mm_mul_ps(r0, r1);
                    let m0 = _mm_hadd_ps(m0, m0);
                    let m0 = _mm_hadd_ps(m0, m0);

                    let m1 = _mm_mul_ps(r0, r2);
                    let m1 = _mm_hadd_ps(m1, m1);
                    let m1 = _mm_hadd_ps(m1, m1);

                    let u0 = _mm_cvtss_f32(m0);
                    let u1 = _mm_cvtss_f32(m1);

                    let d0 = _mm_set_ps1(*w);
                    let d1 = _mm_set_ps(u0, u1, 0.0, 0.0);
                    let d2 = _mm_div_ps(d1, d0);

                    let re:[f32; 4] = std::mem::transmute(d2);
                    Vec2::new(re[3], re[2])
                }
            }
        }
        _ => panic!("Cannot interpolate type '{}'", field.kind),
    });
    let name = quote::Ident::from(s.name.clone());
    quote!(
        #[inline(always)]
        fn interpolate(v0: &#name, v1: &#name, v2: &#name, w0: &f32, w1: &f32, w2: &f32, w:  &f32) -> #name {
            #[cfg(target_arch = "x86")]
            use std::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64::*;

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
