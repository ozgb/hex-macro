use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Decodes a hex string into a byte array at compile time
///
/// # Examples
///
/// ```rust
/// use hex_macro::hex;
/// const DATA: [u8; 11] = hex!("48656c6c6f20776f726c64");
/// assert_eq!(DATA, *b"Hello world");
/// ```
///
/// ```rust
/// use hex_macro::hex;
/// // With a '0x' prefix
/// const DATA: [u8; 11] = hex!("0x48656c6c6f20776f726c64");
/// assert_eq!(DATA, *b"Hello world");
/// ```
///
/// ```compile_fail
/// use hex_macro::hex;
/// const DATA: [u8; 10] = hex!("48656c6c6f20776f726c64"); // Wrong length byte array
/// ```
///
/// ```compile_fail
/// hex!("notvalidhexstring!"); // not a valid hex string
/// ```
#[proc_macro]
pub fn hex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();

    // Trim '0x' from start
    let input = input.trim_start_matches("0x");

    match hex::decode(input) {
        Ok(decoded) => {
            let len = decoded.len();
            let byte_array = quote! {
                {
                    const DATA: [u8; #len] = [#(#decoded),*];
                    DATA
                }
            };
            byte_array.into()
        }
        Err(_) => panic!("Invalid hex string"),
    }
}
