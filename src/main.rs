use yew::{function_component, html, Callback, Properties};

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {

    html! {
        <main>
           <VideoControls/>
           <VideoSection name="nombre video" id="tmYhb0efRIw"/>
        </main>

    }
}

struct VideoControlProps{
    on_search: Callback<String>,
}
#[function_component(VideoControls)]
fn controls() -> Html {
    let handle_input = Callback::from(|_| {});

    html! {
       <div>
       <div>
            { "Ingresa Una Palabra" }
        </div>
            <div>
                <input type="text" oninput={handle_input}/>
            </div>
            <div><button>{"buscar!"}</button></div>
            
       </div>

    }
}

#[derive(Properties, PartialEq)]
struct VideoSectionProps {
    id: String,
    name: String,
}

#[function_component(VideoSection)]
fn video_section(props: &VideoSectionProps) -> Html {
    let yt_url = format!("https://www.youtube.com/embed/{}", props.id);
    html! {
        <div>
            <iframe width="560" height="315" src={yt_url} frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"></iframe>
        </div>
    }
}