<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
	import { parseURL } from '../utils/utilities';
    import { _ } from 'svelte-i18n';
    // @ts-ignore
    import MdRefresh from 'svelte-icons/md/MdRefresh.svelte';

    // @ts-ignore
    const isTauri = typeof window !== "undefined" && window.__TAURI__;

    const masterPassword = localStorage.getItem('masterPassword')
    
    function addPassword(event: any) {
        if (!isTauri) {
            alert($_("settings.nobrowsersupport"))
            return
        }
        invoke('add_password', {
            name: event.target.name.value,
            username: event.target.username.value,
            password: event.target.password.value,
            url: parseURL(event.target.url.value),
            notes: event.target.notes.value,
            masterPassword
        }).then((res: any) => {
            console.log(res)
            invoke('close_add_password')
        })
    }

    function closeWindow() {
        if (!isTauri) {
            alert($_("settings.nobrowsersupport"))
            return
        }
        invoke('close_add_password')
    }
</script>

<main class="container">
    <h1>{$_("addpw.base")}</h1>
    <form on:submit|preventDefault={addPassword}>
        <div class="row">
            <label for="name" class="form-label">{$_("addpw.name")}</label>
            <input type="text" class="form-control" id="name" placeholder={$_("addpw.name")}>
        </div>
        <div class="row">
            <label for="username" class="form-label">{$_("addpw.username")}</label>
            <input type="text" class="form-control" id="username" placeholder={$_("addpw.username")}>
        </div>
        <div class="row">
            <label for="password" class="form-label">{$_("addpw.password")}</label>
            <input type="text" class="form-control" id="password" placeholder={$_("addpw.password")}>
            <button type="button" class="btn btn-icon" on:click={() => invoke('open_generator')}><MdRefresh /></button>
        </div>
        <div class="row">
            <label for="url" class="form-label">{$_("addpw.url")}</label>
            <input type="text" class="form-control" id="url" placeholder={$_("addpw.url")}>
        </div>
        <div class="row">
            <label for="notes" class="form-label">{$_("addpw.notes")}</label>
            <input type="text" class="form-control" id="notes" placeholder={$_("addpw.notes")}>
        </div>
        <button type="submit" class="btn btn-primary">{$_("addpw.add")}</button>
        <button type="button" class="btn" on:click={closeWindow}>{$_("settings.cancel")}</button>
    </form>
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
        margin-top: 20px;
    }
    
    .row {
        display: flex;
        justify-content: space-between;
        margin: 10px;
        gap: 10px;
        align-items: center;
        margin-bottom: 20px;
    }
</style>