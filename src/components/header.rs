
use yew::prelude::*;

pub struct Model;

pub enum Msg {
    Ok
}

impl Component for Model {

    type Properties = ();
    type Message = Msg; 

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model{}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <h1>{ "asdasdaasd" }</h1>
            <div class="absolute pin-t h-2 w-full bg-blue",></div>
        }
    }
}