use sycamore::prelude::*;

#[component(inline_props)]
pub fn DropDown<'a, G: Html>(cx: Scope<'a>, links: Vec<(String, String)>) -> View<G> {
    let open = create_signal(cx, false);
    view! { cx,
        input(id="toggle-dropdown", type="checkbox", bind:checked=open)
        label(for="toggle-dropdown") { "Dropdown" }
        (if *open.get() {
        view! { cx,
            ul(class="flex flex-col p-2 rounded") {

                a(href="/about") {"about"}
            }
        }
        } else {
            view!{ cx, }
        })
    }
}
