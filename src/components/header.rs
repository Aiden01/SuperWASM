
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct HeaderComp {
    console: ConsoleService
}


impl Component for HeaderComp {

    type Properties = ();
    type Message = (); 

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        HeaderComp{
            console: ConsoleService::new()
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.console.log("hello world");
        true
    }
}

impl Renderable<HeaderComp> for HeaderComp {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="absolute pin-t h-2 w-full bg-blue",></div>
        }
    }
}