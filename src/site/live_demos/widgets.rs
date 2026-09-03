//! Reusable interactive widgets for documentation live demos.

use resuma::prelude::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn CounterWidget() -> View {
    let count = signal(0_i32);
    view! {
        <>
            <p class="demo-output">"Count: " {count}</p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={count.update(|c| *c -= 1)}>"−"</button>
                <button type="button" class="btn btn-sm" onClick={count.update(|c| *c += 1)}>"+"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={count.set(0)}>"reset"</button>
            </div>
        </>
    }
}

#[component]
pub fn ShowWidget() -> View {
    let logged_in = signal(false);
    view! {
        <>
            <Show when={logged_in.get()}>
                <p class="demo-output">"Welcome back!"</p>
            </Show>
            <Show when={!logged_in.get()} fallback={view! { <p class="demo-muted">"Sign in to continue"</p> }}>
                <span></span>
            </Show>
            <button type="button" class="btn btn-sm" onClick={logged_in.update(|v| *v = !*v)}>
                <Show when={logged_in.get()}><span>"Logout"</span></Show>
                <Show when={!logged_in.get()}><span>"Login"</span></Show>
            </button>
        </>
    }
}

#[component]
pub fn EffectsWidget() -> View {
    let first = signal("Ada".to_string());
    let last = signal("Lovelace".to_string());
    let display = signal("Ada Lovelace".to_string());
    effect!([first, last, display], move || {
        display.set(format!("{} {}", first.get(), last.get()));
    });
    view! {
        <>
            <div class="demo-row">
                <input placeholder="First" onInput={js! { state.first.set(event.target.value); }} />
                <input placeholder="Last" onInput={js! { state.last.set(event.target.value); }} />
            </div>
            <p class="demo-output">"Full name: " {display}</p>
        </>
    }
}

#[component]
pub fn ErrorBoundaryWidget() -> View {
    let boom = signal(false);
    view! {
        <>
            <Show when={boom} fallback={view! { <p class="demo-muted">"All good — click to trigger"</p> }}>
                {resuma::error_boundary(Err::<View, &str>("Something broke!"), |e| {
                    view! { <p class="demo-error">{e}</p> }
                })}
            </Show>
            <button type="button" class="btn btn-sm btn-ghost" onClick={boom.set(true)}>"Trigger error"</button>
        </>
    }
}

#[component]
pub fn HandlersWidget() -> View {
    let clicked = signal(0_i32);
    view! {
        <>
            <p class="demo-output">"Clicks: " {clicked}</p>
            <button type="button" class="btn btn-sm" onClick={clicked.update(|c| *c += 1)}>"Click me"</button>
        </>
    }
}

#[island]
pub fn island_demo() -> View {
    let n = signal(0_i32);
    view! {
        <>
            <p class="demo-output">"Island counter: " {n}</p>
            <button type="button" class="btn btn-sm" onClick={n.update(|v| *v += 1)}>"+"</button>
        </>
    }
}

#[component]
pub fn ServerActionWidget() -> View {
    let result = signal(String::new());
    let error = signal(String::new());
    let has_error = computed!([error], move || !error.get().is_empty());
    view! {
        <>
            <p class="demo-muted">
                "Real "
                <code>"#[server]"</code>
                " RPC via "
                <code>"__resuma.safeAction"</code>
                " — errors return "
                <code>"{ ok: false, error }"</code>
                " instead of throwing."
            </p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_echo", ["Hello from docs"]);
                    if (res.ok) {
                        state.result.set(res.value);
                        state.error.set("");
                    } else {
                        state.error.set(res.error);
                        state.result.set("");
                    }
                })}>"docs_echo"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_add", [2, 40]);
                    if (res.ok) {
                        state.result.set("2 + 40 = " + res.value);
                        state.error.set("");
                    } else {
                        state.error.set(res.error);
                        state.result.set("");
                    }
                })}>"docs_add(2,40)"</button>
            </div>
            <p class="demo-output">{result}</p>
            <Show when={has_error}>
                <p class="demo-alert demo-alert--error" role="alert">{error}</p>
            </Show>
        </>
    }
}

