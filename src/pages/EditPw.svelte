<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
	import type { Password, Block } from '../utils/types';
    import { parseURL } from '../utils/utillities';
    import { _ } from 'svelte-i18n';

    // @ts-ignore
    const isTauri = typeof window !== "undefined" && window.__TAURI__;
    
    const masterPassword = localStorage.getItem('masterPassword')

    const passwordId = new URLSearchParams(window.location.search).get('id')

    let password: Password = {
        id: 0,
        name: 'Loading...',
        username: 'Loading...',
        password: {
            nonce: 'Loading...',
            data: 'Loading...'
        },
        url: 'Loading...',
        notes: 'Loading...',
        decrypted_password: 'Loading...'
    }

    if (!isTauri) {
        password = {
            id: 0,
            name: 'Test',
            username: 'Test',
            password: {
                nonce: 'Test',
                data: 'Test'
            },
            url: 'Test',
            notes: 'Test',
            decrypted_password: 'Test'
        }
    } else {
        if (passwordId != null) {
            invoke('get_password', {id: parseInt(passwordId), masterPassword}).then((res: any) => {
                let password_res = res as Password;
                password_res.name = password_res.name.replace(/^"(.*)"$/, '$1');
                password_res.username = password_res.username.replace(/^"(.*)"$/, '$1');
                password_res.decrypted_password = password_res.decrypted_password ? password_res.decrypted_password.replace(/^"(.*)"$/, '$1') : '';
                password_res.url = password_res.url.replace(/^"(.*)"$/, '$1');
                password_res.notes = password_res.notes.replace(/^"(.*)"$/, '$1');
                password = password_res
            })
        } else {
            alert($_('editpw.noid'))
        }
    }

    function editPassword(event: any) {
        if (!isTauri) {
            alert($_('settings.nobrowsersupport'))
            return
        }
        if (passwordId == null) {
            alert($_('editpw.noid'))
            return
        }
        invoke('edit_password', {
            id: parseInt(passwordId),
            name: event.target.name.value,
            username: event.target.username.value,
            password: event.target.password.value,
            url: parseURL(event.target.url.value),
            notes: event.target.notes.value,
            masterPassword
        }).then((res: any) => {
            console.log(res)

            invoke('close_edit_password')
        })
    }
</script>

<main class="container">
    <h1>{$_("editpw.base")}</h1>
    <form on:submit|preventDefault={editPassword}>
        <div class="row">
            <label for="name" class="form-label">{$_("editpw.name")}</label>
            <input type="text" class="form-control" id="name" placeholder={$_("editpw.name")} value={password.name}>
        </div>
        <div class="row">
            <label for="username" class="form-label">{$_("editpw.username")}</label>
            <input type="text" class="form-control" id="username" placeholder={$_("editpw.username")} value={password.username}>
        </div>
        <div class="row">
            <label for="password" class="form-label">{$_("editpw.password")}</label>
            <input type="text" class="form-control" id="password" placeholder={$_("editpw.password")} value={password.decrypted_password}>
        </div>
        <div class="row">
            <label for="url" class="form-label">{$_("editpw.url")}</label>
            <input type="text" class="form-control" id="url" placeholder={$_("editpw.url")} value={password.url}>
        </div>
        <div class="row">
            <label for="notes" class="form-label">{$_("editpw.notes")}</label>
            <input type="text" class="form-control" id="notes" placeholder={$_("editpw.notes")} value={password.notes}>
        </div>
        <button type="submit" class="btn btn-primary">{$_("editpw.base")}</button>
        <button type="button" class="btn" on:click={() => invoke('close_edit_password')}>{$_("settings.cancel")}</button>
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