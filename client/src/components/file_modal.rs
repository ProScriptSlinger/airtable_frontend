use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlInputElement, Event};
use wasm_bindgen::closure::Closure;

#[derive(Properties, PartialEq, Clone)]
pub struct AddFileModalProps {
    pub is_open: bool,
    pub toggle_modal: Callback<()>,
}

#[derive(Clone, Debug)]
struct FilePreview {
    data_url: String,
    mime_type: String,
}

#[function_component(AddFileModal)]
pub fn file_modal(props: &AddFileModalProps) -> Html {
    let AddFileModalProps { is_open, toggle_modal } = props.clone();
    let files = use_state(Vec::new);

    let on_files_selected = {
        let files = files.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(file_list) = input.files() {
                for i in 0..file_list.length() {
                    if let Some(file) = file_list.get(i) {
                        let reader = FileReader::new().unwrap();
                        let files = files.clone();
                        let mime_type = file.type_();

                        let onloadend = Closure::wrap(Box::new(move |event: Event| {
                            let reader: FileReader = event.target().unwrap().dyn_into().unwrap();
                            if let Ok(result) = reader.result() {
                                if let Some(data_url) = result.as_string() {
                                    files.set({
                                        let mut previews = (*files).clone();
                                        previews.push(FilePreview {
                                            data_url,
                                            mime_type: mime_type.clone(),
                                        });
                                        previews
                                    });
                                }
                            }
                        }) as Box<dyn FnMut(_)>);

                        reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                        reader.read_as_data_url(&file).expect("Failed to read file");

                        // Keep the closure alive until the callback is invoked
                        onloadend.forget();
                    }
                }
            }
        })
    };

    let remove_file = {
        let files = files.clone();
        Callback::from(move |index: usize| {
            files.set({
                let mut previews = (*files).clone();
                previews.remove(index);
                previews
            });
        })
    };

    html! {
        if is_open {
            <div id="default-modal" tabindex="-1" class=" bg-white dark:bg-gray-700  fixed top-1/3 left-[20vw] transform -translate-y-1/2 z-50 w-auto h-auto max-h-full overflow-y-auto overflow-x-hidden bg-white rounded shadow-lg">
                <div class="relative w-[60vw]">
                    <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600">
                        <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
                            { "Upload Files" }
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
                    // Preview Section
                    <div id="preview" class="mb-4 flex flex-wrap gap-2 p-2">
                        { for (*files).iter().enumerate().map(|(i, preview)| render_preview(preview, i, remove_file.clone())) }
                        <label for="dropzone-file" class="flex flex-col items-center justify-center w-full h-24 w-24 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-gray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600">
                            <div class="flex flex-col items-center justify-center pt-5 pb-6">
                                <svg class="w-8 h-8 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 16">
                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"/>
                                </svg>
                            </div>
                            <input id="dropzone-file" type="file" multiple=true class="hidden" onchange={on_files_selected} />
                        </label>
                    </div>
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

fn render_preview(preview: &FilePreview, index: usize, remove_file: Callback<usize>) -> Html {
    let FilePreview { data_url, mime_type } = preview;

    html! {
        <div class="relative h-24 w-24">
            <button
                onclick={
                    let remove_file = remove_file.clone();
                    Callback::from(move |_: MouseEvent| {
                        remove_file.emit(index);
                    })
                }
                class="absolute top-1 right-1 bg-red-500 hover:bg-red-300 text-white rounded-full px-2"
            >
                <i class="fas fa-times"/>
            </button>
            {
                if mime_type.starts_with("image/") {
                    html! {
                        <img src={data_url.clone()} class="file-preview w-full h-full border rounded-lg" />
                    }
                } else if mime_type.starts_with("video/") {
                    html! {
                        <video controls=true class="file-preview w-full h-full border rounded-lg">
                            <source src={data_url.clone()} type={mime_type.clone()} />
                            { "Your browser does not support the video tag." }
                        </video>
                    }
                } else if mime_type == "application/pdf" {
                    html! {
                        <iframe src={format!("{}#toolbar=0&navpanes=0&scrollbar=0", data_url)} class="no-scrollbar rounded-lg file-preview w-full h-full border"></iframe>
                    }
                } else {
                    html! {
                        <div class="file-preview w-full h-full bg-gray-200 border flex items-center justify-center">
                            <i class="fas fa-file"/>
                        </div>
                    }
                }
            }
        </div>
    }
}
