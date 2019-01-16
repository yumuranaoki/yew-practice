extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    page: Page,
}

pub enum Msg {
    SwitchTo(Page),
}

pub enum Page {
    Main,
    Sub,
}

fn switch(model: & mut Model, page: Page) {
    model.page = page;
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            page: Page::Main,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SwitchTo(Page::Main) => {
                switch(self, Page::Main);
            }
            Msg::SwitchTo(Page::Sub) => {
                switch(self, Page::Sub);
            }
        }
        true
    }
}


impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        match self.page {
            Page::Main => html! {
                <div id="container",>
                    <nav class="menu",>
                        <button onclick=|_| Msg::SwitchTo(Page::Main),>{ "Main" }</button>
                        <button onclick=|_| Msg::SwitchTo(Page::Sub),>{ "Sub" }</button>
                    </nav>
                    <p>{ "Main" }</p>
                </div>
            },
            Page::Sub => html! {
                <div id="container",>
                    <nav class="menu",>
                        <button onclick=|_| Msg::SwitchTo(Page::Main),>{ "Main" }</button>
                        <button onclick=|_| Msg::SwitchTo(Page::Sub),>{ "Sub" }</button>
                    </nav>
                    <p>{ "Sub" }</p>
                </div>
            },
        }
    }
}