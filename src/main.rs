use leptos::*;
use leptos_router::*;

mod pages;
use pages::projects;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="root box">
                <h1>
                    "Vaurkhorov's Page"
                </h1>
                <ul class="nav box">
                    <li><a href="/">"Home"</a></li>
                    <li><a href="/projects">"Projects"</a></li>
                    <li><a href="/about">"About"</a></li>
                </ul>
                <main>
                    <Routes>
                        <Route path="/" view=|| view!{
                            "this is my page. take a look around?"
                        }/>
                        <Route path="/projects" view=projects::Projects>
                            <Route path="" view=|| view!{}/>
                            <Route path="/:id" view=projects::Project/>
                        </Route>
                        <Route path="/about" view=|| view!{
                            <p>"this is another placeholder. i'll probably put my instagram here. ooh and linkedin too. and github. a resume. my email. the other email."</p>
                            <p>"and a partridge in a pear tree."</p>
                        }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}