use crate::api::{self, User};
use leptos::server::Resource;
use leptos::{either::Either, prelude::*};
use leptos_router::{hooks::use_params_map, *};

#[component]
pub fn User() -> impl IntoView {
    let params = use_params_map();
    let user = Resource::new_serde(
        move || {
            params
                .read()
                .get("id")
                .map(ToOwned::to_owned)
                .unwrap_or_default()
        },
        move |id| async move {
            if id.is_empty() {
                None
            } else {
                api::fetch_api::<User>(&api::user(&id)).await
            }
        },
    );
    view! {
        <div class="user-view">
            <Suspense fallback=|| view! { "Loading..." }>
                {move || Suspend(async move { match user.await {
                    None => Either::Left(view! {  <h1>"User not found."</h1> }),
                    Some(user) => Either::Right(view! {
                        <div>
                            <h1>"User: " {user.id.clone()}</h1>
                            <ul class="meta">
                                <li>
                                    <span class="label">"Created: "</span> {user.created}
                                </li>
                                <li>
                                <span class="label">"Karma: "</span> {user.karma}
                                </li>
                                <li inner_html={user.about} class="about"></li>
                            </ul>
                            <p class="links">
                                <a href=format!("https://news.ycombinator.com/submitted?id={}", user.id)>"submissions"</a>
                                " | "
                                <a href=format!("https://news.ycombinator.com/threads?id={}", user.id)>"comments"</a>
                            </p>
                        </div>
                    })
                }})}
            </Suspense>
        </div>
    }
}
