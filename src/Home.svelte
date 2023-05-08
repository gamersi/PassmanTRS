<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import {listen} from '@tauri-apps/api/event';
    import { writable } from 'svelte/store';
    import type { Password } from './utils/types';
    import FaPlus from 'svelte-icons/fa/FaPlus.svelte'
    import FaEye from 'svelte-icons/fa/FaEye.svelte';
    import FaPen from 'svelte-icons/fa/FaPen.svelte';
    import FaTrash from 'svelte-icons/fa/FaTrash.svelte';

    const passwords = writable<Password[]>([]);
    passwords.subscribe((value) => {
        console.log('passwords have been updated:', value);
    });

    function getPasswords() {
        invoke('get_passwords').then((res: Password[]) => {
            res.forEach((password) => {
                password.name = password.name.replace(/^"(.*)"$/, '$1');
                password.username = password.username.replace(/^"(.*)"$/, '$1');
                password.password = password.password.replace(/^"(.*)"$/, '$1');
                password.url = password.url.replace(/^"(.*)"$/, '$1');
                password.notes = password.notes.replace(/^"(.*)"$/, '$1');
            });
            passwords.set(res);
        });
    }

    const unlisten = listen<string>('refresh_passwords', (event) => {
        console.log('refresh_passwords event received:', event.payload);
        getPasswords();
    });

    getPasswords();

    function openAddPassword() {
        invoke('open_add_password');
    }
</script>

<main class="container">
    <div class="row">
        <h1>Passwörter</h1>
        <button class="btn btn-icon btn-primary" on:click={openAddPassword}><FaPlus /></button>
    </div>
    <div class="contents">
        {#each $passwords as password}
        <div class="card">
            <div class="userInfo">
                <h1 class="title">{password.name || "Kein Name"}</h1>
                <p class="username">{password.username || "Kein Benutzername"}</p>
            </div>
            <div class="cardBtns">
                <button class="btn btn-icon btn-primary"><FaEye /></button>
                <button class="btn btn-icon btn-primary"><FaPen /></button>
                <button class="btn btn-icon btn-danger" on:click={
                    () => {
                        invoke('delete_password', {id: password.id}).then(() => {
                            getPasswords();
                        });
                    }
                }><FaTrash /></button>
            </div>
        </div>
        {/each}
    </div>
</main>

<style>
    .contents {
        margin-top: 20px;
    }
    .row {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .btn {
        margin: 0 10px;
    }

    .card {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        width: 90%;
        /* height: 200px; */
        border-radius: 10px;
        padding: 10px;
        margin: 10px;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
    }

    .title {
        font-size: 20px;
        font-weight: bold;
    }

    .username {
        color: #888888;
        font-size: 15px;
    }

    .cardBtns {
        display: flex;
        justify-content: space-between;
        align-items: center;
        width: 100%;
    }

    .btn-icon {
        width: 30px !important;
        height: 30px !important;
        /* border-radius: 50%;
        display: flex;
        justify-content: center;
        align-items: center; */
    }

    .btn-primary {
        background-color: #007bff;
        color: white !important;
    }

    .btn-danger {
        background-color: #dc3545;
        color: white;
    }

    .btn-primary:hover {
        background-color: #0069d9;
    }

    .btn-danger:hover {
        background-color: #c82333;
    }

    .btn-primary:active {
        background-color: #0062cc;
    }

    .btn-danger:active {
        background-color: #bd2130;
    }


</style>