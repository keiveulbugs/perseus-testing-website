use perseus::errors::ClientError;
use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::header::Layout;

pub fn get_error_views<G: Html>() -> ErrorViews<G> {
    ErrorViews::new(|cx, err, _err_info, _err_pos| {
        match err {
            ClientError::ServerError { status, message: _ } => match status {
                404 => (
                    
                    view! {cx,
                        Layout(title="Page not found:(") {
                        div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 95vh;") {
                            h1 { "This page does not seem to exist :(" }
                            p {
                                "Please check the URL you fucking Nitwit, this page will be fixed later on."

                            }
                            a ( href="") {"home"}
                        }
                    }},

                    view! {cx, title{ "Page not found:( "}},
                ),
                // 4xx is a client error
                _ if (400..500).contains(&status) => (
                    view! { cx,
                        title { "Error" }
                    },
                    view! { cx,
                        p { "There was something wrong with the last request, please try reloading the page." }
                    },
                ),
                // 5xx is a server error
                _ => (
                    view! { cx,
                        title { "Error" }
                    },
                    view! { cx,
                        p { "Sorry, our server experienced an internal error. Please try reloading the page." }
                    },
                ),
            },
            ClientError::Panic(_) => (
                view! { cx,
                    title { "Critical error" }
                },
                view! { cx,
                    p { "Sorry, but a critical internal error has occurred. This has been automatically reported to our team, who'll get on it as soon as possible. In the mean time, please try reloading the page." }
                },
            ),
            ClientError::FetchError(_) => (
                view! { cx,
                    title { "Error" }
                },
                view! { cx,
                    p { "A network error occurred, do you have an internet connection? (If you do, try reloading the page.)" }
                },
            ),
            _ => (
                view! { cx,
                    title { "Error" }
                },
                view! { cx,
                    p { (format!("An internal error has occurred: '{}'.", err)) }
                },
            ),
        }
    })
}
