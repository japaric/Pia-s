use indexmap::IndexMap;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::syntax::{Subscriber, Subscription, Subscriptions};

pub fn subscriptions(
    Subscriptions {
        broker,
        subscribers,
        ..
    }: &Subscriptions,
) -> TokenStream {
    let krate = crate::crate_name();

    let modules = subscribers.iter().enumerate().map(
        |(
            subscriber_index,
            Subscriber {
                subscriptions,
                type_,
                initializer,
                ..
            },
        )| {
            let queues = subscriptions.iter().enumerate().map(
                |(index, Subscription { message_type, .. })| {
                    let queue_id = ident("queue", index);

                    quote!(
                        pub(super) static #queue_id: #krate::internal::Queue<#message_type> =
                            #krate::internal::Queue::new();
                    )
                },
            );

            let subscriber_id = ident("subscriber", subscriber_index);
            quote!(
                mod #subscriber_id {
                    #(#queues)*
                }

                static #subscriber_id: #krate::internal::Shared<#type_> =
                    #krate::internal::Shared::new(#initializer);
            )
        },
    );

    let mut subscriptions_by_message_type = IndexMap::new();
    for (subscriber_index, Subscriber { subscriptions, .. }) in subscribers.iter().enumerate() {
        for (subscription_index, Subscription { message_type, .. }) in
            subscriptions.iter().enumerate()
        {
            let message_id = quote!(#message_type).to_string();
            let queue_id = ident("queue", subscription_index);

            let subscriber_id = ident("subscriber", subscriber_index);
            subscriptions_by_message_type
                .entry(message_id)
                .or_insert_with(|| (message_type, vec![]))
                .1
                .push((queue_id, subscriber_id));
        }
    }

    let publish_impls =
        subscriptions_by_message_type
            .iter()
            .map(|(_message_id, (message_type, subscriptions))| {
                let num_subscriptions = subscriptions.len();

                let enqueues = subscriptions.iter().enumerate().map(
                |(enqueue_index, (queue_id, subscriber_id))| {
                    let message = if num_subscriptions > 1 && enqueue_index != num_subscriptions - 1
                    {
                        quote!(core::clone::Clone::clone(&message))
                    } else {
                        quote!(message)
                    };

                    quote!(
                        #subscriber_id::#queue_id.enqueue(#message);
                        #krate::internal::queue_microtask(|| {
                            if let Some(message) = #subscriber_id::#queue_id.dequeue() {
                                #krate::React::react(&mut *#subscriber_id.borrow_mut(), message);
                            }
                        });
                    )
                },
            );

                quote!(impl #krate::Publish<#message_type> for #broker {
                    fn publish(message: #message_type) {
                        #(#enqueues)*
                    }
                })
            });

    quote!(
        pub struct #broker;

        const _: () = {
            #(#modules)*
            #(#publish_impls)*
        };
    )
    .into()
}

fn ident(prefix: &str, index: usize) -> Ident {
    format_ident!("{prefix}{index}")
}
