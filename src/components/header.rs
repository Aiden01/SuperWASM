
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
            <div class="header",>
                <div class="absolute pin-t h-2 w-full bg-blue",></div>
                <h1 class="text-grey-darkest text-center mt-12 text-5xl font-thin",>{ "Super" }<span class="font-black",>{ "WASM" }</span></h1>
                <p class="text-center text-grey-darkest mt-5",>{ "Some tests with WebAssembly and Rust" }</p>
            </div>
        }
    }
}