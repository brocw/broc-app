import { invoke } from "@tauri-apps/api/core";

export interface Action {
  id: string;
  name: string;
  description: string;
  icon: string;
  category: string;
  execute: () => Promise<string>;
}

const actions: Map<string, Action> = new Map();

export function registerAction(action: Action): void {
  actions.set(action.id, action);
}

export function getAllActions(): Action[] {
  return Array.from(actions.values());
}

export function searchActions(query: string): Action[] {
  if (!query.trim()) return getAllActions();
  const q = query.toLowerCase();
  return getAllActions().filter(
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
