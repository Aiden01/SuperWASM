use yew::prelude::*;
use yew::services::FetchService;



pub struct Post {
    title: String,
    body: String
}

pub struct PostsComp {
    pub posts: Vec<Post>,
    pub fetchAPI: FetchService
}

pub enum Msg {
    AddPost
}

impl Component for PostsComp {

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        PostsComp{
            posts: vec!(Post{title: "Lorem ipsum".to_string(), body: "Lorem ipsum this is a body, so its a bit longer".to_string()}),
            fetchAPI: FetchService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddPost => {
                self.posts.push(Post{title: String::from("Lorem ipsum"), body: String::from("Lorem ipsum this is a body, so its a bit longer")});
                true
            },
            _ => false
        }
    }

}

impl Renderable<PostsComp> for PostsComp {
    fn view(&self) -> Html<Self> {
        html! {
            <h2 class="text-center text-grey-darkest",>{ "Articles" }</h2>
        }
    }
}