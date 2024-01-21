<script>
	import { _ } from "svelte-i18n";
	import { writable } from "svelte/store";

    export let isOpen = writable(false);

    function closeModal() {
        isOpen.set(false);
    }

    document.addEventListener('keyup', (e) => {
        if (e.key === 'Escape') {
            closeModal();
        }
    });
</script>

{#if $isOpen}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="modal" on:click={closeModal}>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
        <div class="modal-content" on:click={(e) => e.stopPropagation()} role="dialog" aria-modal="true">
            <h2>{$_("settings.base")}</h2>
            <slot></slot>
        </div>
    </div>
{/if}

<style>
    .modal {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: rgba(0, 0, 0, 0.7);
        z-index: 9999;
    }

    .modal-content {
        padding: 20px;
        border-radius: 4px;
    }
</style>
