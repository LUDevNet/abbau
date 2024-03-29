use std::collections::BTreeMap;

use abbau_types::{Attribute, Repeat, StringOrArray};
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::Write;

use crate::{ctx::NamingContext, r#type::Type};

fn doc_type_seq<S: AsRef<str>, I: IntoIterator<Item = S>>(
    nc: &NamingContext,
    key: &str,
    iterable: I,
) -> TokenStream {
    let mut text = format!("## {}\n", key);
    for name in iterable.into_iter() {
        let named = nc.resolve(name.as_ref()).unwrap();
        if let Some(m) = &named.source_mod {
            writeln!(text, "- [`{}::{}`]", m, named.ident)
        } else {
            writeln!(text, "- [`{}`]", named.ident)
        }
        .unwrap();
    }
    quote!(#[doc = #text])
}

fn doc_type_list(nc: &NamingContext, key: &str, list: &[String]) -> Option<TokenStream> {
    (!list.is_empty()).then(|| doc_type_seq(nc, key, list))
}

fn doc_type_map<T>(
    nc: &NamingContext,
    key: &str,
    set: &BTreeMap<String, T>,
) -> Option<TokenStream> {
    (!set.is_empty()).then(|| doc_type_seq(nc, key, set.keys()))
}

pub(crate) fn doc_attr(attr: &Attribute) -> TokenStream {
    let attr_doc = attr.doc.as_deref().map(|d| quote!(#[doc = #d]));
    let attr_doc_ref = attr.doc_ref.as_ref();
    let attr_doc_refs = attr_doc_ref.map(StringOrArray::as_slice).unwrap_or(&[]);

    let if_doc = attr
        .if_expr
        .as_ref()
        .map(|i| format!("If: `{}`", i))
        .map(|s| {
            quote!(
                #[doc = #s]
            )
        });
    let repeat_doc = attr.repeat.as_ref().map(|&r| {
        let rep_doc = match r {
            Repeat::Expr => format!("Repeat-Expr: `{}`", attr.repeat_expr.as_deref().unwrap()),
            Repeat::Eos => "Repeat until end-of-stream".to_string(),
            Repeat::Until => todo!(),
        };
        quote!(#[doc = #rep_doc])
    });
    let size_doc = attr.size.as_deref().map(|s| {
        let size_doc = format!("Size: `{}`", s);
        quote!(#[doc = #size_doc])
    });
    let encoding_doc = attr.encoding.as_deref().map(|s| {
        let size_doc = format!("Encoding: `{}`", s);
        quote!(#[doc = #size_doc])
    });
    let enum_doc = attr.r#enum.as_deref().map(|s| {
        let size_doc = format!("Enum: `{}`", s);
        quote!(#[doc = #size_doc])
    });
    quote!(
        #attr_doc
        #(#[doc = #attr_doc_refs])*
        #if_doc
        #size_doc
        #enum_doc
        #encoding_doc
        #repeat_doc
    )
}

pub(crate) fn doc_struct(
    self_ty: &Type,
    nc: &NamingContext,
    doc: Option<&str>,
    doc_ref: Option<&StringOrArray>,
) -> TokenStream {
    let id_doc_text = format!("# Struct `{}`", &self_ty.orig_id);
    let id_doc = quote!(#[doc = #id_doc_text]);
    let parser_doc_text = format!("Parser: [`{}`]", &self_ty.parser_name);
    let parser_doc = quote!(#[doc = #parser_doc_text]);

    let doc = doc.map(|d| quote!(#[doc = #d]));
    let doc_parents = doc_type_list(nc, "Parents", &self_ty.parents);
    let doc_maybe_parents = doc_type_list(nc, "Maybe parents", &self_ty.maybe_parents);
    let doc_depends_on = doc_type_map(nc, "Depends on", &self_ty.depends_on);
    //let doc_may_depend_on = doc_type_map(nc, "May depend on", &self_ty.may_depend_on);
    let doc_refs = doc_ref.map(StringOrArray::as_slice).unwrap_or(&[]);
    let doc_root_obligations = self_ty.root_obligations.doc("_root");
    let doc_parent_obligations = self_ty.parent_obligations.doc("_parent");
    quote!(
        #id_doc
        #doc
        #parser_doc
        #(#[doc = #doc_refs])*
        #doc_parents
        #doc_maybe_parents
        #doc_depends_on
        //#doc_may_depend_on
        #doc_root_obligations
        #doc_parent_obligations
    )
}
