use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Type, TypeArray};

pub(crate) fn impl_macro(ast: &DeriveInput) -> impl Into<TokenStream> {
    if !ast.generics.params.is_empty() {
        panic!("generics are not supported in uniform attribute")
    }

    let vis = &ast.vis;
    let name = &ast.ident;
    let ds = match &ast.data {
        Data::Struct(ds) => ds,
        _ => panic!("uniform attribute applied only with struct"),
    };

    let mut field_name: Vec<_> = ds.fields.iter().map(|field| &field.ident).collect();
    let field_name = &mut field_name;

    let field_type = ds.fields.iter().map(|field| &field.ty);
    let function = ds.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let set_name = format_ident!("set_{}", field_name);

        match &field.ty {
            Type::Array(TypeArray { elem, .. }) => quote! {
                fn #set_name(&self, val: &[#elem]) {
                    self.program.uniform_slice(self.#field_name, val);
                }
            },
            field_type => quote! {
                fn #set_name(&self, val: &#field_type) {
                    self.program.uniform(self.#field_name, val);
                }
            },
        }
    });

    quote! {
        #vis struct #name {
            program: crate::program::Program,
            #( #field_name: crate::program::Location<#field_type> ),*
        }

        impl #name {
            fn new(program: Program) -> Self {
                let _loc = program.locations();
                #( let #field_name = _loc.get(stringify!(#field_name)).unwrap(); )*

                Self {
                    program,
                    #( #field_name ),*
                }
            }

            fn use_program(&self) {
                self.program.use_program()
            }

            #( #function )*
        }
    }
}
