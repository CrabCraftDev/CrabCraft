use proc_macro2::Ident;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Expr, Meta, Token, Type, parse2};
use syn::{
    LitInt,
    parse::{Parse, ParseStream},
};

pub(crate) struct RootPacketArgs {
    pub(crate) state_id_mapping: StateIdMapping,
    pub(crate) name: Expr,
}

#[allow(clippy::large_enum_variant)]
pub(crate) enum StateIdMapping {
    OneState { id: Expr, state: Expr },
    ManyStates { ids: Vec<(Ident, LitInt)> },
}

pub(crate) fn parse_packet_args(attrs: &[Attribute]) -> syn::Result<Option<RootPacketArgs>> {
    for attr in attrs {
        if !attr.path().is_ident("packet") {
            continue;
        }

        let mut id = None;
        let mut state = None;
        let mut name = None;
        let mut state_id_mapping = None;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                id = Some(meta.value()?.parse::<Expr>()?);
                Ok(())
            } else if meta.path.is_ident("ids") {
                let mut state_id_pairs = Vec::new();

                meta.parse_nested_meta(|meta| {
                    let state = meta.path.require_ident()?.clone();
                    let id: LitInt = meta.value()?.parse()?;
                    state_id_pairs.push((state, id));
                    Ok(())
                })?;

                state_id_mapping = Some(state_id_pairs);
                Ok(())
            } else if meta.path.is_ident("state") {
                state = Some(meta.value()?.parse::<Expr>()?);
                Ok(())
            } else if meta.path.is_ident("name") {
                name = Some(meta.value()?.parse::<Expr>()?);
                Ok(())
            } else {
                Err(meta.error("Unrecognized packet argument"))
            }
        })?;

        let state_id_mapping = match state_id_mapping {
            Some(state_id_mapping) => StateIdMapping::ManyStates { ids: state_id_mapping },
            None => StateIdMapping::OneState {
                id: id.ok_or_else(|| Error::new(attr.span(), "Id not provided for packet"))?,
                state: state.ok_or_else(|| Error::new(attr.span(), "State not provided for packet"))?,
            },
        };

        let res = RootPacketArgs {
            name: name.ok_or_else(|| Error::new(attr.span(), "Name not provided for packet"))?,
            state_id_mapping,
        };

        return Ok(Some(res));
    }

    Ok(None)
}

pub enum ProtocolAttribute {
    Bounded(Option<Expr>),
    Type(Option<Type>),
}

impl Parse for ProtocolAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) && input.peek2(Token![=]) {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            match ident.to_string().as_str() {
                "bounded" => Ok(ProtocolAttribute::Bounded(Some(input.parse()?))),
                "protocol_type" => Ok(ProtocolAttribute::Type(Some(input.parse()?))),
                _ => Err(Error::new(ident.span(), "Expected bounded or protocol_type")),
            }
        } else {
            Err(input.error("Invalid syntax! Expected <bounded/protocol_type> = <argument>"))
        }
    }
}

pub fn parse_attributes(attrs: &[Attribute]) -> Result<Option<ProtocolAttribute>, Error> {
    for attribute in attrs {
        let Meta::List(list) = &attribute.meta else {
            continue;
        };

        if !list.path.is_ident("protocol_attribute") {
            continue;
        }

        let args: ProtocolAttribute = parse2(list.tokens.clone())?;
        return Ok(Some(args));
    }
    Ok(None)
}
