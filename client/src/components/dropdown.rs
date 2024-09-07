use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    pub table_index: usize,
    pub is_open: bool
}

#[function_component]
pub fn Dropdown(props: &DropdownProps) -> Html {
    let DropdownProps { table_index, is_open } = props;

    html! {
        if *is_open {
            <div id={format!("dropdown-{}", table_index)} class="absolute flex-col justify-end py-1 mb-2 space-y-2 bg-white border border-gray-100 rounded-lg shadow-sm dark:bg-gray-700 dark:border-gray-600 top-4 right-1 z-50 ">
                <ul class="text-sm text-gray-500 dark:text-gray-300">
                    <li>
                        <a href="#" class="flex space-x-2 items-center px-5 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-edit"></i>
                            <span class="text-sm font-medium">{"Rename"}</span>
                        </a>
                    </li>
                    <li>
                        <a href="#" class="flex space-x-2 items-center px-5 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-cogs"></i>
                            <span class="text-sm font-medium">{"Design"}</span>
                        </a>
                    </li>
                    <li>
                        <a href="#" class="flex space-x-2 items-center px-5 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-trash"></i>
                            <span class="text-sm font-medium">{"Delete"}</span>
                        </a>
                    </li>
                </ul>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ColumnDropdownProps {
    pub column_index: usize,
    pub is_open: bool
}

#[function_component]
pub fn ColumnDropdown(props: &ColumnDropdownProps) -> Html {
    let ColumnDropdownProps { column_index, is_open } = props;

    html! {
        if *is_open {
            <div id={format!("speed-dial-menu-dropdown-alternative-square-{}", column_index)} class="flex flex-col w-40 justify-end py-1 mb-4 space-y-2 bg-white border border-gray-100 rounded-lg shadow-sm dark:bg-gray-700 dark:border-gray-600 absolute  top-8 left-4 z-50 ">
                <ul class="text-sm text-gray-500 dark:text-gray-300">
                    <li>
                        <a href="#" class="flex space-x-2 items-center px-5 py-2  border-b border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-pencil"></i>
                            <span style="text-transform: none;" class="text-sm font-medium">{"Edit field"}</span>
                        </a>
                    </li>
                    <li>
                        <a href="#" class="flex space-x-2 items-center px-5 py-2  hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-arrow-left"></i>
                            <span style="text-transform: none;" class="text-sm font-medium">{"Insert left"}</span>
                        </a>
                    </li>
                    <li>
                        <a href="#" class="flex space-x-2 items-center px-5 py-2 border-b border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-arrow-right"></i>
                            <span style="text-transform: none;" class="text-sm font-medium">{"Insert right"}</span>
                        </a>
                    </li>
                    <li>
                        <a href="#" class="flex space-x-2 items-center px-5 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white">
                            <i class="fas fa-trash text-danger"></i>
                            <span style="text-transform: none;" class="text-sm font-medium">{"Delete"}</span>
                        </a>
                    </li>
                </ul>
            </div>
        }
    }
}


