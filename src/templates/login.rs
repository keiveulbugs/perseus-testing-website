use crate::components::header::Layout;
use perseus::prelude::*;
use sycamore::prelude::*;
// use openidconnect::{
//     AccessTokenHash,
//     AuthenticationFlow,
//     AuthorizationCode,
//     ClientId,
//     ClientSecret,
//     CsrfToken,
//     Nonce,
//     IssuerUrl,
//     PkceCodeChallenge,
//     RedirectUrl,
//     Scope,
// };
// use openidconnect::core::{
//   CoreAuthenticationFlow,
//   CoreClient,
//   CoreProviderMetadata,
//   CoreResponseType,
//   CoreUserInfoClaims,
// };
// use anyhow::anyhow;

// use openidconnect::reqwest::http_client;
// use url::Url;

fn login<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Layout(title = "Login") {
            div(class="container mx-auto min-h-screen pt-15") {
                div(class="flex flex-wrap justify-center") {
                    div(class="flex flex-col break-words bg-white border rounded shadow-lg") {
                        div(class="w-full max-w-2xl") {
                            div(class="font-semibold py-5 px-10") {
                                p {"Login"}
                                
                            }
                        }

                    }
                    
                }
            }
        }
    }
}


#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Login!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("login").view(login).head(head).build()
}
