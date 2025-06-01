use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let icon: Html = html! {
        <img class="icon" src="images/markdown.png" />
    };

    let footer: Html = html! {
        <footer role="contentinfo" class="center">
            <a href={"https://www.flaticon.com/free-icons/markdown"}
                title="markdown icons">
                {"Markdown icons created by Muhammad Andy - Flaticon"}
            </a>
        </footer>
    };

    html! {
        <>
            <div class="center">
                {icon}
            </div>
            {footer}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
