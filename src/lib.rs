
#[macro_use]
extern crate yew;

pub mod components;

use yew::services::ConsoleService;
use yew::prelude::*;
use self::components::header::HeaderComp;
use self::components::posts::PostsComp;

pub struct Model {
    console: ConsoleService,
    value: i32
}

pub enum Msg {
    Ok
}

impl Component for Model {

    type Properties = ();
    type Message = Msg; 

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            value: 0,
            console: ConsoleService::new()
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.console.log("Hello world");
        true
    }

}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <HeaderComp: />
                <PostsComp: />
            </div>
        }
    }
}