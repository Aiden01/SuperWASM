use yew::prelude::*;



pub struct Post {
    title: String,
    body: String
}

pub struct PostsComp {
    pub posts: Vec<Post>
}

pub enum Msg {
    AddPost
}

impl Component for PostsComp {

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        PostsComp{
            posts: vec!(Post{title: "Lorem ipsum".to_string(), body: "Lorem ipsum this is a body, so its a bit longer".to_string()})
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
            <div class="posts",>
                <div class="container mx-auto",>
                    <button class="px-5 py-2 bg-blue text-white font-semibold", onclick=|_| Msg::AddPost,>{ "Click me" }</button>
                    { self.format_posts() }
                </div>
            </div>
        }
    }
}

#[derive(Debug)]
pub struct PostComp{
    title: String,
    body: String
}

#[derive(PartialEq, Clone)]
pub struct PropsPostComp{
    title: String,
    body: String
}

impl Default for PropsPostComp {
    fn default() -> Self {
        PropsPostComp{
            title: String::new(),
            body: String::new()
        }
    }
}

impl Component for PostComp {

    type Message = ();
    type Properties = PropsPostComp;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        PostComp{
            title: props.title,
            body: props.body
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

}

impl Renderable<PostComp> for PostComp {
    fn view(&self) -> Html<Self> {
        html! {
        <div class="mx-auto p-10 bg-white shadow",>
            <h1 class="text-grey-darkest font-bold",>{ self.title }</h1>
        </div>
        }
    }
}

impl PostsComp {

    fn render(&self, post: Post) -> Html<Self> {
        html! {
            <PostComp: title=post.title, body=post.body, />
        }
    }

    fn format_posts(&self) -> Html<Self> {

        html! {
            <div>
                { for self.posts.iter().map(move |&p| self.render(p)) }
            </div>
        }
    }
}