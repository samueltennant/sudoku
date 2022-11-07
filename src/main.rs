use grid::Grid;
use yew::{html, Component, Context};

mod grid;

enum Msg {
    Start,
    Reset,
    // bool dictates wether won or not
    End(bool),
}

struct Model {
    grid: Option<Grid>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { grid: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Msg::AddOne => {
            //     self.value += 1;
            //     true
            // }
            _ => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        // let link = ctx.link();

        html! {
            <div>
            <h1>{"Hello, world!"}</h1>
            </div>
        }
    }
}

fn main() {
    // yew::start_app::<Model>();
    let grid = Grid::new();
}
