use bounce::{use_atom, use_atom_value};
use crate::controllers::table::Tables;
use crate::controllers::table::Table;
use yew::prelude::*;

#[function_component]
pub fn Sidebar() -> Html {

    let tables = use_atom_value::<Tables>();
    let table_setter = use_atom::<Table>();

    html! {
        <aside id="default-sidebar" class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform sm:translate-x-0 border-r-[1px] border-[#1C2634]" aria-label="Sidebar">
            <div class="h-full py-4 overflow-y-auto dark:bg-[#191919] ">
                <a href="#" class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-[#202020] group">
                    <svg class="w-8 h-8 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 22 21">
                        <path d="M16.975 11H10V4.025a1 1 0 0 0-1.066-.998 8.5 8.5 0 1 0 9.039 9.039.999.999 0 0 0-1-1.066h.002Z"/>
                        <path d="M12.5 0c-.157 0-.311.01-.565.027A1 1 0 0 0 11 1.02V10h8.975a1 1 0 0 0 1-.935c.013-.188.028-.374.028-.565A8.51 8.51 0 0 0 12.5 0Z"/>
                    </svg>
                    <span class="ms-3 font-bold">{"Airtable alternatives"}</span>
                </a>
                <hr class="border-[#1C2634] my-2"/>
                <ul class="space-y-2 font-medium px-3">
                    <li>
                        <a href="#" class="flex items-center justify-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-[#202020] group  border-[1px] border-[#1C2634]">
                            <i class="fas fa-plus"></i>
                        </a>
                        {for tables.tables.iter().enumerate().map(|(index, table)| {
                            let table_clone = table.clone();
                            let setter = table_setter.clone();
        
                            html! {
                                <li key={index}
                                    onclick={{
                                        let table = table_clone.clone();
                                        Callback::from(move |_| {
                                            setter.set(table.clone()); 
                                        })
                                    }}
                                >
                                    <a href="#" class="flex items-center  p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-[#202020] group  border-[1px] border-[#1C2634]">
                                        <i class="fas fa-table"></i>
                                        <span class="ms-3">{table.name.clone()}</span> 
                                    </a>
                                </li>
                            }
                        })}
                    </li>
                </ul>
            </div>
        </aside>
    }
}
