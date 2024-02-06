use leptos::*;
use leptos_router::{use_params_map, Outlet};

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="container box">
            <h1>
                "My Projects"
            </h1>
            <p>
                "these are the projects i've been working on"
            </p>
            <nav>
                
            </nav>
            <Outlet/>
        </div>
    }
}

#[component]
pub fn Project() -> impl IntoView {
    let params = use_params_map();
    let project_id = move || params.with(|params| params.get("id").cloned().unwrap_or(String::from("None")));

    view! {
        <h1>
            {project_id}
        </h1>
        <p>
            "descriptions don't work yet"
        </p>
    }
}