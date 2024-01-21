<script lang="ts">
	import { message } from '@tauri-apps/api/dialog';
    import { invoke } from '@tauri-apps/api/tauri'
	import generatePassword from '../utils/generator';
	import type { GeneratorOptions } from 'src/utils/types';
	import { _ } from 'svelte-i18n';
    // @ts-ignore
    import FaCopy from 'svelte-icons/fa/FaCopy.svelte';
    import { writable } from 'svelte/store';

    const generatedPassword = writable('password');

    // @ts-ignore
    const isTauri = typeof window !== "undefined" && window.__TAURI__;

    function handleSubmit(event: any) {
        event.preventDefault()
        const length = event.target.length.value
        const minUppercase: number = parseInt(event.target.uppercase.value)
        const minLowercase: number = parseInt(event.target.lowercase.value)
        const minNumbers: number = parseInt(event.target.numbers.value)
        const minSymbols: number = parseInt(event.target.symbols.value)
        const options: GeneratorOptions = {
            minUppercase,
            minLowercase,
            minNumbers,
            minSymbols
        }
        if (minUppercase + minLowercase + minNumbers + minSymbols > length) {
            message($_('generate.error'))
            return
        }
        generatedPassword.set(generatePassword(length, options));
    }

    function closeWindow() {
        if (!isTauri) {
            alert($_('settings.nobrowsersupport'))
            return
        }
        invoke('close_generator')
    }

    generatedPassword.set(generatePassword(10, {
        minUppercase: 1,
        minLowercase: 1,
        minNumbers: 1,
        minSymbols: 1
    }))

</script>

<main class="container">
    <h1>{$_("generate.base")}</h1>
    <div class="passwordField">
        <p>{$generatedPassword}</p>
        <button type="button" class="btn btn-icon" on:click={() => navigator.clipboard.writeText($generatedPassword)}><FaCopy /></button>
    </div>
    <form on:submit={handleSubmit}>
        <div class="form-group">
            <label for="length">{$_("generate.length")}</label>
            <input type="number" id="length" name="length" min="1" max="120" value="10" required />
        </div>
        <div class="form-group">
            <label for="uppercase">{$_("generate.uppercase")}</label>
            <input type="number" id="uppercase" name="uppercase" min="0" max="120" value="1" required />
        </div>
        <div class="form-group">
            <label for="lowercase">{$_("generate.lowercase")}</label>
            <input type="number" id="lowercase" name="lowercase" min="0" max="120" value="1" required />
        </div>
        <div class="form-group">
            <label for="numbers">{$_("generate.numbers")}</label>
            <input type="number" id="numbers" name="numbers" min="0" max="120" value="1" required />
        </div>
        <div class="form-group">
            <label for="symbols">{$_("generate.symbols")}</label>
            <input type="number" id="symbols" name="symbols" min="0" max="120" value="1" required />
        </div>
        <button type="submit" id="genSubmit" class="btn btn-primary">{$_("generate.generate")}</button>
        <button type="button" class="btn" on:click={closeWindow}>{$_("settings.close")}</button>
    </form>
</main>

<style>
    .passwordField {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 10px;
    }
    .passwordField p {
        margin: 0;
    }
    .passwordField button {
        margin-left: 10px;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100%;
    }

    .form-group {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 10px;
    }
</style>