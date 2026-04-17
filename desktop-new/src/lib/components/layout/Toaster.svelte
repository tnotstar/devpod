<script lang="ts">
import { CircleCheck, CircleX, Info, X } from "@lucide/svelte"
import { toasts } from "$lib/stores/toasts.js"
import type { Toast } from "$lib/stores/toasts.js"
import { fly } from "svelte/transition"
import { onDestroy } from "svelte"

// Track timers per toast so we can pause/resume on hover
let timers = $state<
  Map<
    string,
    {
      timeoutId: ReturnType<typeof setTimeout>
      remaining: number
      startedAt: number
    }
  >
>(new Map())

function startTimer(toast: Toast) {
  const existing = timers.get(toast.id)
  const remaining = existing?.remaining ?? toast.duration

  const timeoutId = setTimeout(() => {
    toasts.dismiss(toast.id)
    timers.delete(toast.id)
  }, remaining)

  timers.set(toast.id, { timeoutId, remaining, startedAt: Date.now() })
}

function pauseTimer(id: string) {
  const entry = timers.get(id)
  if (!entry) return
  clearTimeout(entry.timeoutId)
  const elapsed = Date.now() - entry.startedAt
  timers.set(id, {
    ...entry,
    remaining: Math.max(0, entry.remaining - elapsed),
    timeoutId: 0 as unknown as ReturnType<typeof setTimeout>,
  })
}

function resumeTimer(id: string) {
  const entry = timers.get(id)
  if (!entry || entry.remaining <= 0) return

  const timeoutId = setTimeout(() => {
    toasts.dismiss(id)
    timers.delete(id)
  }, entry.remaining)

  timers.set(id, { ...entry, timeoutId, startedAt: Date.now() })
}

// Start timers for new toasts
$effect(() => {
  for (const toast of $toasts) {
    if (!timers.has(toast.id)) {
      startTimer(toast)
    }
  }
})

// Cleanup on destroy
onDestroy(() => {
  for (const entry of timers.values()) {
    clearTimeout(entry.timeoutId)
  }
})

function progressBarColor(variant: Toast["variant"]): string {
  if (variant === "error") return "bg-destructive"
  if (variant === "success") return "bg-green-500"
  return "bg-foreground/30"
}
</script>

{#if $toasts.length > 0}
  <div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
    {#each $toasts as toast (toast.id)}
      <button
        type="button"
        class="group relative flex items-center gap-3 overflow-hidden rounded-md border px-4 py-3 text-sm shadow-lg {toast.variant === 'error'
          ? 'border-destructive bg-destructive/10 text-destructive'
          : toast.variant === 'success'
            ? 'border-green-500 bg-green-500/10 text-green-700 dark:text-green-400'
            : 'border-border bg-card text-card-foreground'}"
        onclick={() => toasts.dismiss(toast.id)}
        onmouseenter={() => pauseTimer(toast.id)}
        onmouseleave={() => resumeTimer(toast.id)}
        transition:fly={{ x: 100, duration: 200 }}
      >
        {#if toast.variant === "success"}
          <CircleCheck class="h-4 w-4 shrink-0" />
        {:else if toast.variant === "error"}
          <CircleX class="h-4 w-4 shrink-0" />
        {:else}
          <Info class="h-4 w-4 shrink-0" />
        {/if}
        <span class="flex-1 text-left">{toast.message}</span>
        <X class="h-3 w-3 shrink-0 opacity-50" />

        <!-- Progress bar -->
        <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-transparent">
          <div
            class="h-full {progressBarColor(toast.variant)} toast-progress group-hover:[animation-play-state:paused]"
            style="animation-duration: {toast.duration}ms;"
          ></div>
        </div>
      </button>
    {/each}
  </div>
{/if}

<style>
  @keyframes shrink {
    from {
      width: 100%;
    }
    to {
      width: 0%;
    }
  }
  .toast-progress {
    animation: shrink linear forwards;
  }
</style>
