use yew::prelude::*;
use bounce::{use_atom_value, use_atom};
use crate::controllers::table::{Table, ContentType};
use crate::components::dropdown::ColumnDropdown;
use crate::components::drawer::Drawer;
use crate::components::multi_select::CustomMultiSelect;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::console;

#[derive(Properties, PartialEq)]
pub struct DisplayCellIconProps {
    pub content_type: ContentType,
}

#[function_component(DisplayCellIcon)]
fn display_cell_icon(props: &DisplayCellIconProps) -> Html {
    let DisplayCellIconProps { content_type } = props;
    
    fn get_icon(content_type: &ContentType) -> &'static str {
        match content_type {
            ContentType::Text => "<i class=\"fas fa-font\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Number => "<i class=\"fas fa-hashtag\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::SingleSelect => "<i class=\"fas fa-list\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::MultipleSelect => "<i class=\"fas fa-tasks\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Date => "<i class=\"fas fa-calendar-alt\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Url => "<i class=\"fas fa-link\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Checkbox => "<i class=\"fas fa-check-square\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Attachment => "<i class=\"fas fa-paperclip\" style=\"color: white; font-size: 15px;\"></i>",
            ContentType::Subtask => "<i class=\"fas fa-project-diagram\" style=\"color: white; font-size: 15px;\"></i>",
        }
    }
    
    let icon_html = get_icon(&content_type);

    html! {
        <div class="flex items-center gap-2">
            { Html::from_html_unchecked(AttrValue::from(icon_html)) }
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct EditableCellProps {
    pub content_type: ContentType,
    pub content: Vec<String>,
    pub on_save: Callback<Vec<String>>,
}

#[function_component(EditableCell)]
fn editable_cell(props: &EditableCellProps) -> Html {
    let props = props.clone();
    let EditableCellProps { content_type, content, on_save } = props.clone();
    let is_editing = use_state(|| false);
    let content_state = use_state(|| content.clone());
    
    let toggle_editing = {
        let is_editing = is_editing.clone();
        move |_: MouseEvent| {
            is_editing.set(!*is_editing);
        }
    };
    
    let handle_change = {
        let content_state = content_state.clone();
        move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            content_state.set(vec![input.value()]);
        }
    };

    let handle_checkbox_change = {
        let content_state = content_state.clone();
        move |e: web_sys::Event| {
            let input: web_sys::HtmlInputElement = e.target().unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            content_state.set(vec![if input.checked() { "true".to_string() } else { "false".to_string() }]);
        }
    };
    
    let handle_save = {
        let is_editing = is_editing.clone();
        let content_state = content_state.clone();
        let on_save = on_save.clone();
        // console::log_1(&"handle_save ----->".into());
        move |_: MouseEvent| {
            on_save.emit(content_state.to_vec());
            is_editing.set(false);
        }
    };
    let badge_classes = [
                            ("bg-blue-100", "text-blue-800", "dark:bg-blue-900", "dark:text-blue-300"),
                            ("bg-gray-100", "text-gray-800", "dark:bg-gray-700", "dark:text-gray-300"),
                            ("bg-red-100", "text-red-800", "dark:bg-red-900", "dark:text-red-300"),
                            ("bg-green-100", "text-green-800", "dark:bg-green-900", "dark:text-green-300"),
                            ("bg-yellow-100", "text-yellow-800", "dark:bg-yellow-900", "dark:text-yellow-300"),
                            ("bg-indigo-100", "text-indigo-800", "dark:bg-indigo-900", "dark:text-indigo-300"),
                            ("bg-purple-100", "text-purple-800", "dark:bg-purple-900", "dark:text-purple-300"),
                            ("bg-pink-100", "text-pink-800", "dark:bg-pink-900", "dark:text-pink-300")
                        ];

    let selected_values = use_state(|| vec![]);

    let handle_select_change = {
        let selected_values = selected_values.clone();
        Callback::from(move |values: Vec<String>| {
            selected_values.set(values);
        })
    };
    html! {
        if *is_editing {
            <div class="flex items-center gap-2">
                {
                    match content_type {
                        ContentType::Text | ContentType::Url => html! {
                            <input type="text"  value={content_state[0].clone()} oninput={handle_change}  class="min-w-32 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                        },
                        ContentType::SingleSelect | ContentType::MultipleSelect => html! {
                            <CustomMultiSelect  handle_change={handle_select_change} value={(*content_state).clone()} />
                        },
                        ContentType::Number => html! {
                            <input type="number"  value={content_state[0].clone()} oninput={handle_change} class="min-w-32 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                        },
                        ContentType::Date => html! {
                            <input type="date" value={content_state[0].clone()} oninput={handle_change}  class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                        },
                        ContentType::Checkbox => html! {
                            <input id="default-checkbox" type="checkbox" checked={content_state[0] == "true"} onchange={handle_checkbox_change}  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"/>
                        },
                        _ => html! {
                            <input type="checkbox" value={content_state[0].clone()} oninput={handle_change} class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                        }
                    }
                }
                <button onclick={handle_save} class="text-white border border-gray-600 hover:bg-gray-600 hover:border-gray-800 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-lg text-sm w-full sm:w-auto p-1 text-center dark:border-gray-600 dark:hover:border-gray-700 dark:focus:ring-gray-800"><i class="fas fa-archive"></i></button>
            </div>
        } else {
            <div class="cursor-pointer" onclick={toggle_editing}>
                {match content_type {
                    ContentType::SingleSelect | ContentType::MultipleSelect => {
                        html! {
                            <>
                                { for content_state.iter().enumerate().map(|(index, value)| {
                                    let (bg_class, text_class, dark_bg_class, dark_text_class) = badge_classes[index % badge_classes.len()];
                                    html! {
                                        <span class={classes!(
                                            bg_class,
                                            text_class,
                                            dark_bg_class,
                                            dark_text_class,
                                            "text-xs",
                                            "font-medium",
                                            "me-2",
                                            "px-2.5",
                                            "py-0.5",
                                            "rounded"
                                        )}>
                                            {value.clone()}
                                        </span>
                                    }
                                }) }
                            </>
                        }
                    },
                    // ContentType::Number => html! {
                    //     <input type="number"  value={content_state[0].clone()} oninput={handle_change}  class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                    // },
                    // ContentType::Date => html! {
                    //     <input type="date" value={content_state[0].clone()} oninput={handle_change}  class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                    // },
                    ContentType::Checkbox => html! {
                        <input id="default-checkbox" type="checkbox" checked={content_state[0] == "true"} onchange={handle_checkbox_change}  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"/>
                    },
                    _ => html! {
                        <span>
                            { &content_state[0] }
                        </span>
                    }
                }}
            </div>
        }
    }
}

#[function_component]
pub fn Content() -> Html {
    let table = use_atom_value::<Table>();

    let open_dropdowns = use_state::<Option<usize>, _>(|| None);
    let toggle_dropdown = {
        let open_dropdowns = open_dropdowns.clone();
        move |index: Option<usize>| {
            open_dropdowns.set(index);
        }
    };

    let open_drawer = use_state(|| false);

    let toggle_drawer = {
        let open_drawer = open_drawer.clone();
        move |_| {
            open_drawer.set(!*open_drawer);
        }
    };

    let table_handle = use_atom::<Table>();
    let update_cell_content = {
        let table_handle = table_handle.clone(); // Clone if necessary for use inside closure
        let table = (*table).clone(); // Dereference Rc and clone Table
    
        move |row_idx: usize, col_idx: usize, new_content: Vec<String>| {
            let mut updated_table = table.clone(); // Clone the current table state
            
            // Ensure the indices are within valid bounds
            if row_idx < updated_table.data.len() && col_idx < updated_table.data[row_idx].len() {
                updated_table.data[row_idx][col_idx].content = new_content; // Update the specific cell content
                table_handle.set(updated_table); // Set the updated table back to the state
            } else {
                // Log the error or handle it accordingly
                web_sys::console::log_1(&format!("Index out of bounds: row_idx={}, col_idx={}", row_idx, col_idx).into());
            }
        }
    };
    

    html! {
        <div class="p-4 sm:ml-64 min-h-screen overflow-x-auto">
            <div>
                <h2 class="text-white">{table.name.clone()}</h2>
                <hr class="border-gray-600 my-2"/>
                <div class="flex">
                    <div class="relative overflow-x-auto shadow-md sm:rounded-lg flex-col w-full" style="overflow:visible">
                        <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400 h-fit">
                            <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                                <tr>
                                    {for table.data[0].iter().enumerate().map(|(index, header)| {
                                        let is_open = *open_dropdowns == Some(index);
                                        html! { 
                                            <th scope="col" class="px-6 py-3 cursor-pointer relative border-r border-gray-600" 
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
                                            >
                                                <div class="flex gap-x-2">
                                                    <DisplayCellIcon content_type={header.content_type.clone()} />
                                                    {header.content[0].clone()}
                                                    <i class="fas fa-sort-desc mb-1"></i>
                                                    <ColumnDropdown column_index={index} is_open={is_open}/>
                                                </div>
                                            </th> 
                                        }
                                    })}
                                </tr>
                            </thead>
                            <tbody>
                            {for table.data[1..].iter().enumerate().map(|(row_idx, row)| html! {
                                    <tr class="odd:bg-white odd:dark:bg-gray-900 even:bg-gray-50 even:dark:bg-gray-800 border-b dark:border-gray-700 ">
                                        {for row.iter().enumerate().map(|(col_idx, cell)| html! { 
                                            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white border-r border-gray-600">
                                                <EditableCell
                                                    content_type={cell.content_type.clone()}
                                                    content={cell.content.clone()}
                                                    on_save={Callback::from({
                                                        let update_cell_content = update_cell_content.clone();
                                                        move |new_content: Vec<String>| {
                                                            update_cell_content(row_idx + 1, col_idx, new_content);
                                                        }
                                                    })}
                                                />
                                            </th> 
                                        })}
                                    </tr>
                                })}
                            </tbody>
                        </table>
                        <div class="w-full bg-gray-800 hover:bg-gray-700 text-white cursor-pointer text-sm text-center py-4 border-r border-gray-600">
                            <i class="fas fa-plus"></i>
                        </div>
                    </div>
                    <div class="w-20 bg-gray-700 hover:bg-gray-500 text-white cursor-pointer text-sm text-center py-4"
                        onclick = {
                            let toggle_drawer = toggle_drawer.clone();
                            Callback::from(move |_: MouseEvent| {
                                toggle_drawer(());
                            })
                        }
                    ><i class="fas fa-plus"></i></div>
                </div>
            </div>
            <Drawer is_open={*open_drawer} toggle_drawer={toggle_drawer}/>
        </div>
    }
}
