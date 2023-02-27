<script lang="ts">
  import EmojiItem from "./lib/EmojiItem.svelte"
  import SettingWindow from "./lib/SettingWindow.svelte"

  let loading: boolean = false
  let shown: boolean = false

  let searchInput: HTMLInputElement
  let query: string = ""

  let emojis = []

  const searchEmojis = async (_query: string) => {
    let query = _query.trim().toLowerCase()
    if (query.length < 1) query = "empty"

    const cache = localStorage.getItem(query)

    if (cache) return JSON.parse(cache)

    const res = await fetch(
      `https://api.emojisworld.fr/v1/search?q=${query}&limit=25`
    )
    const data = await res.json()

    if ("error" in data) {
      return {
        totals: 1,
        results: [
          {
            id: 3511,
            name: "empty nest",
            emoji: "🪹",
            unicode: "1FAB9",
            version: "14.0",
            category: { id: 4, name: "Animals & Nature" },
            sub_category: { id: 41, name: "plant-other" },
            children: [],
          },
        ],
      }
    }

    localStorage.setItem(query, JSON.stringify(data))

    return data
  }

  document.addEventListener("keyup", (event) => {
    if (event.key != "/") return

    searchInput.focus()
  })

  document.addEventListener("keypress", (event) => {
    if (event.key.toUpperCase() != "F" || !event.shiftKey) return

    shown = !shown
  })
</script>

<div class="fixed top-6 z-50 left-6 right-6">
  <input
    bind:this={searchInput}
    bind:value={query}
    type="text"
    class="w-full outline-none bg-black/50 rounded-xl px-3 py-1 caret-white"
    placeholder="Press / to focus"
    on:keypress={async (event) => {
      if (event.key != "Enter") return
      loading = true

      const res = await searchEmojis(query)
      emojis = res.results

      loading = false
    }}
  />
</div>
<div class="py-6" />
<div class="grid grid-cols-2 gap-4">
  {#if loading}
    <p class="text-base">Loading..</p>
  {:else}
    {#each emojis as emoji}
      <EmojiItem emoji={emoji.emoji} name={String(emoji.name).split(":")[0]} />
    {/each}
  {/if}
</div>

<div class="fixed bottom-4 right-4" data-tauri-drag-region>
  <svg
    data-tauri-drag-region
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
    stroke-width="1.5"
    stroke="currentColor"
    class="w-6 h-6 cursor-pointer rounded-full transition duration-300 scale-90 stroke-white/50 hover:scale-100 hover:stroke-white/70"
  >
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      d="M3.75 9h16.5m-16.5 6.75h16.5"
    />
  </svg>
</div>

<SettingWindow
  {shown}
  onCloseRequested={() => {
    shown = false
  }}
/>
