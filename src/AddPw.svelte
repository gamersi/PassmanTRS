<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    function addPassword(event) {
        invoke('add_password', {
            name: event.target.name.value,
            username: event.target.username.value,
            password: event.target.password.value,
            url: event.target.url.value,
            notes: event.target.notes.value
        }).then((res) => {
            console.log(res)
            // close window
            invoke('close_add_password')
        })
    }
</script>

<main class="container">
    <h1>Passwort hinzufügen</h1>
    <form on:submit|preventDefault={addPassword}>
        <div class="mb-3">
            <label for="name" class="form-label">Name</label>
            <input type="text" class="form-control" id="name" placeholder="Name">
        </div>
        <div class="mb-3">
            <label for="username" class="form-label">Benutzername</label>
            <input type="text" class="form-control" id="username" placeholder="Benutzername">
        </div>
        <div class="mb-3">
            <label for="password" class="form-label">Passwort</label>
            <input type="text" class="form-control" id="password" placeholder="Passwort">
        </div>
        <div class="mb-3">
            <label for="url" class="form-label">URL</label>
            <input type="text" class="form-control" id="url" placeholder="URL">
        </div>
        <div class="mb-3">
            <label for="notes" class="form-label">Notizen</label>
            <input type="text" class="form-control" id="notes" placeholder="Notizen">
        </div>
        <button type="submit" class="btn btn-primary">Passwort hinzufügen</button>
    </form>
</main>

<style>
    .mb-3 {
        margin-bottom: 5px;
    }
</style>