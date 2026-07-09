<script lang="ts">
    import { onMount } from "svelte";

    let time = $state(new Date());

    onMount(() => {
        setInterval(() => {
            time = new Date();
        }, 1000);
    });
</script>

<h1>NOT FOUND</h1>
{time}
