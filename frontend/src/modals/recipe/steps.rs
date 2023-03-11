use crate::contexts::prelude::{push_toast, use_toasts};
use crate::core::handlers::{api_error_to_toast, logout_on_401};
use crate::core::types::recipe::{Step, UpdateStep};
use crate::{contexts::login::use_login, core::types::recipe::UpdateRecipe};

use crate::modals::Modal;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditStepProps {
    pub len: usize,
    pub index: usize,
    pub step: Step,
    pub on_input: Callback<(usize, Step)>,
    pub on_move_up: Callback<usize>,
    pub on_move_down: Callback<usize>,
    pub on_delete: Callback<usize>,
}

#[function_component(EditStep)]
pub fn recipe_step(props: &EditStepProps) -> Html {
    let step_state = use_state(|| Step {
        title: None,
        description: "".to_owned(),
    });

    {
        let initial_step = props.step.clone();
        let step_state = step_state.clone();
        use_effect_with_deps(
            move |_| {
                step_state.set(initial_step);
            },
            props.step.clone(),
        );
    }

    let on_move_up = {
        let on_move_up_callback = props.on_move_up.clone();
        let index = props.index;
        Callback::from(move |_| {
            on_move_up_callback.emit(index);
        })
    };

    let on_move_down = {
        let on_move_down_callback = props.on_move_down.clone();
        let index = props.index;
        Callback::from(move |_| {
            on_move_down_callback.emit(index);
        })
    };

    let on_delete = {
        let on_delete_callback = props.on_delete.clone();
        let index = props.index;
        Callback::from(move |_| {
            on_delete_callback.emit(index);
        })
    };

    let on_title_input = {
        let on_input_callback = props.on_input.clone();
        let index = props.index;
        let step_state = step_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut step = (*step_state).clone();
                if input.value().is_empty() {
                    step.title = None;
                } else {
                    step.title = Some(input.value());
                }
                step_state.set(step.clone());
                on_input_callback.emit((index, step));
            }
        })
    };

    let on_description_input = {
        let on_input_callback = props.on_input.clone();
        let index = props.index;
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                let mut step = (*step_state).clone();
                step.description = input.value();
                step_state.set(step.clone());
                on_input_callback.emit((index, step));
            }
        })
    };

    html! {
        <li class="mb-4 p-4 rounded bg-base-200">
            <div class="flex mb-2">
                <input
                    class="input input-bordered w-full mr-2"
                    oninput={on_title_input}
                    value={props.step.title.clone().unwrap_or_default()}
                    type="text"
                    placeholder={format!("Step {}", props.index+1)}
                />
                <div class="btn-group">
                    {
                        if props.index == 0 {
                            html!{<button type="button" class="btn btn-disabled">{"Up"}</button>}
                        } else {
                            html!{<button type="button" class="btn" onclick={on_move_up}>{"Up"}</button>}
                        }
                    }
                    {
                        if props.len == props.index+1 {
                            html!{<button type="button" class="btn btn-disabled">{"Down"}</button>}
                        } else {
                            html!{<button type="button" class="btn" onclick={on_move_down}>{"Down"}</button>}
                        }
                    }
                    <button type="button" class="btn" onclick={on_delete}>{"X"}</button>
                </div>
            </div>
            <textarea
                class="textarea textarea-bordered w-full"
                oninput={on_description_input}
                value={props.step.description.clone()}
                placeholder="Description here..."
                required=true
            />
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct EditStepsProps {
    pub id: String,
    pub steps: Vec<Step>,
    pub onclose: Callback<Option<Vec<Step>>>,
}

#[function_component(EditSteps)]
pub fn recipe_steps(props: &EditStepsProps) -> Html {
    let login_ctx = use_login().unwrap();
    let toasts_ctx = use_toasts().unwrap();
    let steps_state = use_state(Vec::default);
    let is_loading_state = use_state(bool::default);

    {
        let initial_steps = props.steps.clone();
        let steps_state = steps_state.clone();
        use_effect_with_deps(
            move |_| {
                steps_state.set(initial_steps.clone());
            },
            (),
        );
    }

    let on_save = {
        let id = props.id.to_string();
        let on_close_callback = props.onclose.clone();
        let steps_state = steps_state.clone();
        let is_loading_state = is_loading_state.clone();
        Callback::from(move |_| {
            let login_ctx = login_ctx.clone();
            let api = login_ctx.http_api.clone().unwrap();
            let toasts_ctx = toasts_ctx.clone();
            let id = id.clone();
            let on_close_callback = on_close_callback.clone();
            let is_loading_state = is_loading_state.clone();
            let steps = (*steps_state).clone();
            wasm_bindgen_futures::spawn_local(async move {
                is_loading_state.set(true);
                let result = api
                    .patch_update_recipe(
                        id,
                        &UpdateRecipe {
                            steps: Some(
                                steps
                                    .iter()
                                    .map(|step| UpdateStep {
                                        title: step.title.clone(),
                                        description: Some(step.description.clone()),
                                    })
                                    .collect(),
                            ),
                            ..Default::default()
                        },
                    )
                    .await;
                is_loading_state.set(false);
                match result {
                    Ok(_) => {
                        on_close_callback.emit(Some(steps));
                    }
                    Err(e) => {
                        push_toast(&toasts_ctx, api_error_to_toast(&e, "saving recipe steps"));
                        logout_on_401(&e, &login_ctx);
                    }
                };
            });
        })
    };

    let on_cancel = {
        let on_close_callback = props.onclose.clone();
        Callback::from(move |_| {
            on_close_callback.emit(None);
        })
    };

    let on_step_input = {
        let steps_state = steps_state.clone();
        Callback::from(move |update: (usize, Step)| {
            let mut steps = (*steps_state).clone();
            steps[update.0] = update.1;
            steps_state.set(steps);
        })
    };

    let on_step_move_up = {
        let steps_state = steps_state.clone();
        Callback::from(move |index: usize| {
            if index != 0 {
                let mut steps = (*steps_state).clone();
                steps.swap(index, index - 1);
                steps_state.set(steps.clone());
            }
        })
    };

    let on_step_move_down = {
        let steps_state = steps_state.clone();
        Callback::from(move |index: usize| {
            let mut steps = (*steps_state).clone();
            if index != steps.len() - 1 {
                steps.swap(index, index + 1);
                steps_state.set(steps);
            }
        })
    };

    let on_delete_step = {
        let steps_state = steps_state.clone();
        Callback::from(move |index: usize| {
            let mut steps = (*steps_state).clone();
            steps.remove(index);
            steps_state.set(steps);
        })
    };

    let on_add_step = {
        let steps_state = steps_state.clone();
        Callback::from(move |_| {
            let mut steps = (*steps_state).clone();
            steps.push(Step {
                title: None,
                description: "".to_owned(),
            });
            steps_state.set(steps);
        })
    };

    html! {
        <Modal title={"Edit Steps"} oncancel={on_cancel} onsave={on_save} loading={*is_loading_state}>
            <div class="max-h-[50vh] lg:max-h-[60vh] overflow-y-auto">
                <ol>
                {
                    for (*steps_state).clone().iter().enumerate().map(|(i, step)| {
                        html!{<EditStep
                            len={(*steps_state).clone().len()}
                            index={i}
                            step={step.clone()}
                            on_input={on_step_input.clone()}
                            on_move_up={on_step_move_up.clone()}
                            on_move_down={on_step_move_down.clone()}
                            on_delete={on_delete_step.clone()}
                        />}
                    })
                }
                </ol>
                <button type="button" class="btn w-full" onclick={on_add_step}>{"Add Step"}</button>
            </div>
        </Modal>
    }
}
