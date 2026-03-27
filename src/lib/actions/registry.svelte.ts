import { untrack } from "svelte";
import { invoke } from "@tauri-apps/api/core";

export interface Action {
  id: string;
  name: string;
  description: string;
  icon: string;
  category: string;
  execute: () => Promise<string>;
  secondaryAction?: { label: string; execute: () => void };
}

let actionsArray = $state<Action[]>([]);

export function registerAction(action: Action): void {
  const idx = untrack(() => actionsArray.findIndex((a) => a.id === action.id));
  if (idx >= 0) {
    actionsArray[idx] = action;
  } else {
    actionsArray.push(action);
  }
}

export function unregisterAction(id: string): void {
  const idx = untrack(() => actionsArray.findIndex((a) => a.id === id));
  if (idx >= 0) actionsArray.splice(idx, 1);
}

export function getAllActions(): Action[] {
  return actionsArray;
}

export function searchActions(query: string): Action[] {
  if (!query.trim()) return actionsArray;
  const q = query.toLowerCase();
  return actionsArray.filter(
    (a) =>
      a.name.toLowerCase().includes(q) ||
      a.description.toLowerCase().includes(q) ||
      a.category.toLowerCase().includes(q)
  );
}

export function backendAction(
  opts: Omit<Action, "execute"> & { command: string; args?: Record<string, unknown> }
): Action {
  return {
    id: opts.id,
    name: opts.name,
    description: opts.description,
    icon: opts.icon,
    category: opts.category,
    execute: async () => {
      const result = await invoke(opts.command, opts.args ?? {});
      return typeof result === "string" ? result : JSON.stringify(result, null, 2);
    },
  };
}
