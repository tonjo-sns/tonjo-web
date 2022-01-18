use base64::{encode_config, URL_SAFE};
use tonjo_sns_client::*;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

struct Post {
    update: Vec<u8>,
}
#[derive(Properties, PartialEq)]
struct PostProps {
    #[prop_or_default]
    update: Vec<u8>,
}

impl Component for Post {
    type Message = NoneMsg;
    type Properties = PostProps;
    fn create(ctx: &Context<Self>) -> Self {
        Post {
            update: ctx.props().update.clone(),
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let update = Update::load(&self.update);
        html! {
            <>
                <h6>{ encode_config(update.author_id().as_slice(), URL_SAFE) }</h6>
                <div>{ std::str::from_utf8(&update.filebytes()).unwrap_or("") }</div>
                <hr />
            </>
        }
    }
}

enum NoneMsg {}

struct Main {
    value: Vec<Vec<u8>>,
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
            value: Default::default(),
            account,
            post_input: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MainMsg::PostString(post) => {
                let new_post = self.account.post(&post);
                self.value.push(new_post);
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
            {
                for self.value.iter().filter(|e| {
                    let update = Update::load(e);
                    update.filename() == POST_FILETYPE
                }).map(|post| html!(<Post update={post.clone()} />))
            }
            </main>
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
