extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet as Set;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, ItemFn, Token};

struct ArgsHoldingIdents {
    idents: Set<Ident>,
}

impl Parse for ArgsHoldingIdents {
    fn parse(args: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(args)?;
        Ok(ArgsHoldingIdents {
            idents: vars.into_iter().collect(),
        })
    }
}

/// This macro is used to generate the implementation of the MessageName trait for a struct.
/// ```rust
/// #[message_name("A1Request")]
/// ```

#[proc_macro_attribute]
pub fn message_name(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_name: syn::LitStr = syn::parse(attr).unwrap();
    let ast = syn::parse(item).unwrap();

    let gen = impl_message_name(&ast, &attr_name);

    gen
}

fn impl_message_name(ast: &syn::DeriveInput, attr_name: &syn::LitStr) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MessageName for #name {
            fn message_name(&self) -> &'static str {
                #attr_name
            }
        }
    };
    gen.into()
}

/// Macro for implementing a handler for a request.
/// Should be use like this:
/// ```rust
/// request_handler!(A1RequestHandler, (message: A1Request) => {
///    // Do something with the message
///  // Return a response
/// });
/// ```
///
/// The macro will generate the following code:
/// ```rust
/// #[message_handler("A1Request")]
/// pub struct A1RequestHandler {}
/// pub struct A1RequestHandler {}
///
/// #[async_trait]
/// impl RequestHandler for A1RequestHandler {
///    async fn handle_request(&self, _request: Arc<dyn Request>) -> Arc<dyn Response> {
///       // Do something with the message
///      // Return a response
///   }
/// }
/// ```

#[proc_macro]
pub fn request_handler(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemFn);
    let name = &ast.sig.ident;
    let args = &ast.sig.inputs;
    let body = &ast.block;
    let gen = quote! {
        #[message_handler(#name)]
        pub struct #name {}
        pub struct #name {}

        #[async_trait]
        impl RequestHandler for #name {
            async fn handle_request(&self, _request: Arc<dyn Request>) -> Arc<dyn Response> {
                #body
            }
        }
    };
    gen.into()
}
