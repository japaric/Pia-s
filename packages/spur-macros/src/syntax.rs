use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Token, Type, braced, bracketed, parenthesized, token};

mod keyword {
    use syn::custom_keyword;

    custom_keyword!(subscribed);
    custom_keyword!(to);
}

pub struct Subscriptions {
    _const: Token![const],
    _underscore: Token![_],
    _colon: Token![:],
    pub broker: Ident,
    _equal: Token![=],
    _braces: token::Brace,
    pub subscribers: Vec<Subscriber>,
    _semicolon: Token![;],
}

impl Parse for Subscriptions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let braced_content;
        Ok(Self {
            _const: input.parse()?,
            _underscore: input.parse()?,
            _colon: input.parse()?,
            broker: input.parse()?,
            _equal: input.parse()?,
            _braces: braced!(braced_content in input),
            subscribers: {
                let mut subscribers = vec![];
                while !braced_content.is_empty() {
                    subscribers.push(braced_content.parse()?);
                }
                subscribers
            },
            _semicolon: input.parse()?,
        })
    }
}

pub struct Subscriber {
    pub subscriptions: Vec<Subscription>,
    _const: Token![const],
    _underscore: Token![_],
    _colon: Token![:],
    pub type_: Type,
    _equal: Token![=],
    pub initializer: Expr,
    _semicolon: Token![;],
}

impl Parse for Subscriber {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            subscriptions: {
                let mut subscriptions = vec![];
                while input.peek(Token![#]) {
                    subscriptions.push(input.parse()?);
                }
                subscriptions
            },
            _const: input.parse()?,
            _underscore: input.parse()?,
            _colon: input.parse()?,
            type_: input.parse()?,
            _equal: input.parse()?,
            initializer: input.parse()?,
            _semicolon: input.parse()?,
        })
    }
}

pub struct Subscription {
    _pound: Token![#],
    _brackets: token::Bracket,
    _subscribed: keyword::subscribed,
    _parentheses: token::Paren,
    _to: keyword::to,
    _equal: Token![=],
    pub message_type: Type,
}

impl Parse for Subscription {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bracketed_content;
        let parenthesized_content;

        Ok(Self {
            _pound: input.parse()?,
            _brackets: bracketed!(bracketed_content in input),
            _subscribed: bracketed_content.parse()?,
            _parentheses: parenthesized!(parenthesized_content in bracketed_content),
            _to: parenthesized_content.parse()?,
            _equal: parenthesized_content.parse()?,
            message_type: parenthesized_content.parse()?,
        })
    }
}
