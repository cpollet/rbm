mod bookmark_provider;
mod bookmarks;
mod create_bookmark;
mod data;
mod delete_bookmark;
mod edit_bookmark;
mod login;
mod logout;
mod menu;
mod signup;
mod signup_success;
mod validate_email;

use crate::bookmark_provider::BookmarkProvider;
use crate::bookmarks::bookmark::BookmarkHOC;
use crate::bookmarks::bookmarks_provider::BookmarksProvider;
use crate::bookmarks::BookmarksHOC;
use crate::create_bookmark::CreateBookmark;
use crate::delete_bookmark::DeleteBookmarkHOC;
use crate::edit_bookmark::EditBookmarkHOC;
use crate::menu::Menu;
use crate::signup::Signup;
use crate::signup_success::SignupSuccess;
use crate::validate_email::ValidateEmail;
use gloo_net::http::Request;
use login::Login;
use logout::Logout;
use rest_api::application::{GetApplicationResult, URL_APPLICATION};
use rest_api::sessions::{CreateSessionResult, URL_SESSIONS_CURRENT};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("/bookmarks")]
    Bookmarks,

    #[at("/bookmarks/~add")]
    AddBookmark,

    #[at("/bookmarks/:id")]
    ViewBookmark { id: i32 },

    #[at("/bookmarks/:id/~delete")]
    DeleteBookmark { id: i32 },

    #[at("/bookmarks/:id/~edit")]
    EditBookmark { id: i32 },

    #[at("/tags")]
    TagCloud,

    #[at("/tools")]
    Tools,

    #[at("/signup")]
    Signup,

    #[at("/signup/success")]
    SignupSuccess,

    #[at("/login")]
    Login,

    #[at("/email/:uuid/~validate")]
    ValidateEmail { uuid: uuid::Uuid },

    #[at("/logout")]
    Logout,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Default)]
struct State {
    username: Option<AttrValue>,
    commit: Option<AttrValue>,
    build_date: Option<AttrValue>,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(State::default);

    {
        let state = state.clone();
        use_effect_once(move || {
            let mut new_state = (*state).clone();
            spawn_local(async move {
                if let Some(CreateSessionResult::Success(session)) =
                    CreateSessionResult::from(Request::get(URL_SESSIONS_CURRENT).send().await).await
                {
                    new_state.username = Some(AttrValue::from(session.username));
                }
                if let Some(GetApplicationResult::Success(application)) =
                    GetApplicationResult::from(Request::get(URL_APPLICATION).send().await).await
                {
                    new_state.commit = Some(AttrValue::from(application.commit));
                    new_state.build_date = Some(AttrValue::from(application.build_date));
                }
                state.set(new_state);
            });

            || {}
        });
    }

    let onlogin = {
        let state = state.clone();
        Callback::from(move |username: AttrValue| {
            let mut new_state = (*state).clone();
            new_state.username = Some(username);
            state.set(new_state);
        })
    };
    let onlogout = {
        let state = state.clone();
        Callback::from(move |_: ()| {
            let mut new_state = (*state).clone();
            new_state.username = None;
            state.set(new_state);
        })
    };

    html! {
        <>
            <BrowserRouter>
                <Menu username={state.username.clone()} />
                <a
                    class="github-fork-ribbon right-bottom fixed"
                    href="https://github.com/cpollet/rbm"
                    data-ribbon="Fork me on GitHub"
                    title="Fork me on GitHub"
                >
                    {"Fork me on GitHub"}
                </a>
                <div class="content">
                    <Switch<Route> render={move |route| match route {
                        Route::Index | Route::Bookmarks => {
                            html! {
                                <BookmarksProvider>
                                    <BookmarksHOC />
                                </BookmarksProvider>
                            }
                        }
                        Route::AddBookmark => {
                            // todo prevent display when not logged
                            html! {
                                <CreateBookmark />
                            }
                        }
                        Route::ViewBookmark { id } => {
                            html! {
                                <BookmarkProvider {id}>
                                    <BookmarkHOC />
                                </BookmarkProvider>
                            }
                        }
                        Route::DeleteBookmark { id } => {
                            // todo prevent display when not logged
                            html! {
                                <BookmarkProvider {id}>
                                    <DeleteBookmarkHOC />
                                </BookmarkProvider>
                            }
                        }
                        Route::EditBookmark { id } => {
                            // todo prevent display when not logged
                            html! {
                                <BookmarkProvider {id}>
                                    <EditBookmarkHOC />
                                </BookmarkProvider>
                            }
                        }
                        Route::TagCloud => {
                            html! {
                                {"todo: tag cloud"}
                            }
                        }
                        Route::Tools => {
                            // todo prevent display when not logged
                            html! {
                                {"todo: tools"}
                            }
                        }
                        Route::Signup => {
                            html! {
                                <Signup />
                            }
                        }
                        Route::SignupSuccess => {
                            html! {
                                <SignupSuccess />
                            }
                        }
                        Route::Login => {
                            html! {
                                <Login onlogin={onlogin.clone()} />
                            }
                        }
                        Route::ValidateEmail { uuid } => {
                            html! {
                                <ValidateEmail {uuid} />
                            }
                        }
                        Route::Logout => {
                            html! {
                                <Logout onlogout={onlogout.clone()} />
                            }
                        }
                        Route::NotFound => {
                            html! {
                                <h1>{"404 Not Found"}</h1>
                            }
                        }
                    } } />
                </div>
            </BrowserRouter>
            <div class="footer">
                <a
                    href={format!("https://github.com/cpollet/rbm/tree/{}",
                                  state.commit
                                    .as_ref()
                                    .map(AttrValue::to_string)
                                    .unwrap_or_else(|| "master".to_string())
                         )}
                >
                    {state.commit.clone()}
                </a>
                {" · "} {state.build_date.clone()}
            </div>
        </>
    }
}