#[component]
pub fn SecurityServerActionWidget() -> View {
    let result = signal(String::new());
    let error = signal(String::new());
    let has_error = computed!([error], move || !error.get().is_empty());
    view! {
        <>
            <p class="demo-muted">
                "Same RPC as above, plus a validation failure (422) via "
                <code>"docs_todo_add"</code>
                "."
            </p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_echo", ["Hello from docs"]);
                    if (res.ok) {
                        state.result.set(res.value);
                        state.error.set("");
                    } else {
                        state.error.set(res.error);
                        state.result.set("");
                    }
                })}>"docs_echo"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_add", [2, 40]);
                    if (res.ok) {
                        state.result.set("2 + 40 = " + res.value);
                        state.error.set("");
                    } else {
                        state.error.set(res.error);
                        state.result.set("");
                    }
                })}>"docs_add(2,40)"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_todo_add", [""]);
                    if (res.ok) {
                        state.error.set("unexpected ok");
                        state.result.set("");
                    } else {
                        state.error.set(res.error);
                        state.result.set("");
                    }
                })}>"Fail validation"</button>
            </div>
            <p class="demo-output">{result}</p>
            <Show when={has_error}>
                <p class="demo-alert demo-alert--error" role="alert">{error}</p>
            </Show>
        </>
    }
}

#[component]
pub fn WhoAmIWidget() -> View {
    let user_id = signal("—".to_string());
    let roles = signal(String::new());
    let authenticated = signal(false);
    let error = signal(String::new());
    let has_error = computed!([error], move || !error.get().is_empty());

    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_whoami", []);
            if (!res.ok) { state.error.set(res.error); return; }
            state.error.set("");
            state.user_id.set(res.value.user_id ?? "—");
            const r = res.value.roles;
            state.roles.set(Array.isArray(r) ? r.join(", ") : String(r ?? ""));
            state.authenticated.set(!!res.value.authenticated);
        }
    "#,
        user_id,
        roles,
        authenticated,
        error
    );

    view! {
        <>
            <p class="demo-muted">
                "Every "
                <code>"#[server]"</code>
                " action runs through "
                <code>"set_action_middleware"</code>
                " — switch user cookie and refresh."
            </p>
            <div class="demo-row todo-demo-user-row">
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=guest; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_whoami", []);
                    if (!res.ok) { state.error.set(res.error); return; }
                    state.error.set("");
                    state.user_id.set(res.value.user_id ?? "—");
                    const r = res.value.roles;
                    state.roles.set(Array.isArray(r) ? r.join(", ") : String(r ?? ""));
                    state.authenticated.set(!!res.value.authenticated);
                })}>"guest"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=alice; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_whoami", []);
                    if (!res.ok) { state.error.set(res.error); return; }
                    state.error.set("");
                    state.user_id.set(res.value.user_id ?? "—");
                    const r = res.value.roles;
                    state.roles.set(Array.isArray(r) ? r.join(", ") : String(r ?? ""));
                    state.authenticated.set(!!res.value.authenticated);
                })}>"alice"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=bob; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_whoami", []);
                    if (!res.ok) { state.error.set(res.error); return; }
                    state.error.set("");
                    state.user_id.set(res.value.user_id ?? "—");
                    const r = res.value.roles;
                    state.roles.set(Array.isArray(r) ? r.join(", ") : String(r ?? ""));
                    state.authenticated.set(!!res.value.authenticated);
                })}>"bob"</button>
                <button type="button" class="btn btn-sm" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_whoami", []);
                    if (!res.ok) { state.error.set(res.error); return; }
                    state.error.set("");
                    state.user_id.set(res.value.user_id ?? "—");
                    const r = res.value.roles;
                    state.roles.set(Array.isArray(r) ? r.join(", ") : String(r ?? ""));
                    state.authenticated.set(!!res.value.authenticated);
                })}>"Refresh"</button>
            </div>
            <dl class="demo-kv">
                <div class="demo-kv__row">
                    <dt>"user_id"</dt>
                    <dd><code>{user_id}</code></dd>
                </div>
                <div class="demo-kv__row">
                    <dt>"roles"</dt>
                    <dd><code>{roles}</code></dd>
                </div>
                <div class="demo-kv__row">
                    <dt>"authenticated"</dt>
                    <dd>
                        <Show when={authenticated.get()}><code>"true"</code></Show>
                        <Show when={!authenticated.get()}><code>"false"</code></Show>
                    </dd>
                </div>
            </dl>
            <Show when={has_error}>
                <p class="demo-alert demo-alert--error" role="alert">{error}</p>
            </Show>
        </>
    }
}

