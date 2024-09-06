use yew::prelude::*;
use bounce::{use_atom, use_atom_value};
use crate::controllers::table::Table;

#[function_component]
pub fn Content() -> Html {
    let table = use_atom_value::<Table>();
    html! {
        <div class="p-4 sm:ml-64 min-h-screen">
            <div>
                <h2 class="text-white">{table.name.clone()}</h2>
                <hr class="border-[#1C2634] my-2"/>
                <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                    <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
                        <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                            <tr>
                                {for table.data[0].iter().map(|header| html! { 
                                    <th scope="col" class="px-6 py-3">
                                        {header}
                                    </th> 
                                })}
                            </tr>
                        </thead>
                        <tbody>
                            {for table.data[1..].iter().map(|row| html! {
                                <tr class="odd:bg-white odd:dark:bg-gray-900 even:bg-gray-50 even:dark:bg-gray-800 border-b dark:border-gray-700">
                                    {for row.iter().map(|cell| html! { 
                                        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                            {cell}
                                        </th> 
                                    })}
                                </tr>
                            })}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
        
    }
}
