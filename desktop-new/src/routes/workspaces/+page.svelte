<script lang="ts">
import { goto } from "$app/navigation"
import { Button } from "$lib/components/ui/button/index.js"
import { Input } from "$lib/components/ui/input/index.js"
import WorkspaceCard from "$lib/components/workspace/WorkspaceCard.svelte"
import { workspaces } from "$lib/stores/workspaces.js"

let search = $state("")
let filtered = $derived(
  $workspaces.filter((ws) =>
    ws.id.toLowerCase().includes(search.toLowerCase()),
  ),
)
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-bold">Workspaces</h1>
    <Button onclick={() => goto("/workspaces/new")}>Create Workspace</Button>
  </div>

  <Input
    placeholder="Search workspaces..."
    value={search}
    oninput={(e) => (search = e.currentTarget.value)}
  />

  {#if filtered.length === 0}
    <div class="flex flex-col items-center justify-center gap-4 py-16 text-center">
      <p class="text-muted-foreground">
        {search ? "No workspaces match your search." : "No workspaces yet."}
      </p>
      {#if !search}
        <Button onclick={() => goto("/workspaces/new")}>Create your first workspace</Button>
      {/if}
    </div>
  {:else}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
      {#each filtered as workspace (workspace.id)}
        <WorkspaceCard {workspace} />
      {/each}
    </div>
  {/if}
</div>
