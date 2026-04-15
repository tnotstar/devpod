<script lang="ts">
import { page } from "$app/stores"
import { goto } from "$app/navigation"
import { onMount } from "svelte"
import { Button } from "$lib/components/ui/button/index.js"
import { Separator } from "$lib/components/ui/separator/index.js"
import { badgeVariants } from "$lib/components/ui/badge/index.js"
import { machines } from "$lib/stores/machines.js"
import {
  machineStart,
  machineStop,
  machineDelete,
  machineStatus,
} from "$lib/ipc/commands.js"

let id = $derived($page.params.id)
let machine = $derived($machines.find((m) => m.id === id))

let status = $state<string | null>(null)
let error = $state("")

onMount(async () => {
  try {
    status = await machineStatus(id)
  } catch {
    // Status fetch failed
  }
})

async function handleStart() {
  try {
    await machineStart(id)
    status = await machineStatus(id)
  } catch (err) {
    error = err instanceof Error ? err.message : String(err)
  }
}

async function handleStop() {
  try {
    await machineStop(id)
    status = await machineStatus(id)
  } catch (err) {
    error = err instanceof Error ? err.message : String(err)
  }
}

async function handleDelete() {
  try {
    await machineDelete(id)
    goto("/machines")
  } catch (err) {
    error = err instanceof Error ? err.message : String(err)
  }
}
</script>

<div class="space-y-6">
  <div class="flex items-center gap-4">
    <Button variant="ghost" size="sm" onclick={() => goto("/machines")}>
      &larr; Back
    </Button>
    <h1 class="text-2xl font-bold">{id}</h1>
    {#if status}
      <span class={badgeVariants({ variant: "default" })}>{status}</span>
    {/if}
  </div>

  {#if error}
    <div class="rounded-md border border-destructive bg-destructive/10 p-3 text-sm text-destructive">
      {error}
    </div>
  {/if}

  <div class="flex gap-2">
    <Button variant="outline" size="sm" onclick={handleStart}>Start</Button>
    <Button variant="outline" size="sm" onclick={handleStop}>Stop</Button>
    <Button variant="destructive" size="sm" onclick={handleDelete}>Delete</Button>
  </div>

  <Separator />

  {#if !machine}
    <p class="text-muted-foreground">Machine not found.</p>
  {:else}
    <div class="grid grid-cols-2 gap-4 text-sm">
      <div class="text-muted-foreground">ID</div>
      <div>{machine.id}</div>

      <div class="text-muted-foreground">Provider</div>
      <div>{machine.provider?.name ?? "N/A"}</div>

      <div class="text-muted-foreground">Status</div>
      <div>{status ?? machine.status ?? "Unknown"}</div>

      <div class="text-muted-foreground">Created</div>
      <div>{machine.creationTimestamp ?? "N/A"}</div>

      <div class="text-muted-foreground">Last Used</div>
      <div>{machine.lastUsedTimestamp ?? "N/A"}</div>
    </div>
  {/if}
</div>
