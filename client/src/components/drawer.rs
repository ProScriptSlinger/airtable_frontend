use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)] 
pub struct DrawerProps {
    pub is_open: bool, 
    pub toggle_drawer: Callback<()>,
}

#[function_component]
pub fn Drawer(props: &DrawerProps) -> Html {
    let DrawerProps { is_open, toggle_drawer } = props;

    html! {
        if *is_open {
            <div id="drawer-example" class="fixed top-0 right-0 z-40 h-screen p-4 overflow-y-auto transition-transform bg-white w-80 dark:bg-gray-800" tabindex="-1" aria-labelledby="drawer-label">
                <h5 id="drawer-label" class="inline-flex items-center mb-4 text-base font-semibold text-gray-500 dark:text-gray-400">
                    {"Add Field"}
                </h5>
                <button  
                    onclick = {
                        let toggle_drawer = toggle_drawer.clone();
                        Callback::from(move |_: MouseEvent| {
                            toggle_drawer.emit(());
                        })
                    }
                    type="button" data-drawer-hide="drawer-example" aria-controls="drawer-example" class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 absolute top-2.5 end-2.5 flex items-center justify-center dark:hover:bg-gray-600 dark:hover:text-white" >
                    <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                    </svg>
                    <span class="sr-only">{"Close menu"}</span>
                </button>
                <hr class="border-gray-600 my-2"/>
                <form class="max-w-sm mx-auto">
                    <div class="mb-5">
                        <label for="name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Name"}</label>
                        <input type="text" id="name" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg  block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Field Name" required={true} />
                    </div>
                    <div class="mb-5">
                        <label for="type" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Type"}</label>
                        <select id="type" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg  block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                            <option>                            
                                {"Text"}
                            </option>
                            <option>{"Number"}</option>
                            <option>{"Single Select"}</option>
                            <option>{"Multiple Select"}</option>
                            <option>{"Date"}</option>
                            <option>{"URL"}</option>
                            <option>{"Checkbox"}</option>
                            <option>{"Attachment"}</option>
                            <option>{"Subtask"}</option>
                        </select>
                    </div>
                    <button type="submit" class="text-white border border-gray-600 hover:bg-gray-600 hover:border-gray-800 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:border-gray-600 dark:hover:border-gray-700 dark:focus:ring-gray-800">{"Save"}</button>
                    <button
                        onclick = {
                            let toggle_drawer = toggle_drawer.clone();
                            Callback::from(move |_: MouseEvent| {
                                toggle_drawer.emit(());
                            })
                        }
                         class="text-white hover:bg-gray-600 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center mx-4">{"Cancel"}</button>
                </form>
            </div>
        }
    }
}
