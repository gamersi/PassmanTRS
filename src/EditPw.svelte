<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
	import type { Password } from './utils/types';

    // @ts-ignore
    const isTauri = typeof window !== "undefined" && window.__TAURI__;

    const passwordId = new URLSearchParams(window.location.search).get('id')
    let password: Password = {
        id: 0,
        name: 'Loading...',
        username: 'Loading...',
        password: 'Loading...',
        url: 'Loading...',
        notes: 'Loading...'
    }

    if (!isTauri) {
        password = {
            id: 0,
            name: 'Test',
            username: 'Test',
            password: 'Test',
            url: 'Test',
            notes: 'Test'
        }
    } else {
        invoke('get_password', {id: parseInt(passwordId)}).then((res: Password) => {
            res.name = res.name.replace(/^"(.*)"$/, '$1');
            res.username = res.username.replace(/^"(.*)"$/, '$1');
            res.password = res.password.replace(/^"(.*)"$/, '$1');
            res.url = res.url.replace(/^"(.*)"$/, '$1');
            res.notes = res.notes.replace(/^"(.*)"$/, '$1');
            password = res
        })
    }

    function editPassword(event) {
        if (!isTauri) {
            alert('Dieser Button funktioniert nur in der Tauri App')
            return
        }
        invoke('edit_password', {
            id: parseInt(passwordId),
            name: event.target.name.value,
            username: event.target.username.value,
            password: event.target.password.value,
            url: event.target.url.value,
            notes: event.target.notes.value
        }).then((res) => {
            console.log(res)

            // close window
            invoke('close_edit_password')
        })
    }
</script>

<main class="container">
    <h1>Passwort bearbeiten</h1>
    <form on:submit|preventDefault={editPassword}>
        <div class="mb-3">
            <label for="name" class="form-label">Name</label>
            <input type="text" class="form-control" id="name" placeholder="Name" value={password.name}>
        </div>
        <div class="mb-3">
            <label for="username" class="form-label">Benutzername</label>
            <input type="text" class="form-control" id="username" placeholder="Benutzername" value={password.username}>
        </div>
        <div class="mb-3">
            <label for="password" class="form-label">Passwort</label>
            <input type="text" class="form-control" id="password" placeholder="Passwort" value={password.password}>
        </div>
        <div class="mb-3">
            <label for="url" class="form-label">URL</label>
            <input type="text" class="form-control" id="url" placeholder="URL" value={password.url}>
        </div>
        <div class="mb-3">
            <label for="notes" class="form-label">Notizen</label>
            <input type="text" class="form-control" id="notes" placeholder="Notizen" value={password.notes}>
        </div>
        <button type="submit" class="btn btn-primary">Passwort bearbeiten</button>
    </form>
</main>

<style>
    .mb-3 {
        margin-bottom: 5px;
    }
</style>