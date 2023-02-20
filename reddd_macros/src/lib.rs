use darling::{FromDeriveInput, ToTokens};

mod entity;
mod util;
mod value_type;

macro_rules! define_proc_macro {
    ($trait:ident #[ $($attr:ident),* ]) => {
        paste::paste! {
            #[proc_macro_error::proc_macro_error]
            #[proc_macro_derive($trait, attributes($($attr),*))]
            pub fn [<derive $trait:snake>](
                input: proc_macro::TokenStream,
            ) -> proc_macro::TokenStream {
                let input = syn::parse_macro_input!(input as syn::DeriveInput);

                match [<$trait:snake>]::$trait::from_derive_input(&input) {
                    | Ok(tokens) => tokens,
                    | Err(err) => panic!("{err:?}"),
                }
                .into_token_stream()
                .into()
            }
        }
    };
}

define_proc_macro!(ValueType #[ main_field ]);
define_proc_macro!(Entity #[ id_field, created_at_field ]);
