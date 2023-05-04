use darling::{FromDeriveInput, ToTokens};
use proc_macro::TokenStream;

mod entity;
mod usecase;
mod util;
mod value_type;

macro_rules! parse_derive_input {
    ($input:ident => $parser:ty) => {{
        let input = syn::parse_macro_input!($input as syn::DeriveInput);

        let ts: TokenStream = match <$parser>::from_derive_input(&input) {
            | Ok(tokens) => tokens,
            | Err(err) => return err.write_errors().into(),
        }
        .into_token_stream()
        .into();

        ts
    }};
}

macro_rules! define_proc_macro {
    ($trait:ident with $impl:ty [ $($attr:ident),* ]) => {
        paste::paste! {
            #[proc_macro_error::proc_macro_error]
            #[proc_macro_derive($trait, attributes($($attr),*))]
            pub fn [<derive_ $trait:snake>](input: TokenStream) -> TokenStream {
                parse_derive_input!(input => $impl).into()
            }
        }
    };

    ($trait:ident [ $($attr:ident),* ]) => {
        paste::paste! {
            define_proc_macro!($trait with [<$trait:snake>]::$trait [ $($attr),* ]);
        }
    };
}

macro_rules! concat_token_streams {
    ($($ts:expr),+) => {{
        let mut ts = TokenStream::new();

        $(ts.extend($ts);)*

        ts
    }}
}

define_proc_macro!(ValueType[main_field]);
define_proc_macro!(Entity [id_field, created_at_field]);
define_proc_macro!(UseCase with usecase::UseCase [usecase]);

#[proc_macro_derive(
    MutableEntity,
    attributes(id_field, created_at_field, updated_at_field)
)]
#[proc_macro_error::proc_macro_error]
pub fn derive_mutable_entity(input: TokenStream) -> TokenStream {
    concat_token_streams!(
        derive_entity(input.clone()),
        parse_derive_input!(input => entity::MutableEntity)
    )
}
