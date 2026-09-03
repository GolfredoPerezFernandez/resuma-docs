/** Run docs_showcase + mount flow execution panel (keeps page handlers under inline size). */

type DemoKey = "exec";

type DemoIds = {
  topic: string;
  blurb: string;
  err: string;
  slot: string;
  btn: string;
  guide: string;
  placeholder: string;
};

const DEMOS: Record<DemoKey, DemoIds> = {
  exec: {
    topic: "exec-topic",
    blurb: "exec-blurb",
    err: "exec-err",
    slot: "exec-flow-slot",
    btn: "exec-start-btn",
    guide: "exec",
    placeholder: "exec-flow-placeholder",
  },
};

type SafeActionResult =
  | { ok: true; value: Record<string, unknown> }
  | { ok: false; error: string };

type ResumaGlobal = {
  safeAction(name: string, args: unknown[]): Promise<SafeActionResult>;
};

declare global {
  interface Window {
    __resuma?: ResumaGlobal;
    __resumaCoreReady?: Promise<void>;
  }
}

function setGuideStep(guideId: string, step: number, status?: string) {
  const root = document.querySelector<HTMLElement>(`[data-guide="${guideId}"]`);
  if (!root) return;
  root.querySelectorAll<HTMLElement>("[data-step]").forEach((el) => {
    const n = Number(el.dataset.step);
    el.classList.toggle("is-active", n === step);
    el.classList.toggle("is-done", n < step);
  });
  const statusEl = root.querySelector<HTMLElement>("[data-guide-status]");
  if (statusEl && status) statusEl.textContent = status;
}

function showPlaceholder(id: string, visible: boolean) {
  const el = document.getElementById(id);
  if (!el) return;
  el.hidden = !visible;
}

export async function runDocsFlowWorker(key: DemoKey): Promise<void> {
  const ids = DEMOS[key];
  const topic = (document.getElementById(ids.topic) as HTMLInputElement | null)?.value ?? "";
  const blurb =
    (document.getElementById(ids.blurb) as HTMLTextAreaElement | null)?.value ?? "";
  const errEl = document.getElementById(ids.err) as HTMLParagraphElement | null;
  const slot = document.getElementById(ids.slot) as HTMLDivElement | null;
  const btn = document.getElementById(ids.btn) as HTMLButtonElement | null;

  if (!errEl || !slot || !btn) return;

  const __resuma = window.__resuma;
  if (!__resuma) {
    errEl.textContent = "Resuma runtime not ready.";
    errEl.hidden = false;
    return;
  }

  errEl.hidden = true;
  btn.disabled = true;
  setGuideStep(ids.guide, 2, "Starting worker — panel loading…");
  showPlaceholder(ids.placeholder, false);
  try {
    const res = await __resuma.safeAction("run_docs_flow_worker", [topic, blurb]);
    if (!res.ok) {
      errEl.textContent = res.error;
      errEl.hidden = false;
      setGuideStep(ids.guide, 1, "Ready — start with step 1.");
      showPlaceholder(ids.placeholder, true);
      return;
    }

    const panelHtml = String(res.value.panel_html ?? "");
    if (!panelHtml) {
      errEl.textContent = "Worker started but no panel HTML was returned.";
      errEl.hidden = false;
      setGuideStep(ids.guide, 1, "Ready — start with step 1.");
      showPlaceholder(ids.placeholder, true);
      return;
    }

    if (window.__resumaCoreReady) await window.__resumaCoreReady;

    let flow: {
      disconnectFlowWidgets: (el: Element) => void;
      initFlowWidgets: (root: HTMLElement, opts: { flush: boolean }) => void;
      teardownGraph: (graphId: string) => void;
      syncFlowControls: (root: HTMLElement) => void;
    };
    try {
      flow = await import("/_resuma/flow.js?v=1.3.1");
    } catch (e) {
      errEl.textContent = "Could not load Flow widgets: " + String(e);
      errEl.hidden = false;
      setGuideStep(ids.guide, 1, "Ready — start with step 1.");
      showPlaceholder(ids.placeholder, true);
      return;
    }

    const oldGraphId =
      slot.querySelector<HTMLElement>("[data-r-flow-execution]")?.getAttribute(
        "data-r-flow-execution",
      ) ?? "";

    flow.disconnectFlowWidgets(slot);
    if (oldGraphId) flow.teardownGraph(oldGraphId);
    slot.hidden = true;
    slot.innerHTML = "";
    delete slot.dataset.docsExecPanel;

    slot.dataset.docsExecPanel = "1";
    slot.innerHTML = panelHtml;
    slot.querySelectorAll("style[data-r-flow-styles]").forEach((n) => n.remove());
    slot.hidden = false;
    flow.initFlowWidgets(slot, { flush: false });
    flow.syncFlowControls?.(slot);
    window.setTimeout(() => flow.syncFlowControls?.(slot), 400);
    window.setTimeout(() => flow.syncFlowControls?.(slot), 1500);
    setGuideStep(
      ids.guide,
      3,
      "Panel live — under Controls, click Pause or Cancel while status is Running.",
    );
  } finally {
    btn.disabled = false;
  }
}
