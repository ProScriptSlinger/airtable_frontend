use yew::prelude::*;
use yew::functional::use_effect_with;
use web_sys::console;

#[derive(Properties, PartialEq, Clone)]
pub struct CustomMultiSelectProps {
    pub handle_change: Callback<Vec<String>>,
    pub value: Vec<String>,
    pub multiple: bool,
}

#[function_component(CustomMultiSelect)]
pub fn my_component(props: &CustomMultiSelectProps) -> Html {
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

    let dropdown_open = use_state(|| false);
    let selected_values = use_state_eq(|| props.value.clone());
    let content_state = vec!["Jose".to_string(), "Abel".to_string(), "Hardy".to_string(), "Stanley".to_string(), "Brondon".to_string()];

    // Sync selected_values with props.value changes
    {
        let selected_values = selected_values.clone();
        let props_value = props.value.clone();

        use_effect_with(
            props_value.clone(),
            move |_| {
                selected_values.set(props_value.to_vec());
                || ()
            }
        );
    }

    let toggle_dropdown = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| dropdown_open.set(!*dropdown_open))
    };

    let handle_select_change = {
        let selected_values = selected_values.clone();
        let handle_change = props.handle_change.clone();
        let multiple = props.multiple; // Clone the multiple property
        Callback::from(move |value: String| {
            let new_selected = if multiple {
                let mut updated_selected = (*selected_values).clone();
                if updated_selected.contains(&value) {
                    updated_selected.retain(|v| v != &value);
                } else {
                    updated_selected.push(value);
                }
                updated_selected
            } else {
                vec![value]
            };
            console::log_2(&"hello world".into(), &format!("{:?}", new_selected.to_vec()).into());
            selected_values.set(new_selected.clone());
            handle_change.emit(new_selected);
        })
    };

    let remove_badge = {
        let selected_values = selected_values.clone();
        let handle_change = props.handle_change.clone();
        Callback::from(move |value: String| {
            let mut new_selected = (*selected_values).clone();
            new_selected.retain(|v| v != &value);
            selected_values.set(new_selected.clone());
            console::log_2(&"hello world".into(), &format!("{:?}", new_selected.to_vec()).into());
            handle_change.emit(new_selected);
        })
    };

    let badges: Vec<Html> = selected_values.iter().enumerate().map(|(index, value)| {
        let (bg_class, text_class, dark_bg_class, dark_text_class) = badge_classes[index % badge_classes.len()];
        let value_clone = value.clone();
        let remove_badge = remove_badge.clone();

        html! {
            <span class={classes!(
                "inline-flex",
                "items-center",
                "py-0.5",
                "pl-2.5",
                "pr-1",
                "rounded",
                "text-xs",
                "font-medium",
                "mr-2",
                bg_class,
                text_class,
                dark_bg_class,
                dark_text_class
            )}>
                {value.clone()}
                <button type="button" class="text-white ml-1 h-4 w-4 rounded-full inline-flex items-center justify-center text-black hover:bg-black hover:bg-opacity-20 focus:outline-none focus:bg-opacity-30"
                    onclick={remove_badge.reform(move |_| value_clone.clone())}>
                    {"×"}
                </button>
            </span>
        }
    }).collect();


    html! {
        <div class="w-full relative" style="min-width: 200px;">
            <div onclick={toggle_dropdown.clone()} class="cursor-pointer bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                {for badges}
            </div>
            if *dropdown_open {
                <ul class="absolute z-10 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg w-full mt-1 overflow-hidden">
                    { for content_state.iter().map(|value| {
                        let value_clone = value.clone();
                        let selected = selected_values.contains(&value_clone);  // Pass a reference here
                        let (bg_class, text_class, dark_bg_class, dark_text_class) = badge_classes[content_state.iter().position(|x| x == value).unwrap() % badge_classes.len()];
                        html! {
                            <li class={classes!(
                                "cursor-pointer",
                                "px-4",
                                "py-2",
                                "hover:bg-blue-200",
                                bg_class,
                                text_class,
                                dark_bg_class,
                                dark_text_class
                            )} onclick={handle_select_change.reform(move |_| value_clone.clone())}>
                                {if selected { "✓ " } else { "" }} {value}
                            </li>
                        }
                    })}

                </ul>
            }
        </div>
    }
}
