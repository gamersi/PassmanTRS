<script lang="ts">
    import { _ } from "svelte-i18n";

    export let passwordStrength = 0;
</script>

<div class="password-strength-meter">
    <input
        type="range"
        min="0"
        max="5"
        step="1"
        disabled
        value={passwordStrength}
        class:password-strength-0={passwordStrength === 0}
        class:password-strength-1={passwordStrength === 1}
        class:password-strength-2={passwordStrength === 2}
        class:password-strength-3={passwordStrength === 3}
        class:password-strength-4={passwordStrength === 4}
        class:password-strength-5={passwordStrength === 5}
    />
    <p>{passwordStrength}</p>
</div>

<style>
    :root {
        --password-strength-0: #ff0000;
        --password-strength-1: #ff8000;
        --password-strength-2: #ffff00;
        --password-strength-3: #80ff00;
        --password-strength-4: #00ff00;
        --password-strength-5: #00ff80;
    }

    .password-strength-meter {
        display: flex;
        align-items: center;
    }

    p {
        margin-left: 10px;
    }

    input {
        -webkit-appearance: none;
        appearance: none;
        width: 100%;
        height: 10px;
        border-radius: 5px;
        background: var(--password-strength-0);
        outline: none;
        opacity: 0.7;
        transition: opacity 0.2s;
    }

    input::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 20px;
        height: 20px;
        border-radius: 50%;
        background: var(--password-strength-4);
        cursor: pointer;
    }

    input:disabled {
        opacity: 1;
    }

    input:disabled::-webkit-slider-thumb {
        background: transparent;
    }

    .password-strength-0 {
        background: var(--password-strength-0);
    }

    .password-strength-1 {
        background: var(--password-strength-1);
    }

    .password-strength-2 {
        background: var(--password-strength-2);
    }

    .password-strength-3 {
        background: var(--password-strength-3);
    }

    .password-strength-4 {
        background: var(--password-strength-4);
    }

    .password-strength-5 {
        background: var(--password-strength-5);
    }
</style>
