import type { UnlistenFn } from "@tauri-apps/api/event"
import { writable } from "svelte/store"
import { workspaceList } from "$lib/ipc/commands.js"
import { onWorkspacesChanged } from "$lib/ipc/events.js"
import type { Workspace } from "$lib/types/index.js"

export const workspaces = writable<Workspace[]>([])
export const workspacesLoading = writable(true)

let unlisten: UnlistenFn | null = null

export async function initWorkspaces() {
  workspacesLoading.set(true)
  try {
    const list = await workspaceList()
    workspaces.set(list)
  } catch {
    // Tauri not available (e.g. during SSR or browser preview)
  } finally {
    workspacesLoading.set(false)
  }

  try {
    unlisten = await onWorkspacesChanged((updated) => {
      workspaces.set(updated)
    })
  } catch {
    // Event listener setup failed
  }
}

export function destroyWorkspaces() {
  if (unlisten) {
    unlisten()
    unlisten = null
  }
}
