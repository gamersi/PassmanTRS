<script>
	import {availableLocales, getLocale, setLocale} from "../locales/i18n";

	let selectedLanguage = getLocale();
    console.log(selectedLanguage);

	function changeLanguage() {
		setLocale(selectedLanguage);
	}
</script>

<div class="custom-select">
	<select bind:value={selectedLanguage} on:change={changeLanguage}>
		{#each availableLocales as locale}
			<option value={locale}>{locale}</option>
		{/each}
	</select>
</div>

<style>
	select {
		appearance: none;
		background-color: transparent;
		border: none;
		margin: 0;
        padding: 10px;
        width: 100%;
        height: 100%;
		font-family: inherit;
		font-size: inherit;
        cursor: inherit;
		outline: none;
        color: inherit;
	}

	.custom-select {
        display: inline-block;
		border-radius: 8px;
        margin: 0 10px;
		cursor: pointer;
        border: 1px solid transparent;
        color: var(--color);
        background-color: var(--btn-color);
        transition: all 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
        font-size: 1em;
	}
</style>
