use crate::components::dropdown::DropDown;
use crate::components::header::Layout;
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    // let open = create_signal(cx, true);
    view! { cx,
        Layout(title = "My site") {

            div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 95vh;") {
                h1 { "Welcome to the site!" }
                p {
                    "A lot more pages are needed"

                }


                    }
                }

    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
