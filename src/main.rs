use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GreetingProps {
    pub name: String,
}

#[function_component(Greeting)]
fn greeting(props: &GreetingProps) -> Html {
    html! {
        <div>
            <h1 class=" text-green-500">{ format!("Hello, {}!", props.name) }</h1>
        </div>
    }
}


#[function_component]
fn App() -> Html {
  let names = vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()];

  let counter = use_state(|| 0);
  let onclick = {
    let counter = counter.clone();
    move |_| {
      let value = *counter + 1;
      counter.set(value);
    }
  };

  html! {
    <div>
      <button {onclick}>{ "+2" }</button>
      <p>{ *counter }</p>
      { 
                for names.iter().map(|name| html! {
                    <Greeting name={name.clone()} />
                })
            }
    </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
