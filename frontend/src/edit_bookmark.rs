use crate::data::Bookmark;
use crate::Route;
use gloo_net::http::Request;
use rest_api::{BookmarkResponse, UpdateBookmarkRequest, URL_BOOKMARK};
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone, Copy)]
pub struct Props {
    pub id: i32,
}

#[derive(Clone, PartialEq)]
struct State {
    bookmark: Bookmark,
}

#[function_component(EditBookmark)]
pub fn edit_bookmark(props: &Props) -> Html {
    let state = use_state(|| None);
    let navigator = use_navigator().unwrap();

    {
        let state = state.clone();
        let props = props.clone();
        use_effect(move || {
            if state.is_none() {
                spawn_local(async move {
                    let res = fetch_bookmark(props.id).await;
                    // todo implement a 500 page
                    if let Ok(bookmark) = res {
                        state.set(Some(State { bookmark }))
                    }
                });
            }

            || {}
        });
    }

    let onsubmit = {
        let state = state.clone();
        let props = props.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(state) = (*state).clone() {
                let navigator = navigator.clone();
                spawn_local(async move {
                    // TODO finish this
                    let _todo = Request::put(&URL_BOOKMARK.replace(":id", &props.id.to_string()))
                        .json(&UpdateBookmarkRequest::from(&state.bookmark))
                        .expect("could not set json")
                        .send()
                        .await;
                    navigator.push(&Route::Bookmarks);
                })
            }
        })
    };
    let onclick_cancel = {
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            navigator.push(&Route::Bookmarks);
        })
    };

    let oninput_url = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(mut new_state) = (*state).clone() {
                let input: HtmlInputElement = e.target_unchecked_into();
                new_state.bookmark.url = AttrValue::from(input.value());
                state.set(Some(new_state));
            }
        })
    };
    let oninput_title = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(mut new_state) = (*state).clone() {
                let value = e
                    .target_unchecked_into::<HtmlInputElement>()
                    .value()
                    .to_string();
                if value.is_empty() {
                    new_state.bookmark.title = None
                } else {
                    new_state.bookmark.title = Some(AttrValue::from(value))
                }
                state.set(Some(new_state));
            }
        })
    };
    let oninput_description = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(mut new_state) = (*state).clone() {
                let value = e
                    .target_unchecked_into::<HtmlInputElement>()
                    .value()
                    .to_string();
                if value.is_empty() {
                    new_state.bookmark.description = None
                } else {
                    new_state.bookmark.description = Some(AttrValue::from(value))
                }
                state.set(Some(new_state));
            }
        })
    };

    html! {
        <div class="edit-bookmark">
            <h1 class="edit-bookmark__title">{"Edit bookmark"}</h1>
            if let Some(state) = &*state {
                <form {onsubmit}>
                    <p>{"URL"}</p>
                    <p>
                        <input
                            class="edit-bookmark__url-input"
                            type="text"
                            value={state.bookmark.url.clone()}
                            oninput={oninput_url}
                        />
                    </p>
                    <p>{"Title"}</p>
                    <p>
                        <input
                            class="edit-bookmark__title-input"
                            type="text"
                            value={state.bookmark.title.clone()}
                            oninput={oninput_title}
                        />
                    </p>
                    <p>{"Description"}</p>
                    <p>
                        <textarea
                            class="edit-bookmark__description-input"
                            value={state.bookmark.description.clone()}
                            oninput={oninput_description}
                        />
                    </p>
                    <p>
                        <button type="button" onclick={onclick_cancel} class="edit-bookmark__submit--safe">{"Cancel"}</button>
                        {" "}
                        <button type="submit" class="edit-bookmark__submit--action">{"Update"}</button>
                    </p>
                </form>
            }
        </div>
    }
}

async fn fetch_bookmark(id: i32) -> Result<Bookmark, String> {
    match Request::get(&URL_BOOKMARK.replace(":id", &id.to_string()))
        .send()
        .await
    {
        Err(_) => Err("Error fetching data".to_string()),
        Ok(resp) => {
            if !resp.ok() {
                Err(format!(
                    "Error fetching data: {} ({})",
                    resp.status(),
                    resp.status_text()
                ))
            } else {
                resp.json::<BookmarkResponse>()
                    .await
                    .map_err(|err| err.to_string())
                    .map(Bookmark::from)
            }
        }
    }
}
