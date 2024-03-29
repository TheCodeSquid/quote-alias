//! The [`alias!`] macro can be used to assign token streams to convenient identifiers.
//!
//! ```
//! use quote::quote;
//! use quote_alias::alias;
//!
//! alias! {
//!     Foo(my_crate::Foo),
//! }
//!
//! # fn main() {
//! // same as: quote! { my_crate::Foo::new() };
//! let create_foo = quote! { #Foo::new() };
//! # }
//! ```
//!
//! See [`alias!`] for more detailed documentation and usage examples.

#[doc(hidden)]
pub use proc_macro2::TokenStream;
#[doc(hidden)]
pub use quote::{quote, ToTokens};

/// Assigns a token stream to an identifier.
///
/// This is done by generating a unit struct that implements [`ToTokens`].
/// The struct can be then interpolated in [`quote!`] invocations or have its `ToTokens` methods called directly.
///
/// Visibility and doc comments are also passed through to the struct.
///
/// # Usage
/// ```
/// use quote_alias::alias;
/// use quote::quote;
///
/// alias! {
///     /// `Foo` from `my_crate::foo`
///     pub Foo(my_crate::foo::Foo),
///     
///     this_and_that {
///         my_crate::this();
///         my_crate::that();
///     },
/// }
///
/// # fn main() {
/// // same as: quote! { my_crate::foo::Foo::new() };
/// quote! { #Foo::new() };
///
/// // same as: quote! {
/// //     my_crate::this();
/// //     my_crate::that();
/// // };
/// quote! { #this_and_that };
/// # }
/// ```
///
/// [`quote!`]: quote::quote
/// [`ToTokens`]: quote::ToTokens
#[macro_export]
macro_rules! alias {
    ($( $(#[doc = $doc:expr])* $vis:vis $ident:ident $body:tt ),* $(,)?) => {
        $(
            $(#[doc = $doc])*
            $crate::alias!(@struct $vis $ident $body);
        )*
    };

    (@struct $vis:vis $ident:ident { $($tt:tt)* }) => {
        $crate::alias!(@struct $vis $ident ( $($tt)* ));
    };
    (@struct $vis:vis $ident:ident ( $($tt:tt)* )) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug)]
        $vis struct $ident;
        impl $crate::ToTokens for $ident {
            fn to_tokens(&self, tokens: &mut $crate::TokenStream) {
                tokens.extend($crate::quote! { $($tt)* });
            }
        }
    };
}
