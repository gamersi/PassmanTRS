<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
	import type { Password } from './utils/types';

    // @ts-ignore
    const isTauri = typeof window !== "undefined" && window.__TAURI__;

    // get password id from url params
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
            // replace empty strings with n/a
            res.name = res.name === '' ? 'n/a' : res.name
            res.username = res.username === '' ? 'n/a' : res.username
            res.password = res.password === '' ? 'n/a' : res.password
            res.url = res.url === '' ? 'n/a' : res.url
            res.notes = res.notes === '' ? 'n/a' : res.notes
            password = res
        })
    }

    function closeWindow() {
        if (!isTauri) {
            alert('Dieser Button funktioniert nur in der Tauri App')
            return
        }
        invoke('close_view_password')
    }
</script>

<main class="container">
    <h1>Passwort ansehen</h1>
    <div class="mb-3">
        <label for="name" class="form-label">Name</label>
        <input type="text" class="form-control" id="name" placeholder="Name" value="{password.name}" readonly>
    </div>
    <div class="mb-3">
        <label for="username" class="form-label">Benutzername</label>
        <input type="text" class="form-control" id="username" placeholder="Benutzername" value="{password.username}" readonly>
    </div>
    <div class="mb-3">
        <label for="password" class="form-label">Passwort</label>
        <input type="text" class="form-control" id="password" placeholder="Passwort" value="{password.password}" readonly>
    </div>
    <div class="mb-3">
        <label for="url" class="form-label">URL</label>
        <input type="text" class="form-control" id="url" placeholder="URL" value="{password.url}" readonly>
    </div>
    <div class="mb-3">
        <label for="notes" class="form-label">Notizen</label>
        <input type="text" class="form-control" id="notes" placeholder="Notizen" value="{password.notes}" readonly>
    </div>
    <button type="button" class="btn btn-primary" on:click="{() => closeWindow()}">Schließen</button>
</main>

<style>
    .mb-3 {
        margin-bottom: 5px;
    }
</style>