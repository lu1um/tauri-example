<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

  let count: number = $state(0)
  const increment = async () => {
    console.log("Incrementing counter...");
    count = await invoke("increment", { counter: count });
  }
</script>

<button class="counter" onclick={increment}>
  Count is {count}
</button>
