<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
    import { _ } from "svelte-i18n";

    let oldPassword = '';
    let newPassword1 = '';
    let newPassword2 = '';

    function handleChangePassword() {
        if (newPassword1 !== newPassword2) {
            alert($_('masterpassword.match'));
            return;
        }

        invoke('change_master_password', {
            oldPw: oldPassword,
            newPw: newPassword1
        }).then((res: any) => {
            if (res) {
                alert($_('masterpassword.success'));
                location.href = '/';
            } else {
                alert($_('masterpassword.wrongold'));
            }
        });
    }
</script>

<main>
    <h1>{$_("settings.change.masterpassword")}</h1>

    <form on:submit|preventDefault={handleChangePassword}>
        <div class="row">
            <label for="oldPassword">{$_("masterpassword.old")}</label>
            <input type="password" id="oldPassword" placeholder={$_("masterpassword.old")} bind:value={oldPassword} />
        </div>

        <div class="row">
            <label for="newPassword1">{$_("masterpassword.new")}</label>
            <input type="password" id="newPassword1" placeholder={$_("masterpassword.new")} bind:value={newPassword1} />
        </div>

        <div class="row">
            <label for="newPassword2">{$_("masterpassword.repeatnew")}</label>
            <input type="password" id="newPassword2" placeholder={$_("masterpassword.repeatnew")} bind:value={newPassword2} />
        </div>

        <button type="submit" class="btn btn-danger">{$_("settings.change.masterpassword")}</button>
        <button type="button" class="btn btn-primary" on:click={() => location.href = '/'}>{$_("settings.cancel")}</button>
    </form>
</main>

<style>
    .row {
        display: flex;
        flex-direction: column;
        margin-bottom: 10px;
    }
</style>
