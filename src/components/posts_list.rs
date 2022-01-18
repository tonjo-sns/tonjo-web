use yew::prelude::*;
use std::collections::VecDeque;
use super::post::Post;

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
