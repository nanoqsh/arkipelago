use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn impl_macro(ast: &DeriveInput) -> impl Into<TokenStream> {
    if !ast.generics.params.is_empty() {
        panic!("generics are not supported in derive Layout")
    }

    let name = &ast.ident;
    let ds = match &ast.data {
        Data::Struct(ds) => ds,
        _ => panic!("derive Layout applied only with struct"),
    };

    let field_name = ds.fields.iter().map(|field| &field.ident);
    let field_type = ds.fields.iter().map(|field| &field.ty);

    quote! {
        impl crate::layout::Layout for #name {
            fn layout(fs: &mut Vec<crate::layout::Field>) {
                use crate::{attribute::Component, declare::Declare};

                fs.extend([
                    #( crate::layout::Field {
                        name: stringify!(#field_name),
                        declare: <#field_type as Declare>::declare_type,
                        offset: memoffset::offset_of!(#name, #field_name) as u32,
                        components: <#field_type as Component>::COMPONENTS,
                        element_type: <#field_type as Component>::ELEMENT_TYPE,
                    } ),*
                ])
            }
        }
    }
}