#[component]
pub fn SecurityEnvWidget() -> View {
    let resuma_env = signal("—".to_string());
    let trust_proxy = signal("—".to_string());
    let rate_backend = signal("—".to_string());
    let csrf = signal("—".to_string());
    let origin_check = signal("—".to_string());
    let error = signal(String::new());
    let has_error = computed!([error], move || !error.get().is_empty());

    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_runtime_security", []);
            if (!res.ok) { state.error.set(res.error); return; }
            state.error.set("");
            const v = res.value;
            state.resuma_env.set(v.resuma_env ?? "—");
            state.trust_proxy.set(v.trust_proxy || "—");
            state.rate_backend.set(v.rate_backend ?? "—");
            state.csrf.set(v.csrf ?? "—");
            state.origin_check.set(v.origin_check ?? "—");
        }
    "#,
        resuma_env,
        trust_proxy,
        rate_backend,
        csrf,
        origin_check,
        error
    );

    view! {
        <>
            <p class="demo-muted">
                "Live snapshot from this server process (no secrets) — compare with "
                <code>"resuma doctor"</code>"."
            </p>
            <button type="button" class="btn btn-sm" onClick={js!(async () => {
                const res = await __resuma.safeAction("docs_runtime_security", []);
                if (!res.ok) { state.error.set(res.error); return; }
                state.error.set("");
                const v = res.value;
                state.resuma_env.set(v.resuma_env ?? "—");
                state.trust_proxy.set(v.trust_proxy || "—");
                state.rate_backend.set(v.rate_backend ?? "—");
                state.csrf.set(v.csrf ?? "—");
                state.origin_check.set(v.origin_check ?? "—");
            })}>"Refresh"</button>
            <dl class="demo-kv">
                <div class="demo-kv__row">
                    <dt><code>"RESUMA_ENV"</code></dt>
                    <dd><code>{resuma_env}</code></dd>
                </div>
                <div class="demo-kv__row">
                    <dt><code>"RESUMA_TRUST_PROXY"</code></dt>
                    <dd><code>{trust_proxy}</code></dd>
                </div>
                <div class="demo-kv__row">
                    <dt>"rate backend"</dt>
                    <dd><code>{rate_backend}</code></dd>
                </div>
                <div class="demo-kv__row">
                    <dt>"CSRF"</dt>
                    <dd><code>{csrf}</code></dd>
                </div>
                <div class="demo-kv__row">
                    <dt>"origin check"</dt>
                    <dd><code>{origin_check}</code></dd>
                </div>
            </dl>
            <Show when={has_error}>
                <p class="demo-alert demo-alert--error" role="alert">{error}</p>
            </Show>
        </>
    }
}

#[component]
pub fn JsWidget() -> View {
    let msg = signal(String::new());
    view! {
        <>
            <input placeholder="Type here" onInput={js! { state.msg.set(event.target.value); }} />
            <p class="demo-output">"js! input: " {msg}</p>
        </>
    }
}

#[component]
fn SlotsDemoShell() -> View {
    view! {
        <div class="demo-slot-shell">
            <Slot name="header" />
            <Slot />
        </div>
    }
}

pub fn slots_widget() -> View {
    view! {
        <SlotsDemoShell>
            <h4 slot="header">"Header slot"</h4>
            <p>"Default slot body"</p>
        </SlotsDemoShell>
    }
}

pub fn nav_link_widget() -> View {
    view! {
        <div class="demo-row">
            <NavLink href="/docs/components/signals" activeClass="active">"Signals"</NavLink>
            <NavLink href="/docs/components/form" activeClass="active">"Form"</NavLink>
            <NavLink href="/docs/flow/loaders" activeClass="active">"Loaders"</NavLink>
        </div>
    }
}

