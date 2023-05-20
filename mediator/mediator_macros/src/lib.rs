use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemStruct};

/// Implement the trait MessageName for the given struct.
/// Example:
/// #[message(message_name)]
#[proc_macro_attribute]
pub fn message(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = input_struct.ident.clone();

    // Parse the name of the message
    let input_args = parse_macro_input!(args as Ident);
    let message_name = input_args.to_string();

    // Create the `message_name` method.
    let message_name_impl = quote! {
        fn message_name(&self) -> &'static str {
            #message_name
        }
    };

    // Create the `as_any` method.
    let as_any_impl = quote! {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    };

    // Implement the trait for the struct.
    let impl_block = quote! {
        impl MessageName for #struct_name {
            #message_name_impl
            #as_any_impl
        }
    };

    let output = quote! {
        #input_struct
        #impl_block
    };

    output.into()
}
