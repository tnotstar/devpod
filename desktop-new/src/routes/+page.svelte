<script lang="ts">
import { Button } from "$lib/components/ui/button"
import { onMount } from "svelte"

let platform = $state("detecting...")

onMount(async () => {
  try {
    const os = await import("@tauri-apps/plugin-os")
    platform = os.platform()
  } catch {
    platform = "browser (not in Tauri)"
  }
})
</script>

<div class="flex flex-col items-center justify-center h-screen gap-4 bg-background">
  <h1 class="text-4xl font-bold text-foreground">DevPod</h1>
  <p class="text-muted-foreground">Platform: {platform}</p>
  <Button variant="default">Get Started</Button>
</div>