#[component]
pub fn GreetFormWidget() -> View {
    let result = signal(String::new());
    visible_task!(
        r#"
        async () => {
            const form = document.getElementById("docs-greet-form");
            const out = document.getElementById("docs-greet-out");
            if (!form || !out) return;
            const onSubmit = async (ev) => {
                if (ev.target !== form) return;
                ev.preventDefault();
                const fd = new FormData(form);
                const body = {};
                fd.forEach((v, k) => { body[k] = String(v); });
                const params = new URLSearchParams(body);
                try {
                    const res = await fetch(form.action, {
                        method: "POST",
                        credentials: "same-origin",
                        headers: {
                            "content-type": "application/x-www-form-urlencoded",
                            accept: "application/json",
                        },
                        body: params.toString(),
                    });
                    const data = await res.json();
                    if (!res.ok || data.ok === false) {
                        const fields = data.field_errors ?? {};
                        out.textContent = Object.values(fields).join(" · ") || data.error || "Submit failed";
                        return;
                    }
                    out.textContent = data.value?.message ?? JSON.stringify(data.value);
                } catch (e) {
                    out.textContent = String(e);
                }
            };
            form.addEventListener("submit", onSubmit);
            return () => form.removeEventListener("submit", onSubmit);
        }
    "#
    );
    view! {
        <>
            <Form submit={crate::site::demo_actions::docs_greet} id="docs-greet-form">
                <label>"Name" <input name="name" type="text" required=true /></label>
                <button type="submit" class="btn btn-sm">"Greet via #[submit]"</button>
            </Form>
            <p id="docs-greet-out" class="demo-output">{result}</p>
        </>
    }
}

#[component]
pub fn VisibleTaskWidget() -> View {
    let armed = signal(false);
    visible_task!(
        r#"
        async (state, __resuma) => {
            state.armed.set(true);
        }
    "#,
        armed
    );
    view! {
        <>
            <p class="demo-muted">"This panel uses " <code>"visible_task!"</code> " — the message below appears when the demo scrolls into view."</p>
            <Show
                when={armed.get()}
                fallback={view! { <p class="demo-output">"Waiting for viewport…"</p> }}
            >
                <p class="demo-output">"Visible task ran — deferred client work is active."</p>
            </Show>
        </>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct UiStore {
    theme: String,
    count: i32,
}

#[component]
pub fn StoreWidget() -> View {
    let ui = use_store(UiStore {
        theme: "dark".into(),
        count: 0,
    });
    let label = signal("Theme: dark · Count: 0".to_string());
    view! {
        <>
            <p class="demo-output">{label}</p>
            <button type="button" class="btn btn-sm" onClick={js! {
                state.ui.update(s => { s.count += 1; });
                const u = state.ui.value;
                state.label.set("Theme: " + u.theme + " · Count: " + u.count);
            }}>"Increment store"</button>
        </>
    }
}

#[data]
struct LocaleCtx {
    lang: String,
}

static LOCALE: resuma::ContextId<LocaleCtx> = resuma::ContextId::new();

#[component]
fn LocaleProvider() -> View {
    provide_context(&LOCALE, LocaleCtx { lang: "es".into() });
    view! { <LocaleConsumer /> }
}

#[component]
fn LocaleConsumer() -> View {
    let locale = use_context(&LOCALE);
    view! { <p class="demo-output">"Context locale: " {locale.lang.clone()}</p> }
}

pub fn context_widget() -> View {
    view! { <LocaleProvider /> }
}

#[component]
pub fn DebounceWidget() -> View {
    let results = signal(String::new());
    view! {
        <>
            <input type="search" placeholder="Type 2+ chars…" onInput={js! {
                const q = event.target.value;
                clearTimeout(window.__docsDebounce);
                window.__docsDebounce = setTimeout(() => {
                    state.results.set(
                        q.length >= 2 ? "Debounced: '" + q + "'" : ""
                    );
                }, 300);
            }} />
            <p class="demo-output">{results}</p>
        </>
    }
}

#[component]
pub fn PortalWidget() -> View {
    let open = signal(false);
    view! {
        <>
            <button type="button" class="btn btn-sm" onClick={open.set(true)}>"Open modal"</button>
            <Show when={open}>
                {portal("modals", vec![Child::View(view! {
                    <div class="demo-modal-backdrop" onClick={open.set(false)}>
                        <div class="demo-modal" onClick={js! { event.stopPropagation(); }}>
                            <h4>"Portal modal"</h4>
                            <p>"Rendered via portal() into #modals"</p>
                            <button type="button" class="btn btn-sm" onClick={open.set(false)}>"Close"</button>
                        </div>
                    </div>
                })])}
            </Show>
        </>
    }
}

#[component]
pub fn ThemeWidget() -> View {
    let dark = signal(true);

    let dark_theme = Theme {
        mode: "dark".into(),
        primary: "#712cf9".into(),
        background: "#0b1020".into(),
        foreground: "#e6e8ee".into(),
    };
    let light_theme = Theme {
        mode: "light".into(),
        primary: "#4f46e5".into(),
        background: "#f4f7fb".into(),
        foreground: "#0f172a".into(),
    };

    provide_theme(dark_theme.clone());

    view! {
        <>
            <Show
                when={dark}
                fallback={view! {
                    <div class="demo-theme-panel" style={theme_css_vars(&light_theme)}>
                        <p>"Mode: " <strong>"light"</strong> " — panel uses " <code>"theme_css_vars"</code></p>
                        <button
                            type="button"
                            class="btn btn-sm btn-themed"
                            onClick={dark.set(true)}
                        >
                            "Switch to dark"
                        </button>
                    </div>
                }}
            >
                <div class="demo-theme-panel" style={theme_css_vars(&dark_theme)}>
                    <p>"Mode: " <strong>"dark"</strong> " — panel uses " <code>"theme_css_vars"</code></p>
                    <button
                        type="button"
                        class="btn btn-sm btn-themed"
                        onClick={dark.set(false)}
                    >
                        "Switch to light"
                    </button>
                </div>
            </Show>
        </>
    }
}

#[component]
pub fn ReactivityWidget() -> View {
    let n = signal(0_i32);
    let doubled = signal(0_i32);
    effect!([n, doubled], move || {
        doubled.set(n.get() * 2);
    });
    view! {
        <>
            <p class="demo-output">"n = " {n} " → doubled = " {doubled}</p>
            <button type="button" class="btn btn-sm" onClick={n.update(|v| *v += 1)}>"Bump n"</button>
        </>
    }
}

#[component]
pub fn LoaderInvalidationWidget() -> View {
    let stamp = signal(String::new());
    let status = signal(String::new());
    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_loader_stamp", []);
            if (res.ok) state.stamp.set(res.value);
        }
    "#,
        stamp
    );
    view! {
        <>
            <p class="demo-muted">
                "Server stamp via "
                <code>"#[server]"</code>
                " — invalidate SPA prefetch cache, then refresh."
            </p>
            <p class="demo-output">"Stamp: " {stamp}</p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_loader_stamp", []);
                    state.stamp.set(res.ok ? res.value : res.error);
                    state.status.set(res.ok ? "Refreshed via action" : res.error);
                })}>"Refresh stamp"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    await __resuma.invalidate("/docs/flow/loaders");
                    state.status.set("Invalidated /docs/flow/loaders prefetch");
                })}>"Invalidate loaders"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    await __resuma.invalidate();
                    const res = await __resuma.safeAction("docs_loader_stamp", []);
                    state.stamp.set(res.ok ? res.value : res.error);
                    state.status.set("Invalidated current route + refreshed");
                })}>"Invalidate + refresh"</button>
            </div>
            <p class="demo-muted">{status}</p>
        </>
    }
}

