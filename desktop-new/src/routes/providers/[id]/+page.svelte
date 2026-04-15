<script lang="ts">
import { page } from "$app/stores"
import { goto } from "$app/navigation"
import { onMount } from "svelte"
import { Button } from "$lib/components/ui/button/index.js"
import { Input } from "$lib/components/ui/input/index.js"
import { Label } from "$lib/components/ui/label/index.js"
import { Separator } from "$lib/components/ui/separator/index.js"
import { badgeVariants } from "$lib/components/ui/badge/index.js"
import { providers } from "$lib/stores/providers.js"
import {
  providerUse,
  providerUpdate,
  providerDelete,
  providerOptions,
  providerSetOptions,
} from "$lib/ipc/commands.js"
import type { ProviderOption, OptionValue } from "$lib/types/index.js"

let id = $derived($page.params.id)
let provider = $derived($providers.find((p) => p.name === id))

let options = $state<Record<string, ProviderOption>>({})
let optionValues = $state<Record<string, string>>({})
let error = $state("")
let saving = $state(false)

onMount(async () => {
  try {
    options = await providerOptions(id)
    for (const [key, opt] of Object.entries(options)) {
      optionValues[key] = opt.default != null ? String(opt.default) : ""
    }
  } catch {
    // Options not available
  }
})

async function handleSetDefault() {
  try {
    await providerUse(id)
  } catch (err) {
    error = err instanceof Error ? err.message : String(err)
  }
}

async function handleUpdate() {
  try {
    await providerUpdate(id)
  } catch (err) {
    error = err instanceof Error ? err.message : String(err)
  }
}

async function handleDelete() {
  try {
    await providerDelete(id)
    goto("/providers")
  } catch (err) {
    error = err instanceof Error ? err.message : String(err)
  }
}

async function handleSaveOptions() {
  error = ""
  saving = true
  try {
    const values: Record<string, OptionValue> = {}
    for (const [key, val] of Object.entries(optionValues)) {
      if (val !== "") values[key] = val
    }
    await providerSetOptions(id, values)
  } catch (err) {
    error = err instanceof Error ? err.message : String(err)
  } finally {
    saving = false
  }
}
</script>

<div class="space-y-6">
  <div class="flex items-center gap-4">
    <Button variant="ghost" size="sm" onclick={() => goto("/providers")}>
      &larr; Back
    </Button>
    <h1 class="text-2xl font-bold">{id}</h1>
    {#if provider?.version}
      <span class={badgeVariants({ variant: "secondary" })}>{provider.version}</span>
    {/if}
    {#if provider?.state?.initialized}
      <span class={badgeVariants({ variant: "default" })}>initialized</span>
    {/if}
  </div>

  {#if error}
    <div class="rounded-md border border-destructive bg-destructive/10 p-3 text-sm text-destructive">
      {error}
    </div>
  {/if}

  <div class="flex gap-2">
    <Button variant="outline" size="sm" onclick={handleSetDefault}>Set Default</Button>
    <Button variant="outline" size="sm" onclick={handleUpdate}>Update</Button>
    <Button variant="destructive" size="sm" onclick={handleDelete}>Delete</Button>
  </div>

  {#if !provider}
    <p class="text-muted-foreground">Provider not found.</p>
  {:else}
    {#if provider.description}
      <p class="text-muted-foreground">{provider.description}</p>
    {/if}

    <Separator />

    <div class="space-y-4">
      <h2 class="text-lg font-semibold">Options</h2>

      {#if Object.keys(options).length === 0}
        <p class="text-sm text-muted-foreground">No configurable options available.</p>
      {:else}
        <div class="space-y-4">
          {#each Object.entries(options) as [key, opt] (key)}
            {#if !opt.hidden}
              <div class="space-y-2">
                <Label>{opt.displayName ?? opt.name}</Label>
                {#if opt.description}
                  <p class="text-xs text-muted-foreground">{opt.description}</p>
                {/if}
                {#if opt.enum && opt.enum.length > 0}
                  <select
                    class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    value={optionValues[key] ?? ""}
                    onchange={(e) => (optionValues[key] = e.currentTarget.value)}
                  >
                    <option value="">-- Select --</option>
                    {#each opt.enum as enumVal}
                      <option value={enumVal}>{enumVal}</option>
                    {/each}
                  </select>
                {:else}
                  <Input
                    type={opt.password ? "password" : "text"}
                    placeholder={opt.default != null ? String(opt.default) : ""}
                    value={optionValues[key] ?? ""}
                    oninput={(e) => (optionValues[key] = e.currentTarget.value)}
                  />
                {/if}
              </div>
            {/if}
          {/each}

          <Button onclick={handleSaveOptions} disabled={saving}>
            {saving ? "Saving..." : "Save Options"}
          </Button>
        </div>
      {/if}
    </div>
  {/if}
</div>
