use yew::prelude::*;

// pub enum Msg {
//     Add,
//     Sub,
// }

// pub struct CounterPage {
//     counter: i32,
// }

// impl Component for CounterPage {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self { counter: 0 }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::Add => {
//                 self.counter += 1;
//                 true
//             }
//             Msg::Sub => {
//                 self.counter -= 1;
//                 true
//             }
//         }
//     }
//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             <div>
//                 <h1>{ "Counter测试" }</h1>
//                 <div>{"counter: "}{self.counter}</div>
//                 <div>
//                 <button onclick={ctx.link().callback(move|_|Msg::Add)}>{ "增加" }</button>
//                 <button onclick={ctx.link().callback(move|_|Msg::Sub)}>{ "减少" }</button>
//                 </div>
//             </div>
//         }
//     }
// }

#[function_component]
pub fn CounterPage() -> Html {
    let counter = use_state(|| 0);

    let add_callback = {
        let counter_clone = counter.clone();
        Callback::from(move |_| {
            counter_clone.set(*counter_clone+1);
        })
    };

    let sub_callback = {
        let counter_clone = counter.clone();
        Callback::from(move |_| {
            counter_clone.set(*counter_clone-1);
        })
    };

    html! {
     <div>
        <h1>{ "Counter测试" }</h1>
        <div>{"counter: "}{*counter}</div>
        <div>
            <button onclick={add_callback}>{ "增加" }</button>
            <button onclick={sub_callback}>{ "减少" }</button>
        </div>
    </div>
    }
}