#[component]
pub fn DeployInfoWidget() -> View {
    let payload = signal(String::new());
    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_deploy_info", []);
            state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
        }
    "#,
        payload
    );
    view! {
        <>
            <p class="demo-muted">"Live process env — same vars you set on Fly, DigitalOcean, or Docker."</p>
            <button type="button" class="btn btn-sm" onClick={js!(async () => {
                const res = await __resuma.safeAction("docs_deploy_info", []);
                state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
            })}>"Refresh"</button>
            <pre class="code demo-output">{payload}</pre>
        </>
    }
}

#[component]
pub fn CacheControlWidget() -> View {
    view! {
        <section data-docs-cache-demo="">
            <dl class="demo-kv">
                <div class="demo-kv__row">
                    <dt>"Loader policy"</dt>
                    <dd><code>"#[load(cache = \"public, max-age=60\")]"</code></dd>
                </div>
                <div class="demo-kv__row">
                    <dt>"Response Cache-Control"</dt>
                    <dd><code data-cache-header="">"—"</code></dd>
                </div>
                <div class="demo-kv__row">
                    <dt>"Server stamp (now)"</dt>
                    <dd><code data-cache-stamp="">"—"</code></dd>
                </div>
            </dl>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js!(async () => {
                    const mod = await import("/static/client/docs-copy.js?v=1.3.1");
                    await mod.refreshCacheDemo?.();
                })}>"Refresh headers"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    await __resuma.invalidate(location.pathname);
                })}>"SPA reload"</button>
            </div>
            <p class="demo-hint">
                <code>"Refresh headers"</code>
                " fetches this page with "
                <code>"cache: no-store"</code>
                ". "
                <code>"SPA reload"</code>
                " uses "
                <code>"invalidate()"</code>
                " to bust prefetch and re-run "
                <code>"#[load]"</code>
                " — watch the SSR timestamp above change."
            </p>
            <p class="demo-muted" data-cache-status=""></p>
        </section>
    }
}

