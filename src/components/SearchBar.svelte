<script lang="ts">
    import { _ } from "svelte-i18n";

    export let search = "";
</script>

<input type="text" placeholder={$_("start.search")} bind:value={search} />
