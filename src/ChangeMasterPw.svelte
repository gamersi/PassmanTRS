<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

    let oldPassword = '';
    let newPassword1 = '';
    let newPassword2 = '';

    function handleChangePassword() {
        if (newPassword1 !== newPassword2) {
            alert('Die neuen Passwörter stimmen nicht überein');
            return;
        }

        invoke('change_master_password', {
            oldPw: oldPassword,
            newPw: newPassword1
        }).then((res: any) => {
            if (res) {
                alert('Das Masterpassword wurde erfolgreich geändert');
                location.href = '/';
            } else {
                alert('Das alte Masterpassword ist falsch');
                
            }
        });
    }
</script>

<main>
    <h1>Masterpassword ändern</h1>

    <form on:submit|preventDefault={handleChangePassword}>
        <div class="row">
            <label for="oldPassword">Altes Masterpassword</label>
            <input type="password" id="oldPassword" placeholder="Altes Masterpassword" bind:value={oldPassword} />
        </div>

        <div class="row">
            <label for="newPassword1">Neues Masterpassword:</label>
            <input type="password" id="newPassword1" placeholder="Neues Masterpassword" bind:value={newPassword1} />
        </div>

        <div class="row">
            <label for="newPassword2">Neues Masterpassword wiederholen:</label>
            <input type="password" id="newPassword2" placeholder="Neues Masterpassword wiederholen" bind:value={newPassword2} />
        </div>

        <button type="submit" class="btn btn-danger">Masterpassword ändern</button>
        <button type="button" class="btn btn-primary" on:click={() => location.href = '/'}>Abbrechen</button>
    </form>
</main>

<style>
    .row {
        display: flex;
        flex-direction: column;
        margin-bottom: 10px;
    }
</style>