#[component]
pub fn ViewTransitionsWidget() -> View {
    with_view_transition(
        "docs-vt-demo",
        vec![Child::View(view! {
            <>
                <p class="demo-muted">
                    "Click navigate — every " <code>"NavLink"</code> " on this site uses Resuma SPA navigation with the View Transitions API."
                </p>
                <div class="demo-row">
                    <NavLink href="/docs/cookbook/theme" activeClass="active">"→ Theme"</NavLink>
                    <NavLink href="/docs/cookbook/portals" activeClass="active">"→ Portals"</NavLink>
                    <NavLink href="/docs/cookbook/debouncer" activeClass="active">"→ Debouncer"</NavLink>
                </div>
            </>
        })],
    )
}

#[component]
pub fn PipelineWidget() -> View {
    let step = signal(0_i32);
    let label = signal("1. SSR render".to_string());
    view! {
        <>
            <p class="demo-output">{label}</p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js! {
                    const s = (state.step.value + 1) % 5;
                    state.step.set(s);
                    const labels = [
                        "1. SSR render",
                        "2. Serialize signals",
                        "3. Embed payload",
                        "4. Loader resumes",
                        "5. Client reactive",
                    ];
                    state.label.set(labels[s]);
                }}>"Next step"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js! {
                    state.step.set(0);
                    state.label.set("1. SSR render");
                }}>"Reset"</button>
            </div>
        </>
    }
}

#[component]
pub fn PopupWidget() -> View {
    view! {
        <Popup id="docs-menu" positions="bottom right top left">
            <button slot="anchor" type="button" class="btn btn-sm">"Open menu"</button>
            <div class="demo-popup-panel">
                <p>"Follows the button (CSS anchor). Flip: bottom → right → top → left."</p>
                <a href="/docs/components/modal">"Modal docs"</a>
            </div>
        </Popup>
    }
}

#[component]
pub fn ModalWidget() -> View {
    view! {
        <Modal id="docs-confirm">
            <button slot="trigger" type="button" class="btn btn-sm">"Open dialog"</button>
            <div>
                <h4>"Stacked modal"</h4>
                <p>"ESC and backdrop dismiss. Focus returns to this trigger."</p>
                <form method="dialog">
                    <button type="submit" class="btn btn-sm">"Close"</button>
                </form>
            </div>
        </Modal>
    }
}

#[component]
pub fn GestureWidget() -> View {
    let msg = signal("Pan, pinch, long-press, double-tap, or scroll-wheel.".to_string());
    view! {
        <>
            <GestureView
                preferredPan="horizontal"
                panThreshold={10}
                class="demo-gesture-pad"
                onPan={js! { state.msg.set("pan " + Math.round(event.detail.dx) + "," + Math.round(event.detail.dy)); }}
                onLongPress={js! { state.msg.set("long-press"); }}
                onDoubletap={js! { state.msg.set("double-tap"); }}
                onPinch={js! { state.msg.set("pinch " + event.detail.scale.toFixed(2)); }}
                onScrollwheel={js! { state.msg.set("wheel"); }}
            >
                <p class="demo-output">{msg}</p>
            </GestureView>
        </>
    }
}

#[component]
pub fn VirtualForWidget() -> View {
    let rows = signal((0..400).map(|i| format!("Row {i}")).collect::<Vec<_>>());
    view! {
        <div data-r-virtual-scroller="true" class="demo-virtual-scroller">
            <For each={rows} virtual itemHeight={36} overscan={4} let:label>
                <div class="demo-virtual-row">{label.clone()}</div>
            </For>
        </div>
    }
}

#[component]
pub fn DesktopUiWidget() -> View {
    let info = signal(String::new());
    view! {
        <>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js! {
                    __resuma.announce("Saved");
                    state.info.set("announced + online=" + __resuma.online() + " idle=" + __resuma.presence().idle);
                }}>"Announce"</button>
                <button type="button" class="btn btn-sm" onClick={js! {
                    const m = __resuma.measure(event.currentTarget);
                    state.info.set(m ? ("w=" + Math.round(m.width)) : "no layout");
                }}>"Measure"</button>
            </div>
            <p class="demo-output">{info}</p>
        </>
    }
}
