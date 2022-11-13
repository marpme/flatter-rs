use yew::{classes, function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    log::info!("Applying layout to site");

    html! {
        <main class={classes!("absolute", "inset-0", "bg-grid-slate-900/[0.04]", "bg-[bottom_1px_center]", "bg-grid-slate-400/[0.05]", "bg-bottom", "border-b", "border-slate-100/5")}>
            <nav class="bg-gray-800">
                <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                    <div class="relative flex h-16 items-center justify-between">
                      <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                        <div class="flex flex-shrink-0 items-center">
                          <img class="block h-8 w-auto lg:hidden" src="https://tailwindui.com/img/logos/mark.svg?color=red&shade=400" alt="Your Company" />
                          <img class="hidden h-8 w-auto lg:block" src="https://tailwindui.com/img/logos/mark.svg?color=red&shade=400" alt="Your Company" />
                          <h3 class="flex text-lg leading-6 font-semibold pl-2 border-b-2 -mb-px text-slate-100 border-transparent">{"Flatter"}</h3>
                        </div>
                        <div class="hidden sm:ml-6 sm:block">
                          <div class="flex space-x-4">
                            <a href="#" class="bg-gray-900 text-white px-3 py-2 rounded-md text-sm font-medium" aria-current="page">{"Dashboard"}</a>
                          </div>
                        </div>
                      </div>
                      <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
                        <div class="relative ml-3">
                          <div>
                            <div class="tooltip tooltip-bottom" data-tip="This is Roofy.">
                                <button type="button" class="flex rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800" id="user-menu-button" aria-expanded="false" aria-haspopup="true">
                                  <img class="h-8 w-8 rounded-full" src="https://images.unsplash.com/photo-1589254066213-a0c9dc853511?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=facearea&w=256&h=256&q=80" alt="" />
                                </button>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
            </nav>
            <div class={classes!("container", "mx-auto")}>
                { for props.children.iter() }
            </div>
        </main>
    }
}
