<script lang="ts">
import { Button } from "$lib/components/ui/button/index.js"
import { Input } from "$lib/components/ui/input/index.js"
import MachineCard from "$lib/components/machine/MachineCard.svelte"
import { machines } from "$lib/stores/machines.js"

let search = $state("")
let filtered = $derived(
  $machines.filter((m) => m.id.toLowerCase().includes(search.toLowerCase())),
)
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-bold">Machines</h1>
  </div>

  <Input
    placeholder="Search machines..."
    value={search}
    oninput={(e) => (search = e.currentTarget.value)}
  />

  {#if filtered.length === 0}
    <div class="flex flex-col items-center justify-center gap-4 py-16 text-center">
      <p class="text-muted-foreground">
        {search ? "No machines match your search." : "No machines yet."}
      </p>
      <p class="text-sm text-muted-foreground">
        Machines are created automatically when you start a workspace with a provider that supports them.
      </p>
    </div>
  {:else}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
      {#each filtered as machine (machine.id)}
        <MachineCard {machine} />
      {/each}
    </div>
  {/if}
</div>
