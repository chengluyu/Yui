use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::{Error, GenericArgument, Lit, Path, PathArguments, PathSegment, Type, TypePath};

#[inline]
pub fn unwrap_punctuated_first<T, P>(
    punctuated: &Punctuated<T, P>,
    error: Error,
) -> Result<&T, Error> {
    match punctuated.first() {
        Some(s) => Ok(s),
        None => Err(error),
    }
}

#[inline]
pub fn unwrap_punctuated_last<T, P>(
    punctuated: &Punctuated<T, P>,
    error: Error,
) -> Result<&T, Error> {
    match punctuated.last() {
        Some(s) => Ok(s),
        None => Err(error),
    }
}

#[inline]
pub fn get_nested_type<'a>(
    segment: &'a PathSegment,
    message: &'static str,
) -> Result<&'a Type, Error> {
    let error = Error::new_spanned(segment, message);
    match &segment.arguments {
        PathArguments::AngleBracketed(argument) => {
            match unwrap_punctuated_first(&argument.args, error.clone())? {
                GenericArgument::Type(nested_type) => Ok(nested_type),
                _ => Err(error.clone()),
            }
        }
        _ => Err(error.clone()),
    }
}

#[inline]
pub fn unwrap_type_path<'a>(ty: &'a Type, message: &'static str) -> Result<&'a TypePath, Error> {
    match ty {
        Type::Path(type_path) => Ok(type_path),
        _ => Err(Error::new_spanned(ty, message)),
    }
}

#[inline]
pub fn get_lit_str(lit: &Lit, path: &Path) -> Result<String, Error> {
    match lit {
        Lit::Str(lit_str) => Ok(lit_str.value()),
        _ => Err(Error::new_spanned(
            lit,
            format!(
                "expected {} attribute to be a string",
                path.get_ident().unwrap()
            ),
        )),
    }
}

pub fn get_lit_as_string(lit: &Lit, path: &Path) -> Result<String, Error> {
    match lit {
        Lit::Str(lit_str) => Ok(lit_str.value()),
        Lit::Int(lit_int) => Ok(lit_int.to_string()),
        Lit::Float(lit_float) => Ok(lit_float.to_string()),
        Lit::Bool(lit_bool) => Ok(lit_bool.value.to_string()),
        _ => Err(Error::new_spanned(
            lit,
            format!(
                "expected {} attribute to be a string",
                path.get_ident().unwrap()
            ),
        )),
    }
}

#[inline]
pub fn get_lit_int<T: FromStr>(lit: &Lit, path: &Path) -> Result<T, Error>
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    match lit {
        Lit::Int(lit_int) => Ok(lit_int.base10_parse().unwrap()),
        _ => Err(Error::new_spanned(
            lit,
            format!(
                "expected {} attribute to be a integer",
                path.get_ident().unwrap()
            ),
        )),
    }
}

#[inline]
pub fn get_lit_float<T: FromStr>(lit: &Lit, path: &Path) -> Result<T, Error>
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    match lit {
        Lit::Float(lit_float) => Ok(lit_float.base10_parse().unwrap()),
        _ => Err(Error::new_spanned(
            lit,
            format!(
                "expected {} attribute to be a float",
                path.get_ident().unwrap()
            ),
        )),
    }
}

#[inline]
pub fn get_lit_bool(lit: &Lit, path: &Path) -> Result<bool, Error> {
    match lit {
        Lit::Bool(lit_bool) => Ok(lit_bool.value),
        _ => Err(Error::new_spanned(
            lit,
            format!(
                "expected {} attribute to be a bool",
                path.get_ident().unwrap()
            ),
        )),
    }
}
