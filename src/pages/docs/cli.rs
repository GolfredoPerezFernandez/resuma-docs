use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"CLI Reference"</h1>
            <p class="lead">"The resuma command scaffolds projects, runs dev servers, builds releases, and generates route registries."</p>

            {crate::site::demos::reference_cli()}

            <h2>"Install"</h2>
            {code_block(r#"cargo install resuma

# From monorepo source:
cargo install --path crates/resuma --features cli"#)}

            <h2>"resuma new / resuma create"</h2>
            <p>"Run " <code>"resuma new"</code> " without arguments in a terminal to pick the project name and template interactively."</p>
            {code_block(r#"resuma new                              # interactive prompts
resuma new my-app
resuma new my-app --template basic          # static SSR (default)
resuma new my-app --template todo           # full showcase
resuma new my-app --template flow           # file-based pages
resuma new my-app --template flow-booking   # appointments + query loaders
resuma new my-app --template flow-fullstack # Flow + SQLx sample
resuma new my-app --template production     # Dockerfile + fly.toml + security stub"#)}

            <h2>"resuma add"</h2>
            {code_block(r#"resuma add              # interactive menu
resuma add sqlx    # src/db.rs, migrations/, deps
resuma add turso  # src/turso.rs, .env.example"#)}

            <h2>"resuma install skill"</h2>
            <p>"Install the Resuma agent skill for Cursor, Codex, or project-local " <code>".cursor/skills/"</code> ". After a CLI upgrade, run with " <code>"--force"</code> " so " <code>"SKILL.md"</code> " picks up " <code>"HtmlTheme"</code> ", overlays, and SEO traps. See " <a href="/docs/integrations/ai_assistant">"AI assistant guide"</a> "."</p>
            {code_block(r#"resuma install skill                 # ~/.cursor/skills/resuma/
resuma install skill --project       # .cursor/skills/resuma/
resuma install skill --target agents # ~/.agents/skills/resuma/
resuma install skill --target all
resuma install skill --list
resuma install skill --force"#)}

            <h2>"resuma update"</h2>
            <p>"Bump " <code>"resuma"</code> " and " <code>"resuma-macros"</code> " in the current project, or reinstall the global CLI."</p>
            {code_block(r#"resuma update              # align project deps with CLI version
resuma update --check      # show versions, no changes
resuma update --cli          # cargo install resuma --force
resuma update --version 1.3.1  # pin a specific release"#)}

            <h2>"resuma doctor"</h2>
            <p>"Quick check for Rust toolchain, " <code>"cargo-watch"</code> ", CLI version, and project " <code>"Cargo.toml"</code> " setup."</p>
            {code_block("resuma doctor")}

            <h2>"resuma dev"</h2>
            <p>"Starts the app with hot reload. Installs " <code>"cargo-watch"</code> " automatically if missing. Saves to " <code>"src/"</code> " or " <code>"Cargo.toml"</code> " trigger a rebuild; the browser refreshes when the dev server comes back."</p>
            {code_block(r#"resuma dev
resuma dev --open
resuma dev --addr 0.0.0.0:8080
resuma dev --no-update     # skip “update Resuma?” (or RESUMA_SKIP_UPDATE=1)
resuma dev --kill-stale    # Linux: fuser -k on the dev port
resuma dev --skip-runtime
# Watches src/, public/, and Cargo.toml
# A second resuma dev in the same project replaces the leftover cargo-watch (pidfile)"#)}

            <h2>"resuma build"</h2>
            {code_block(r#"resuma build                    # release binary
resuma build --static-export    # HTML from src/pages/
resuma build --static-export --out dist --base-url https://example.com"#)}
            <p>
                "After a release build the CLI prints a production checklist. Hosting walkthroughs (Fly, DigitalOcean, Docker): "
                <a href="/docs/cookbook/deploy">"Deploy"</a> "."
            </p>

            <h2>"resuma routes"</h2>
            {code_block(r#"resuma routes --path src/pages
    resuma routes --generate --path src/pages"#)}
        </>
    }
}
