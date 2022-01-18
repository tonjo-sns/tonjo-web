//! tonjo-sns のウェブアプリ。

use std::collections::VecDeque;

use tonjo_sns_client::*;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

mod components;

use components::posts_list::PostsList;

enum NoneMsg {}

struct Main {
    posts: VecDeque<Vec<u8>>,
    account: Account,
    post_input: NodeRef,
}

enum MainMsg {
    PostString(String),
}

impl Component for Main {
    type Message = MainMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let account = Account::new();
        Self {
            posts: Default::default(),
            account,
            post_input: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MainMsg::PostString(post) => {
                let new_post = self.account.post(&post);
                self.posts.push_back(new_post);
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let my_input_ref = self.post_input.clone();
        let post = link.callback(move |_| {
            let input = my_input_ref.cast::<HtmlTextAreaElement>();
            MainMsg::PostString(input.unwrap().value())
        });

        html! {
            <main>
                <textarea ref={self.post_input.clone()}/>
                <button onclick={post}>{ "Post" }</button>
                <PostsList posts={self.posts.clone()} />
            </main>
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
