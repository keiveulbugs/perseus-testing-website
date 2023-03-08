use sycamore::prelude::*;

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps { title, children }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);
    //let open = create_signal(cx, false);
    let theme_state = create_signal(cx, false);

    let mut theme = "light";
    if *theme_state.get() {
      theme="dark"
    } else {
      theme="light"
    }

    view! { cx,
      // tailwind.css
      
      
      html (
        data-theme=theme

      )
      
      

      


      header(class="company-navbar") {

        // LOGO IN NAVBAR, niet in tailwind.css
        div(class="container flex flex-wrap items-center justify-between mx-auto") {
          a (href="", class="flex items-center") {
            img(src=".perseus/static/perseusexamplewebsite.png", class="h-8 mr-3", alt="example logo")
            span(class="self-center text-xl font-bold text-white whitespace-nowrap") {"Example website!"}
          }


        // Fix dat kleine schermen een sub menu krijgen om rond te navigeren
        div(class="hidden w-full md:block md:w-auto md:mt-0") {
          ul(class="flex flex-col align-text-bottom border rounded-lg text-white md:flex-row md:space-x-8 md:text-lg hover:fill-company-hover md:font-bold md:border-0") {
            li() {
              // tailwind.css navbuttons
              a (href="/nothing", class="company-navbutton-effect") {"Nothing"}
            }
            li() {
              a (href="/Empty", class="company-navbutton-effect") {"Empty"}
            }
            div(class="dropdown dropdown-hover ease-out") {
              label(tabindex="0", class="hover:text-company-hover ease-out") {
                a (href="https://github.com/keiveulbugs", class="company-navbutton-effect") {"My Git"}
               }
              ul(tabindex="0", class="dropdown-content menu p-2 bg-company rounded-box w-51rounded-t-none") {
                li() {
                  a (href="https://github.com/keiveulbugs/RustDisco", class="company-navbutton-effect") {"RustDisco"}
                }
                li() {
                  a (href="https://github.com/keiveulbugs/Dexscreener_pricebot", class="company-navbutton-effect") {"Dexscreener bot"}
                }
                li() {
                  a (href="https://github.com/keiveulbugs/Discord-Role-Purger", class="company-navbutton-effect") {"Role purger"}
                }
                
                

              }
            }
            li() {
              a (href="/login", class="block hover:text-company-hover") {"Login"}
            }
            label (class="swap swap-rotate") {
              input (type="checkbox") {
                svg(class="swap-on fill-current w-7 h-7", xmlns="http://www.w3.org/2000/svg", viewBox="0 0 24 24"){
                  path(d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z")
                  
                }
                svg(class="swap-off fill-current w-7 h-7", xmlns="http://www.w3.org/2000/svg", viewBox="0 0 24 24") {
                  path(d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z")
                  
                }
                
                
              }
            }


          }
        }

      }
    }


              main() {
                            (children)


              }
              footer(class="company-footer") {
                div {
                  img(src=".perseus/static/perseusexamplewebsite.png", class="object-scale-down object-center h-10", alt="example logo")
                  p (class="text-xl font-bold") {"A Perseus Example Website!"}
                  a (class="opacity-75 font-semibold hover:text-company-hover", href="https://framesurge.sh/perseus/en-US/docs/") {"More info about perseus"}
                }
                div (class="font-semibold opacity-85 text-center content-center") {
                  span(class="footer-title") {
                    "Contributors"
                  }
                  a (href="https://github.com/keiveulbugs", class="hover:text-company-hover") {
                    "Keiveulbugs"
                  }
                }
                div (class="font-semibold opacity-85") {
                  span(class="footer-title") {
                    "Other sites"
                  }
                  a (href="https://github.com/keiveulbugs/Discord-Role-Purger", class="hover:text-company-hover") {
                    "Role purger"
                  }
                  a (href="https://github.com/keiveulbugs/Dexscreener_pricebot", class="hover:text-company-hover") {
                    "Dexscreener bot"
                  }
                  a (href="https://github.com/keiveulbugs/RustDisco", class="hover:text-company-hover") {
                    "RustDisco"
                  }
                  
                  
                }
              }

          }
}

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    /// The title of the page, which will be displayed in the header.
    pub title: &'a str,
    /// The content to put inside the layout.
    pub children: Children<'a, G>,
}

