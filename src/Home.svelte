<script lang="ts">
	// @ts-nocheck	workaround for TS complaining about the icon imports
	import {invoke} from "@tauri-apps/api/tauri";
	import {listen} from "@tauri-apps/api/event";
	import { open } from '@tauri-apps/api/shell';
	import type { Password } from "./utils/types";
	import FaPlus from "svelte-icons/fa/FaPlus.svelte";
	import FaEye from "svelte-icons/fa/FaEye.svelte";
	import FaPen from "svelte-icons/fa/FaPen.svelte";
	import FaTrash from "svelte-icons/fa/FaTrash.svelte";
	import FaLink from 'svelte-icons/fa/FaLink.svelte';
	import FaLock from 'svelte-icons/fa/FaLock.svelte';
	import FaCopy from 'svelte-icons/fa/FaCopy.svelte';
	import FaCog from 'svelte-icons/fa/FaCog.svelte'
	
	import { passwords, masterPassword, isSettingsOpen } from "./utils/stores";
	import { _ } from "svelte-i18n";
	

    // @ts-ignore
    const isTauri = typeof window !== "undefined" && window.__TAURI__;
	
	passwords.subscribe((value) => {
		console.log("passwords have been updated:", value);
	});

	function getPasswords() {
		let currentMasterPassword = localStorage.getItem("masterPassword");

		invoke("get_passwords", { masterPassword: currentMasterPassword }).then((res) => {
			let passwords_res = res as Password[];
			passwords_res.forEach((password) => {
				password.name = password.name.replace(/^"(.*)"$/, "$1");
				password.username = password.username.replace(/^"(.*)"$/, "$1");
				password.decrypted_password = password.decrypted_password ? password.decrypted_password.replace(/^"(.*)"$/, "$1") : 'Kein Passwort';
				password.url = password.url.replace(/^"(.*)"$/, "$1");
				password.notes = password.notes.replace(/^"(.*)"$/, "$1");
			});
			passwords.set(passwords_res);
		});
	}

    if (!isTauri) {
        passwords.set([
            {
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
        ])
    } else {
		console.log("getting passwords");
        listen<string>("refresh_passwords", (event) => {
            console.log("refresh_passwords event received:", event.payload);
            getPasswords();
        });

        getPasswords();
    }

	function openAddPassword() {
		console.log("opening add password");
        if (!isTauri) {
            window.location.href = "/addPw";
            return;
        }
		invoke("open_add_password");
	}

	function openURL(url: string) {
		open(url);
	}

	function logOut() {
		masterPassword.set("");
		localStorage.removeItem("masterPassword");
		localStorage.removeItem("masterPassword_time");
		console.clear();
		console.log("logged out");
	}
</script>

<main class="container">
	<div class="row">
		<h1>{$_("start.title")}</h1>
		<div class="btn-group">
			<button class="btn btn-icon btn-primary" on:click={() => {
				isSettingsOpen.set(true);
			}}>
				<FaCog />
			</button>
			<button class="btn btn-icon btn-primary" on:click={logOut}>
				<FaLock />
			</button>
			<button class="btn btn-icon btn-primary" on:click={openAddPassword}>
				<FaPlus />
			</button>
		</div>
	</div>
	<div class="contents">
		{#if $passwords.length === 0}
			<p>Keine Passwörter vorhanden</p>
		{/if}
		{#each $passwords as password}
			<div class="card">
				<div class="userInfo">
					<h1 class="title">{password.name || $_("start.placeholder.name")}</h1>
					<p class="username">{password.username || $_("start.placeholder.username")}</p>
				</div>
				<div class="card-btns btn-group">
					{#if password.url}
					<button
						class="btn btn-icon btn-primary"
						on:click={() => {
							if (!isTauri) {
								alert($_("settings.nobrowsersupport"));
								return;
							}
							openURL(password.url);
						}}>
						<FaLink />
					</button>
					{/if}
					<button
						class="btn btn-icon btn-primary"
						on:click={() => {
							if (!isTauri) {
								alert($_("settings.nobrowsersupport"));
								return;
							}
							if (!window.navigator.clipboard) {
								alert($_("start.placeholder.copy"));
								return;
							}
							if (!password.decrypted_password) {
								alert($_("start.placeholder.encrypted"));
								return;
							}
							window.navigator.clipboard.writeText(password.decrypted_password);
						}}>
						<FaCopy />
					</button>
					<button
						class="btn btn-icon btn-primary"
						on:click={() => {
                            if (!isTauri) {
                                window.location.href = "/viewPw";
                                return;
                            }
							invoke("open_view_password", {id: password.id});
						}}>
						<FaEye />
					</button>
					<button
						class="btn btn-icon btn-primary"
						on:click={() => {
                            if (!isTauri) {
                                window.location.href = "/editPw";
                                return;
                            }
							invoke("open_edit_password", {id: password.id});
						}}>
						<FaPen />
					</button>
					<button
						class="btn btn-icon btn-danger"
						on:click={() => {
                            if (!isTauri) {
								alert($_("settings.nobrowsersupport"));
                                return;
                            }
							invoke("delete_password", {id: password.id}).then(() => {
								getPasswords();
							});
						}}>
						<FaTrash />
					</button>
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
	.btn-group {
		display: flex;
		align-items: center;
	}

	.card {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		width: calc(100% - 40px);
		height: 100%;
		border-radius: 10px;
		padding: 10px;
		margin: 10px;
		box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
		transition: all 0.25s;
	}

	.card:hover {
		box-shadow: 0 0 10px rgba(0, 0, 0, 0.4);
	}

	.title {
		font-size: 20px;
		font-weight: bold;
	}

	.username {
		color: #888888;
		font-size: 15px;
	}

	.title, .username {
		min-width: 100px;
		max-width: 200px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.btn-group {
		display: flex;
		align-items: center;
	}

	.card-btns {
		display: flex;
		justify-content: flex-end;
		align-items: center;
		width: 100%;
	}
</style>
