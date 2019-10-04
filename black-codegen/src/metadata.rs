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

#[derive(Debug)]
pub struct StructMeta {
    pub name:   String,
    pub fields: Vec<Field>
}
#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub kind: String,
}

/// Reads through the derive AST tree and returns the structures field name and type information.
pub fn read_struct_metadata(ast: &syn::DeriveInput) -> StructMeta {
    let mut result = StructMeta {
        name:   ast.ident.clone().to_string(),
        fields: vec![]
    };
    if let syn::Body::Struct(ref _struct) = ast.body {
        match _struct {
            syn::VariantData::Struct(fields) => {
                for field in fields {
                    let name = field.ident.clone().unwrap().to_string();
                    match field.ty {
                        syn::Ty::Path(ref _opt, ref path) => {
                            let kind = path.segments[0].ident.to_string();
                            result.fields.push({
                                Field { name, kind }
                            });
                        },
                        _ => panic!(format!("Unable to read field '{}'", name))
                    }
                }
            },
            _ => panic!("Interpolate is only defined for structs"),
        };
    };
    result
}
