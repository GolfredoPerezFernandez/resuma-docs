use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Examples"</h1>
            <p class="lead">
                "Runnable crates in "
                <code>"examples/"</code> " — clone and run with "
                <code>"cargo run -p PACKAGE"</code>". All listen on "
                <code>"http://127.0.0.1:3000"</code> " by default."
            </p>
            <p>
                "Documentation is served at "
                <a href="https://resuma-docs.fly.dev/" target="_blank">"resuma-docs.fly.dev"</a>
                " (source in "
                <code>"apps/resuma-docs"</code>", not an example crate)."
            </p>

            <table class="docs-table">
                <thead>
                    <tr>
                        <th>"Example"</th>
                        <th>"Command"</th>
                        <th>"App type"</th>
                        <th>"What it demonstrates"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><strong>"todo"</strong></td>
                        <td><code>"cargo run -p example-todo"</code></td>
                        <td>"ResumaApp"</td>
                        <td>
                            "Full showcase: signals, "
                            <code>"#[server]"</code> ", "
                            <code>"#[island]"</code> ", "
                            <code>"js!"</code> ", theme, backend security (guards, DTOs, service layer). "
                            <a href="/docs/security/todo">"Docs →"</a>
                        </td>
                    </tr>
                    <tr>
                        <td><strong>"counter"</strong></td>
                        <td><code>"cargo run -p example-counter"</code></td>
                        <td>"ResumaApp"</td>
                        <td>"Minimal resumable counter — smallest interactive app."</td>
                    </tr>
                    <tr>
                        <td><strong>"flow-demo"</strong></td>
                        <td><code>"cargo run -p example-flow-demo"</code></td>
                        <td>"FlowApp"</td>
                        <td>
                            <code>"#[load]"</code> ", streaming SSR, "
                            <code>"#[load(stream)]"</code> ", deferred chunks. "
                            <a href="/docs/flow/streaming">"Docs →"</a>
                        </td>
                    </tr>
                    <tr>
                        <td><strong>"flow-pages"</strong></td>
                        <td><code>"cargo run -p example-flow-pages"</code></td>
                        <td>"FlowApp"</td>
                        <td>
                            "File-based routing, layouts, "
                            <code>"auto_pages"</code> ", "
                            <code>"resuma routes --generate"</code>". "
                            <a href="/docs/flow/pages">"Docs →"</a>
                        </td>
                    </tr>
                    <tr>
                        <td><strong>"production template"</strong></td>
                        <td><code>"resuma new app --template production"</code></td>
                        <td>"FlowApp"</td>
                        <td>
                            "Security stub, Dockerfile, fly.toml, "
                            <code>"/ops"</code> " dashboard. "
                            <a href="/docs/exec/ops">"Docs →"</a>
                        </td>
                    </tr>
                </tbody>
            </table>

            <h2>"Choose an example"</h2>
            <ul>
                <li><strong>"Learning Resuma?"</strong>" → " <code>"counter"</code> " then " <code>"todo"</code></li>
                <li><strong>"Production backend patterns?"</strong>" → " <code>"todo"</code> " + " <a href="/docs/security">"Security docs"</a></li>
                <li><strong>"Multi-page site?"</strong>" → " <code>"flow-pages"</code> " or " <code>"resuma new --template flow"</code></li>
                <li><strong>"Bookings / calendars?"</strong>" → " <code>"resuma new --template flow-booking"</code> " + " <a href="/docs/flow/query_params">"Query params"</a></li>
                <li><strong>"Streaming / loaders?"</strong>" → " <code>"flow-demo"</code></li>
                <li><strong>"Full-stack + SQL?"</strong>" → " <code>"resuma new --template flow-fullstack"</code></li>
                <li><strong>"Production + ops dashboard?"</strong>" → " <code>"resuma new --template production"</code> " + " <a href="/docs/exec">"Resuma OS"</a></li>
            </ul>

            <h2>"Full app from scratch"</h2>
            <p>
                "This is the smoke-test shape we use for a fresh external project. It starts from the public CLI, then exercises routing, "
                <code>"NavLink"</code> ", " <code>"#[load]"</code> ", " <code>"#[submit]"</code> ", "
                <code>"#[server]"</code> ", " <code>"#[data]"</code> ", signals, client handlers, redirects, flash messages, and middleware."
            </p>
            {code_block(r#"cargo install resuma --version 1.3.1 --force
resuma new launchops --template basic
cd launchops

cargo check
cargo run
# open http://127.0.0.1:3000"#)}

            <h3>"src/main.rs pattern"</h3>
            {code_block(r#"use resuma::prelude::*;
use std::sync::{Mutex, OnceLock};

#[data]
struct Project {
    id: u32,
    name: String,
    owner: String,
    budget: u32,
}

#[data]
struct ProjectForm {
    name: String,
    owner: String,
    budget: String,
}

#[data]
struct PlanInput {
    topic: String,
    urgency: i32,
}

#[data]
struct PlanOutput {
    title: String,
    next_step: String,
    score: i32,
}

static PROJECTS: OnceLock<Mutex<Vec<Project>>> = OnceLock::new();

fn projects_store() -> &'static Mutex<Vec<Project>> {
    PROJECTS.get_or_init(|| {
        Mutex::new(vec![Project {
            id: 1,
            name: "Launch Analytics".into(),
            owner: "Ada".into(),
            budget: 2600,
        }])
    })
}

#[load]
async fn projects(_req: &FlowRequest) -> Vec<Project> {
    projects_store().lock().expect("project store lock").clone()
}

#[submit]
async fn add_project(form: ProjectForm, _req: &FlowRequest) -> std::result::Result<Redirect, SubmitError> {
    if form.name.trim().len() < 3 {
        return Err(SubmitError::new("Fix the project").field("name", "Use at least 3 characters"));
    }

    let budget: u32 = form.budget.trim().parse()
        .map_err(|_| SubmitError::new("Fix the project").field("budget", "Budget must be a number"))?;

    let mut projects = projects_store().lock().expect("project store lock");
    let id = projects.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    projects.push(Project {
        id,
        name: form.name.trim().into(),
        owner: form.owner.trim().into(),
        budget,
    });

    Ok(redirect_with_flash("/projects", "Project created"))
}

#[server]
async fn generate_plan(input: PlanInput) -> Result<PlanOutput> {
    if input.topic.trim().is_empty() {
        return Err(ResumaError::validation("topic is required"));
    }

    let score = (input.urgency * 17).clamp(10, 100);
    Ok(PlanOutput {
        title: format!("Plan for {}", input.topic.trim()),
        next_step: "Create a focused implementation checklist".into(),
        score,
    })
}

#[middleware]
async fn request_log(req: FlowRequest) -> resuma::Result<FlowRequest> {
    println!("[launchops] {} {}", req.method, req.path);
    Ok(req)
}

fn nav() -> View {
    view! {
        <nav>
            <NavLink href="/" activeClass="active">"Dashboard"</NavLink>
            <NavLink href="/projects" activeClass="active">"Projects"</NavLink>
            <NavLink href="/workbench" activeClass="active">"Workbench"</NavLink>
        </nav>
    }
}

#[component]
fn ProjectsPage() {
    let rows = use_projects_load();
    let flash = current_request().and_then(|req| flash_message(&req));

    view! {
        <main>
            {nav()}
            <h1>"Projects"</h1>
            {flash.map(|msg| view! { <p class="flash">{msg}</p> }).unwrap_or(View::Empty)}
            <section>
                {rows.iter().map(|project| view! {
                    <article key={project.id.to_string()}>
                        <h2>{project.name.clone()}</h2>
                        <p>{format!("Owner: {} - Budget: ${}", project.owner, project.budget)}</p>
                    </article>
                }).collect::<Vec<_>>()}
            </section>
            <Form submit={add_project}>
                <input name="name" placeholder="Mobile dashboard" />
                <input name="owner" placeholder="Ada" />
                <input name="budget" placeholder="3200" inputmode="numeric" />
                <button type="submit">"Create"</button>
            </Form>
        </main>
    }
}

#[component]
fn WorkbenchPage() {
    let urgency = signal(3_i32);
    let topic = signal("release readiness".to_string());
    let result = signal("No plan generated yet".to_string());

    view! {
        <main>
            {nav()}
            <h1>"Workbench"</h1>
            <p>"Urgency: " {urgency}</p>
            <button type="button" onClick={js! {
                state.urgency.set(Math.max(1, state.urgency.value - 1));
            }}>"-"</button>
            <button type="button" onClick={js! {
                state.urgency.set(Math.min(5, state.urgency.value + 1));
            }}>"+"</button>

            <input value={topic.get()} onInput={js! { state.topic.set(event.target.value); }} />
            <button type="button" onClick={js! {
                const plan = await __resuma.action("generate_plan", [{
                    topic: state.topic.value,
                    urgency: state.urgency.value,
                }]);
                state.result.set(plan.title + ": " + plan.next_step + " (" + plan.score + ")");
            }}>"Ask server"</button>
            <p>{result}</p>
        </main>
    }
}

#[component]
fn DashboardPage() {
    view! {
        <main>
            {nav()}
            <h1>"LaunchOps"</h1>
            <p>"Flow routes, loaders, submits, server actions, signals, SPA navigation, and SSR."</p>
        </main>
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    FlowApp::new()
        .with_title("LaunchOps")
        .component("/", DashboardPage)
        .component("/projects", ProjectsPage)
        .component("/workbench", WorkbenchPage)
        .serve(FlowServeOptions::default())
        .await
}"#)}

            <h2>"CLI templates"</h2>
            <p>
                <code>"resuma new my-app --template basic"</code> " scaffolds like a minimal "
                <code>"counter"</code>". "
                <code>"--template todo"</code> " copies the " <code>"todo"</code> " example (main + security + todo_store)."
            </p>
        </>
    }
}
