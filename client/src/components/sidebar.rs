use bounce::{use_atom, use_atom_value};
use crate::controllers::table::Tables;
use crate::controllers::table::Table;
use yew::prelude::*;
use crate::components::dropdown::Dropdown;
use crate::components::content::{create_table, fetch_tables, update_table, delete_table_func};
use wasm_bindgen_futures::spawn_local;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{HtmlInputElement, MouseEvent};

lazy_static! {
    static ref TABLE_COUNTER: Mutex<usize> = Mutex::new(0);
}

#[function_component]
pub fn Sidebar() -> Html {
    let tables = use_atom_value::<Tables>();
    let table_handle = Rc::new(RefCell::new(use_atom::<Table>()));
    let tables_handle = Rc::new(RefCell::new(use_atom::<Tables>()));

    let open_dropdowns = use_state::<Option<usize>, _>(|| None);

    let toggle_dropdown = {
        let open_dropdowns = open_dropdowns.clone();
        move |index: Option<usize>| {
            open_dropdowns.set(index);
        }
    };
    
    
    let create_table_onclick = {
        let table_handle = Rc::clone(&table_handle); 
        let tables_handle = Rc::clone(&tables_handle);
        
        Callback::from(move |_| {
            let mut counter = TABLE_COUNTER.lock().unwrap();
            *counter += 1;
            let table_id = *counter;

            let table_title = format!("New Table - {}", table_id);
                
            let table_handle = Rc::clone(&table_handle);
            let tables_handle = Rc::clone(&tables_handle);
            
            spawn_local(async move {
                match create_table(table_title).await {
                    Ok(_) => {
                        web_sys::console::log_1(&"Table created successfully".into());
    
                        match fetch_tables().await {
                            Ok(tables) => {
                                let tables_struct = Tables { tables: tables.clone() };
                                tables_handle.borrow_mut().set(tables_struct);
    
                                if let Some(first_table) = tables.first() {
                                    table_handle.borrow_mut().set(first_table.clone());
                                }
    
                                // web_sys::console::log_1(&format!("Fetched Items: {:?}", tables).into());
                            },
                            Err(_e) => {
                                // Handle error
                            }
                        }
                    },
                    Err(e) => {
                        // Handle error
                    },
                }
            });
        })
    };

    let delete_table_onclick = {
        let table_handle = Rc::clone(&table_handle); 
        let tables_handle = Rc::clone(&tables_handle);
        
        Callback::from(move |table_id_with_prefix: String| {
            // Extract the actual table ID
            let id_parts: Vec<&str> = table_id_with_prefix.split(':').collect();
            let table_id = id_parts.get(1).unwrap_or(&"").to_string();
            let table_handle_inner = Rc::clone(&table_handle); 
            let tables_handle_inner = Rc::clone(&tables_handle); 
    
            let table_id_clone = table_id.clone(); 
    
            Callback::from(move |_: MouseEvent| {
                let table_handle = Rc::clone(&table_handle_inner);
                let tables_handle = Rc::clone(&tables_handle_inner);
                let table_id = table_id_clone.clone();
    
                spawn_local(async move {
                    match delete_table_func(table_id.clone()).await {
                        Ok(_) => {
                            web_sys::console::log_1(&"Table deleted successfully".into());
    
                            match fetch_tables().await {
                                Ok(tables) => {
                                    let tables_struct = Tables { tables: tables.clone() };
                                    tables_handle.borrow_mut().set(tables_struct);
    
                                    if let Some(first_table) = tables.first() {
                                        table_handle.borrow_mut().set(first_table.clone());
                                    }
                                },
                                Err(_e) => {
                                    // Handle error
                                }
                            }
                        },
                        Err(e) => {
                            // Handle error
                        },
                    }
                });
            })
        })
    };

    let renaming_id = use_state(|| None::<String>);
    let new_name = use_state(|| "".to_string()); // Example table title

    let on_save_click = {
        let renaming_id = renaming_id.clone();
        let renaming_id_handle = renaming_id.clone();
        let new_name = new_name.clone();
        let tables_handle = Rc::clone(&tables_handle);
        let table_handle = Rc::clone(&table_handle);
        
        Callback::from(move |_: MouseEvent| {
            log::info!("Saving new name: {}", *new_name);
    
            if let Some(ref renaming_id) = *renaming_id {
                let matching_table = tables_handle.borrow().tables.iter().find(|table| table.id == *renaming_id).cloned();
                
                if let Some(existing_table) = matching_table {
                    let new_table = Table {
                        id: renaming_id.clone(),
                        title: (*new_name).clone(),
                        data: existing_table.data.clone(), // Copy data from the matching table
                    };
    
                    let tables_handle_clone = Rc::clone(&tables_handle);  // Clone here to avoid move
                    let table_handle_clone = Rc::clone(&table_handle);    // Clone here to avoid move
                    renaming_id_handle.set(None);
                    spawn_local(async move {
                        match update_table(new_table.clone()).await {
                            Ok(_) => {
                                match fetch_tables().await {
                                    Ok(tables) => {
                                        let tables_struct = Tables { tables: tables.clone() };
                                        // Use the set method provided by the atom handle
                                        tables_handle_clone.borrow_mut().set(tables_struct);
    
                                        // if let Some(first_table) = tables.first() {
                                        table_handle_clone.borrow_mut().set(new_table.clone());
                                        // }
                                        // renaming_id_handle.set(None);
                                    },
                                    Err(_e) => {
                                        // Handle error
                                        // renaming_id_handle.set(None);
                                    }
                                }
                            },
                            Err(e) => {
                                // Handle error
                                // console::error_1(&e.into());
                            }
                        }
                    });

                } else {
                    // console::warn_1(&"No matching table found for renaming_id".into());
                }
            } else {
                // console::warn_1(&"renaming_id is None".into());
            }
        })
    };

    let on_input_change = {
        let new_name = new_name.clone();
        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            new_name.set(input.value());
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
                    <div
                        class="cursor-pointer flex items-center justify-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-[#202020] group border-[1px] border-gray-600"
                        onclick={create_table_onclick}
                    >
                        <i class="fas fa-plus"></i>
                    </div>
                    { for tables.tables.iter().enumerate().map(|(index, table)| {
                        let table_clone = table.clone();
                        let setter = Rc::clone(&table_handle);
        
                        let is_open = *open_dropdowns == Some(index);
        
                        html! {
                            <li key={index}>
                                <div class="flex items-center justify-between p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-[#202020] group border-[1px] border-gray-600 relative">
                                    {
                                        if renaming_id.as_ref() == Some(&table.id.clone()) {
                                            html! {
                                                <div class="flex items-center gap-2">
                                                    <input type="text"  value={(*new_name).clone()} oninput={on_input_change.clone()} class="min-w-32 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg  block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                                                    <button onclick={on_save_click.clone()} class="text-white border border-gray-600 hover:bg-gray-600 hover:border-gray-800 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-lg text-sm w-full sm:w-auto p-1 text-center dark:border-gray-600 dark:hover:border-gray-700 dark:focus:ring-gray-800"><i class="fas fa-archive"></i></button>
                                                </div>
                                            }
                                        } else {
                                            html! {
                                                <>
                                                    <div class="flex items-center space-x-2 cursor-pointer"
                                                        onclick={{
                                                            let table = table_clone.clone();
                                                            Callback::from(move |_| {
                                                                web_sys::console::log_1(&format!("Table------> {:?}",table.clone()).into());
                                                                setter.borrow_mut().set(table.clone());
                                                            })
                                                        }}>
                                                        <i class="fas fa-table"></i>
                                                        <span class="ms-3">{table.title.clone()}</span>
                                                    </div>
                                                    <div data-dial-toggle={format!("dropdown-{}", index)}
                                                        aria-controls={format!("dropdown-{}", index)} aria-expanded="false"
                                                        onmouseenter={{
                                                            let toggle_dropdown = toggle_dropdown.clone();
                                                            Callback::from(move |_| {
                                                                toggle_dropdown(Some(index));
                                                            })
                                                        }}
                                                        onmouseleave={{
                                                            let toggle_dropdown = toggle_dropdown.clone();
                                                            Callback::from(move |_| {
                                                                toggle_dropdown(None);
                                                            })
                                                        }}
                                                        class="relative flex items-center justify-center ml-auto text-white cursor-pointer">
                                                        <svg class="w-6 h-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 16 3">
                                                            <path d="M2 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3Zm6.041 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM14 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3Z"/>
                                                        </svg>
                                                        <span class="sr-only">{"Open actions menu"}</span>
                                                        <Dropdown 
                                                            set_renaming_id={{
                                                                let renaming_id = renaming_id.clone();
                                                                let new_name = new_name.clone();
                                                                let table_title = table.title.clone();  // Capture the table title for use in the closure
                                                                Callback::from(move |id: Option<String>| {
                                                                    renaming_id.set(id.clone());
                                                                    if id.is_some() {
                                                                        new_name.set(table_title.clone());
                                                                    }
                                                                })
                                                            }}
                                                            table_id={table.id.clone()} 
                                                            delete_table={delete_table_onclick.emit(table.id.clone())} // Properly formatted call
                                                            is_open={is_open}
                                                        />
                                                    </div>
                                                </>
                                            }
                                        }
                                    }
                                </div>
                            </li>
                        }
                    })}
                </ul>
            </div>
        </aside>
    }
}
