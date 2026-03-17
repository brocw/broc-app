# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

- `npm run tauri dev` — run the full app (frontend + Rust backend) in development mode
- `npm run tauri build` — produce a distributable binary
- `npm run build` — build frontend only (Vite/SvelteKit static output to `build/`)
- `npm run check` — run svelte-check for type checking
- `cargo check` — check Rust compilation (run from `src-tauri/`)

## Architecture

Tauri v2 desktop app with a SvelteKit (Svelte 5) frontend and Rust backend. SvelteKit is configured as a static SPA (adapter-static with index.html fallback) since Tauri has no Node.js server.

### Action System

The app is a dashboard/launcher built around a modular action pattern:

```
UI Components → Action Registry (TS) → Tauri invoke() IPC → Rust Commands
```

**Frontend registry** (`src/lib/actions/registry.ts`): Actions self-register on import via `registerAction()`. The barrel `index.ts` imports all action files to trigger registration. Use `backendAction()` helper for actions that call Rust commands — it handles IPC invocation and JSON serialization automatically.

**Rust commands** (`src-tauri/src/actions/`): Each action is a `#[tauri::command]` function. Return types implementing `Serialize` are automatically serialized to JSON over IPC.

### Adding a New Action

1. **Rust side** (if backend-backed): new `.rs` file in `src-tauri/src/actions/`, add `pub mod` to `mod.rs`, add to `generate_handler![]` in `lib.rs`
2. **Frontend side**: new `.ts` file in `src/lib/actions/` calling `registerAction()`, add import to `index.ts`

Frontend-only actions skip step 1 and provide an `execute` function directly.

## Key Conventions

- Svelte 5 runes syntax: `$state`, `$derived`, `$props`, `$bindable` — not legacy store syntax
- SvelteKit `$lib` alias maps to `src/lib/`
- Vite dev server runs on port 1420 (hardcoded, strict — Tauri requires this exact port)
- Vite is configured to ignore `src-tauri/` for file watching to avoid interfering with Rust compilation
- Rust crate name is `broc_app_lib` (due to Tauri lib/bin naming convention)
