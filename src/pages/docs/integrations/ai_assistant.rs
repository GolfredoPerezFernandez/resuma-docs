use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"AI assistant (Cursor, Codex, Gemini)"</h1>
            <p class="lead">
                "Teach your editor how to write Resuma — reactive " <code>"view!"</code>
                ", " <code>"HtmlTheme"</code>
                " / " <code>"Popup"</code>
                ", Flow, " <code>"SeoKit"</code>
                ", and the traps that freeze UI or clobber a live theme — with one CLI command."
            </p>

            <h2>"Skill vs MCP — which one?"</h2>
            <table class="bench">
                <thead>
                    <tr>
                        <th></th>
                        <th>"Agent skill (recommended)"</th>
                        <th>"MCP server"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Best for"</td>
                        <td>"Writing & fixing Resuma code in any project"</td>
                        <td>"Tools that call APIs, DBs, or live runtime state"</td>
                    </tr>
                    <tr>
                        <td>"Setup"</td>
                        <td><code>"resuma install skill"</code></td>
                        <td>"Custom server + editor MCP config (not bundled yet)"</td>
                    </tr>
                    <tr>
                        <td>"Editors"</td>
                        <td>"Cursor, Codex/agents, any SKILL.md-compatible agent"</td>
                        <td>"Cursor, Claude Desktop, etc."</td>
                    </tr>
                </tbody>
            </table>
            <p>
                "Start with the " <strong>"skill"</strong> ". It encodes Resuma patterns (signals, Show, HtmlTheme, Popup, Flow, #[server], SeoKit) so the model ships working apps faster. "
                "An official Resuma MCP may come later for docs search and " <code>"resuma routes --generate"</code> " as tools."
            </p>

            <h2>"Install the skill (one command)"</h2>
            {code_block(r#"# Global — available in all projects (Cursor)
resuma install skill

# Only this repo / monorepo
resuma install skill --project

# Codex-style agents path (~/.agents/skills/)
resuma install skill --target agents

# Cursor + agents
resuma install skill --target all

# See paths without writing
resuma install skill --list

# Overwrite existing SKILL.md
resuma install skill --force"#)}

            <h2>"Where files land"</h2>
            <ul>
                <li><code>"~/.cursor/skills/resuma/SKILL.md"</code> " — Cursor personal skill"</li>
                <li><code>".cursor/skills/resuma/SKILL.md"</code> " — committed with your app (team shares the same guidance)"</li>
                <li><code>"~/.agents/skills/resuma/SKILL.md"</code> " — compatible with open agent skills ecosystems"</li>
            </ul>

            <h2>"Gemini / other editors"</h2>
            <p>
                "Copy " <code>"SKILL.md"</code> " from " <code>"resuma install skill --list"</code> " into your editor's rules or instructions file, "
                "or run " <code>"resuma install skill --project"</code> " and point the editor at " <code>".cursor/skills/resuma/SKILL.md"</code> "."
            </p>

            <h2>"What the skill covers"</h2>
            <ul>
                <li><code>"{signal}"</code> " vs " <code>"{signal.get()}"</code> " in " <code>"view!"</code> " (client reactivity)"</li>
                <li>"Reactive " <code>"&lt;Show when={…}&gt;"</code></li>
                <li><code>"js!"</code> ": " <code>"event.currentTarget"</code> " is the handler node (not " <code>"document"</code> ")"</li>
                <li><code>"FlowApp"</code> ", file-based pages (" <code>"page(req: FlowRequest)"</code> "), " <code>"resuma routes --generate"</code></li>
                <li><code>"#[server]"</code> ", " <code>"#[submit]"</code> ", " <code>"#[load]"</code></li>
                <li>
                    <code>"HtmlTheme"</code> " / " <code>"ThemeSwitch"</code> " / " <code>"data-r-theme"</code>
                    " — live " <code>"html[data-theme]"</code>
                    ", not a layout " <code>"onClick"</code>
                </li>
                <li>
                    "Native " <code>"Popup"</code> " / " <code>"Modal"</code> " / " <code>"GestureView"</code>
                    " / " <code>"&lt;For virtual&gt;"</code>
                    " (" <code>"/_resuma/ui.js"</code> " is a lazy fallback only)"
                </li>
                <li>
                    <code>"__resuma.announce"</code> " / " <code>"measure"</code> " / " <code>"storage"</code>
                    " / " <code>"set_page_dir"</code> " / " <code>"set_page_theme"</code>
                </li>
                <li>
                    "SPA must not copy " <code>"data-theme"</code>
                    " from prefetch; skip View Transitions while a popover is open"
                </li>
                <li><code>"ClientComponent::lazy"</code></li>
                <li>
                    <code>"SeoKit"</code> " (in " <code>"prelude"</code> "), " <code>"SITE_URL"</code> ", " <code>"set_page_title"</code> ", "
                    <code>"with_sitemap_exclude"</code> ", " <code>"/robots.txt"</code> " / " <code>"/llms.txt"</code>
                    " — call " <code>"with_seo_kit"</code> " once (do not remount robots)"
                </li>
                <li>"Deploy as a long-running Docker/Fly process — not Lambda or Cloudflare Workers"</li>
                <li>"Resuma OS: workers, queue, scheduler, Flow widgets"</li>
                <li>"Debugging checklist (handler chunks, CSRF, SITE_URL, theme boot, streaming " <code>"#r-live"</code> ")"</li>
            </ul>

            <h2>"Verify"</h2>
            {code_block(r#"resuma doctor
# In Cursor: ask "create a Resuma counter with view! and signal"
# The agent should use {count} not {count.get()} in the template."#)}

            <p>
                <a href="/docs/cli">"CLI reference"</a>
                " · "
                <a href="/docs/getting_started">"Getting started"</a>
                " · "
                <a href="/docs/cookbook/theme">"Theme"</a>
                " · "
                <a href="/docs/components/popup">"Popup"</a>
                " · "
                <a href="/docs/integrations/seo_geo">"SEO, GEO & AEO"</a>
            </p>
        </>
    }
}
