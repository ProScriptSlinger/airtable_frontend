use bounce::{use_atom, use_atom_value};
use crate::controllers::table::Tables;
use crate::controllers::table::Table;
use yew::prelude::*;
use crate::components::dropdown::Dropdown;

#[function_component]
pub fn Sidebar() -> Html {
    let tables = use_atom_value::<Tables>();
    let active_table_setter = use_atom::<Table>();

    let open_dropdowns = use_state::<Option<usize>, _>(|| None);

    let toggle_dropdown = {
        let open_dropdowns = open_dropdowns.clone();
        move |index: Option<usize>| {
            open_dropdowns.set(index);
        }
    };

    html! {
        <aside id="default-sidebar" class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform sm:translate-x-0 border-r-[1px] border-gray-600" aria-label="Sidebar">
            <div class="h-full py-4 overflow-y-auto dark:bg-[#191919]">
                <div  class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-[#202020] group">
                    <svg class="w-8 h-8 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 22 21">
                        <path d="M16.975 11H10V4.025a1 1 0 0 0-1.066-.998 8.5 8.5 0 1 0 9.039 9.039.999.999 0 0 0-1-1.066h.002Z"/>
                        <path d="M12.5 0c-.157 0-.311.01-.565.027A1 1 0 0 0 11 1.02V10h8.975a1 1 0 0 0 1-.935c.013-.188.028-.374.028-.565A8.51 8.51 0 0 0 12.5 0Z"/>
                    </svg>
                    <span class="ms-3 font-bold">{"Airtable alternatives"}</span>
                </div>
                <hr class="border-gray-600 my-2"/>
                <ul class="space-y-2 font-medium px-3">
                    <a href="#" class="flex items-center justify-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-[#202020] group border-[1px] border-gray-600">
                        <i class="fas fa-plus"></i>
                    </a>
                    { for tables.tables.iter().enumerate().map(|(index, table)| {
                        let table_clone = table.clone();
                        let setter = active_table_setter.clone();

                        let is_open = *open_dropdowns == Some(index);

                        html! {
                            <li key={index}
                            >
                                <a href="#" class="flex items-center justify-between p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-[#202020] group border-[1px] border-gray-600 relative">
                                    <div class="flex items-center space-x-2" onclick={{
                                        let table = table_clone.clone();
                                        Callback::from(move |_| {
                                            setter.set(table.clone());
                                        })
                                    }}>
                                        <i class="fas fa-table"></i>
                                        <span class="ms-3">{table.name.clone()}</span>
                                    </div>
                                    <div data-dial-toggle={format!("dropdown-{}", index)}
                                            aria-controls={format!("dropdown-{}", index)} aria-expanded="false"
                                            onmouseenter = {
                                                let toggle_dropdown = toggle_dropdown.clone();
                                                Callback::from(move |_| {
                                                    toggle_dropdown(Some(index));
                                                })
                                            }
                                            onmouseleave={
                                                let toggle_dropdown = toggle_dropdown.clone();
                                                Callback::from(move |_| {
                                                    toggle_dropdown(None);
                                                })
                                            }
                                            class="relative flex items-center justify-center ml-auto text-white">
                                        <svg class="w-6 h-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"
                                                fill="currentColor" viewBox="0 0 16 3">
                                            <path d="M2 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3Zm6.041 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM14 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3Z"/>
                                        </svg>
                                        <span class="sr-only">{"Open actions menu"}</span>
                                        <Dropdown table_index={index} is_open={is_open}/>
                                    </div>
                                </a>
                            </li>
                        }
                    })}
                </ul>
            </div>
        </aside>
    }
} 
