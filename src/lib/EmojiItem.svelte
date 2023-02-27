<script lang="ts">
  import { writeText } from "@tauri-apps/api/clipboard"

  let snack: HTMLDivElement

  export let emoji: string
  export let name: string
</script>

<button
  on:click={async () => {
    await writeText(emoji)
    snack.animate([{ opacity: "1" }, { opacity: "0" }], {
      duration: 1000,
    })
  }}
  class="relative flex flex-col items-center text-center justify-center p-4 rounded-xl cursor-pointer transition duration-300 hover:bg-black/40 focus:outline-white"
>
  <p class="text-5xl">{emoji}</p>
  <div class="py-1.5" />
  <p class="text-xs">{name}</p>
  <div
    bind:this={snack}
    class="absolute top-0 right-0 bottom-0 left-0 w-full h-full bg-black/50 rounded-xl grid place-items-center opacity-0"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="2"
      stroke="currentColor"
      class="w-8 h-8"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M4.5 12.75l6 6 9-13.5"
      />
    </svg>
  </div>
</button>
