use yew::prelude::*;

#[function_component]
fn App() -> Html {
  html! {
    <div class="min-h-screen text-white" style="font-family: 'Fjalla One', sans-serif;">
      <div 
        class="bg-gradient-to-t from-gray-900 to-gray-600 w-full h-screen flex flex-col gap-4 justify-center items-center p-8"
      >
        <div class="w-full flex sm:justify-center">
          <div class="flex flex-row items-center gap-2">
            <img src="img/JFKongphop.jpg" class="w-8 h-8 rounded-full" /> 

            <p class="text-xl">{"JFKongphop"}</p>
          </div>
        </div>
        <div 
          class="flex flex-col gap-4 w-1/2 text-center max-sm:w-full max-sm:text-start max-sm:text-3xl"
        >
          <p 
            class="text-5xl leading-14 max-sm:text-3xl max-sm:leading-10"
          >
            <p>{"Enthusiastic about"} </p>
            <p>{"Blockchain Application,"}</p>
            <p>{"Full-Stack, and Distance Running"} </p>
          </p>
          <p class="text-lg opacity-50">
            {"I’m pursuing studies in Financial Engineering but have a strong interest in Computer Science, particularly in Blockchain technology and Full-Stack development. I’m also passionate about track distance running."}
          </p>
        </div>
      </div>
    </div>

  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
