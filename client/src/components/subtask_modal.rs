use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SubtaskModalProps {
    pub is_open: bool,
    pub toggle_modal: Callback<()>,
}

#[function_component(SubtaskModal)]
pub fn subtask_modal(props: &SubtaskModalProps) -> Html {
    let SubtaskModalProps { is_open, toggle_modal } = props.clone();
    
    html! {
        if is_open {
            <div id="default-modal" tabindex="-1" class=" bg-white dark:bg-gray-700  fixed top-1/3 left-[30vw] transform -translate-y-1/2 z-50 w-auto h-auto max-h-full overflow-y-auto overflow-x-hidden bg-white rounded shadow-lg">
                <div class="relative w-[40vw]">
                    <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600">
                        <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
                            { "Subtask" }
                        </h3>
                        <button 
                            onclick = {
                                let toggle_modal = toggle_modal.clone();
                                Callback::from(move |_: MouseEvent| {
                                    toggle_modal.emit(());
                                })
                            }
                            type="button" class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white"
                        >
                            <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                            </svg>
                            <span class="sr-only">{"Close modal"}</span>
                        </button>
                    </div>
                    </div>
                    <form class="p-4 md:p-5">
                        <div class="grid gap-4 mb-4 grid-cols-2">
                            <div class="col-span-2">
                                <label for="name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Name"}</label>
                                <input type="text" name="name" id="name" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500" placeholder="Type product name"/>
                            </div>
                            <div class="col-span-2 sm:col-span-1">
                                <label for="price" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Completed"}</label>
                                <input type="number" name="price" id="price" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500" placeholder="$2999"/>
                            </div>
                            <div class="col-span-2 sm:col-span-1">
                                <label for="category" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Priority"}</label>
                                <select id="category" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-primary-500 focus:border-primary-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500">
                                <option selected={true}>{"Urgent"}</option> 
                                <option value="soon">{"Soon"}</option>
                                <option value="later">{"Later"}</option> 
                                <option value="whenever">{"Whenever"}</option>
                                </select>
                            </div>
                            <div class="col-span-2">
                                <label for="description" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Description"}</label>
                                <textarea id="description" rows="4" class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Write product description here"></textarea>                    
                            </div>
                        </div>
                    </form>
                    <div class="flex items-center justify-end p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600">
                        <button
                            onclick = {
                                let toggle_modal = toggle_modal.clone();
                                Callback::from(move |_: MouseEvent| {
                                    toggle_modal.emit(());
                                })
                            }
                            class="text-white hover:bg-gray-600 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center mx-4"
                        >
                            {"Cancel"}
                        </button>
                        <button type="submit" class="text-white border border-gray-600 hover:bg-gray-600 hover:border-gray-800 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:border-gray-600 dark:hover:border-gray-700 dark:focus:ring-gray-800">{"Save"}</button>
                    </div>
            </div>
        }
    }
}
