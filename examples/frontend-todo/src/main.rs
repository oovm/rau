use wae_client::prelude::*;

#[component]
fn App() -> Element {
    let todos = signal(vec!["Learn Rust".to_string(), "Build wae".to_string()]);

    html! {
        <div class="p-4">
            <h1 class="text-xl font-bold">Todo List</h1>
            <ul>
                {todos.get().iter().map(|todo| {
                    html! { <li>{todo}</li> }
                }).collect::<Vec<_>>()}
            </ul>
            <button 
                class="mt-4 bg-blue-500 text-white px-4 py-2 rounded"
                on:click={move || {
                    let mut new_todos = todos.get();
                    new_todos.push("New item".to_string());
                    todos.set(new_todos);
                }}
            >
                "Add"
            </button>
        </div>
    }
}

fn main() {
    wae_client::start(App);
}
