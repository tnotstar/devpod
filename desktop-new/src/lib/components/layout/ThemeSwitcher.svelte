<script lang="ts">
import { Button } from "$lib/components/ui/button/index.js"
import { theme, cycleTheme } from "$lib/stores/settings.js"
import Sun from "@lucide/svelte/icons/sun"
import Moon from "@lucide/svelte/icons/moon"
import Monitor from "@lucide/svelte/icons/monitor"
</script>

<Button variant="ghost" size="sm" onclick={cycleTheme} class="gap-1.5 text-muted-foreground">
  {#if $theme === "light"}
    <Sun class="h-4 w-4" />
    <span>Light</span>
  {:else if $theme === "dark"}
    <Moon class="h-4 w-4" />
    <span>Dark</span>
  {:else}
    <Monitor class="h-4 w-4" />
    <span>System</span>
  {/if}
</Button>
