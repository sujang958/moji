<script lang="ts">
  import { isEnabled, enable, disable } from "tauri-plugin-autostart-api"

  export let shown: boolean = false
  export let onCloseRequested: VoidFunction = () => {}
</script>

<div
  class={`w-full h-full top-0 left-0 right-0 bottom-0 z-50 fixed grid place-items-center p-8 ${
    !shown ? "hidden" : "visible"
  }`}
>
  <div
    class="relative flex flex-col w-full h-full text-black bg-white rounded-xl p-4"
  >
    <div class="absolute top-2.5 right-2.5">
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        class="w-6 h-6 stroke-black/80 transition duration-300 hover:stroke-black cursor-pointer scale-90 hover:scale-95 rounded-full hover:bg-black/10"
        on:click={() => {
          onCloseRequested()
        }}
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M6 18L18 6M6 6l12 12"
        />
      </svg>
    </div>
    {#await isEnabled()}
      loading
    {:then yes}
      <label class="font-bold">
        Auto-start on startup <br />
        <input
          class="w-3.5 h-3.5"
          checked={yes}
          type="checkbox"
          on:change={(event) => {
            if (!(event.target instanceof HTMLInputElement)) return

            if (event.target.checked) enable()
            else disable()
          }}
        />
      </label>
    {/await}
  </div>
</div>
