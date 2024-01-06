<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
	import type { Password } from './utils/types';

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
        if (passwordId == null) {
            alert('Keine ID vorhanden')
        } else {
            invoke('get_password', {id: parseInt(passwordId), masterPassword}).then((res) => {
                let password_res = res as Password;
                password_res.name = password_res.name.replace(/^"(.*)"$/, '$1');
                password_res.username = password_res.username.replace(/^"(.*)"$/, '$1');
                password_res.decrypted_password = password_res.decrypted_password ? password_res.decrypted_password.replace(/^"(.*)"$/, '$1') : 'Kein Passwort';
                password_res.url = password_res.url.replace(/^"(.*)"$/, '$1');
                password_res.notes = password_res.notes.replace(/^"(.*)"$/, '$1');
                // replace empty strings with n/a
                password_res.name = password_res.name === '' ? 'n/a' : password_res.name;
                password_res.username = password_res.username === '' ? 'n/a' : password_res.username;
                password_res.decrypted_password = password_res.decrypted_password === '' ? 'n/a' : password_res.decrypted_password;
                password_res.url = password_res.url === '' ? 'n/a' : password_res.url;
                password_res.notes = password_res.notes === '' ? 'n/a' : password_res.notes;
                password = password_res;
            })
        }
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
        <input type="text" class="form-control" id="password" placeholder="Passwort" value="{password.decrypted_password}" readonly>
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