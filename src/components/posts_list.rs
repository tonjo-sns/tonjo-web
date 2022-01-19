use base64::{encode_config, URL_SAFE};
use std::collections::VecDeque;
use tonjo_sns_client::Update;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PostProps {
    pub update: Vec<u8>,
}
#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let update = Update::load(&props.update);
    html! {
        <>
            <div>
                <h6>{ encode_config(update.author_id().as_slice(), URL_SAFE) }</h6>
                <div>{ std::str::from_utf8(&update.filebytes()).unwrap_or("") }</div>
            </div>
            <hr />
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct PostsListProps {
    pub posts: VecDeque<Vec<u8>>,
}
#[function_component(PostsList)]
pub fn posts_list(props: &PostsListProps) -> Html {
    html!(<>{
        for props.posts.iter().map(|post| html!(<Post update={post.clone()} />))
    }</>)
}
