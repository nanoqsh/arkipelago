use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

pub(crate) fn impl_macro(ast: &DeriveInput) -> impl Into<TokenStream> {
    if !ast.generics.params.is_empty() {
        panic!("generics are not supported in shader_set attribute")
    }

    let vis = &ast.vis;
    let name = &ast.ident;
    let ds = match &ast.data {
        Data::Struct(ds) => ds,
        _ => panic!("shader_set attribute applied only with struct"),
    };

    let variant_name = ds.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        format_ident!("Use_{}", field_name)
    });

    let mut field_name: Vec<_> = ds.fields.iter().map(|field| &field.ident).collect();
    let field_name = &mut field_name;
    let mut field_type: Vec<_> = ds.fields.iter().map(|field| &field.ty).collect();
    let field_type = &mut field_type;

    let function = ds.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let use_name = format_ident!("use_{}", field_name);
        let variant_name = format_ident!("Use_{}", field_name);

        quote! {
            pub fn #use_name(&mut self) {
                match self.using {
                    Using::#variant_name => (),
                    _ => {
                        self.#field_name.use_program();
                        self.using = Using::#variant_name;
                    }
                }
            }
        }
    });

    quote! {
        #[allow(non_camel_case_types)]
        enum Using {
            None,
            #( #variant_name ),*
        }

        #vis struct #name {
            using: Using,
            #( #field_name: #field_type ),*
        }

        impl #name {
            fn make(
                #( #field_name: #field_type ),*
            ) -> Self {
                Self {
                    using: Using::None,
                    #( #field_name ),*
                }
            }

            #( #function )*
        }
    }
}
