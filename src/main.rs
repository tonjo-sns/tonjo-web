//! tonjo-sns のウェブアプリ。

use std::collections::VecDeque;

use tonjo_sns_client::*;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

mod components;

use components::posts_list::PostsList;

#[function_component(MainPage)]
fn main_page() -> Html {
    let account_state = use_state(|| Account::new().save_state());
    let posts = use_state(|| VecDeque::<Vec<u8>>::new());
    let draftarea_ref = use_node_ref();
    let handle_post = {
        let draftarea_ref = draftarea_ref.clone();
        let posts = posts.clone();
        let account_state = account_state.clone();
        move |_| {
            if let Some(draftarea_ref) = draftarea_ref.cast::<HtmlTextAreaElement>() {
                let mut account = Account::load_state((*account_state).clone());
                let post_string = draftarea_ref.value();
                let post_vec = account.post(&post_string);
                draftarea_ref.set_value("");
                let mut ps = (*posts).clone();
                ps.push_front(post_vec);
                posts.set(ps)
            }
        }
    };
    html! {
        <main>
            <textarea ref={draftarea_ref}/>
            <button onclick={handle_post}>{ "Post" }</button>
            <PostsList posts={(*posts).clone()} />
        </main>
    }
}

fn main() {
    yew::start_app::<MainPage>();
}
