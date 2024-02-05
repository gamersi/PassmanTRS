<script lang="ts">
  import { message } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import { _ } from "svelte-i18n";
  // @ts-ignore
  import FaCopy from "svelte-icons/fa/FaCopy.svelte";
  // @ts-ignore
  import FaHistory from "svelte-icons/fa/FaHistory.svelte";
  // @ts-ignore
  import FaTrash from "svelte-icons/fa/FaTrash.svelte";
  import { writable } from "svelte/store";

  const generatedPassword = writable("password");

  const masterPassword = localStorage.getItem("masterPassword");

  const generatedPasswords = writable([]);

  // @ts-ignore
  const isTauri = typeof window !== "undefined" && window.__TAURI__;

  function handleSubmit(event: any) {
    event.preventDefault();
    const length: number = parseInt(event.target.length.value);
    const minUppercase: number = parseInt(event.target.uppercase.value);
    const minLowercase: number = parseInt(event.target.lowercase.value);
    const minNumbers: number = parseInt(event.target.numbers.value);
    const minSymbols: number = parseInt(event.target.symbols.value);

    if (minUppercase + minLowercase + minNumbers + minSymbols > length) {
      message($_("generate.error"));
      return;
    }
    invoke("generate_password", {
      length,
      options: {
        min_lowercase: minLowercase,
        min_uppercase: minUppercase,
        min_numbers: minNumbers,
        min_symbols: minSymbols,
      },
      masterPassword,
    }).then((res) => {
      generatedPassword.set(res as string);
      refreshHistory();
    });
  }

  function closeWindow() {
    if (!isTauri) {
      alert($_("settings.nobrowsersupport"));
      return;
    }
    invoke("close_generator");
  }

  invoke("generate_password", {
    length: 10,
    options: {
      min_lowercase: 1,
      min_uppercase: 1,
      min_numbers: 1,
      min_symbols: 1,
    },
    masterPassword,
  }).then((res) => {
    generatedPassword.set(res as string);
    refreshHistory();
  });

  function clearHistory() {
    invoke("clear_generator_history", { masterPassword }).then(() => {
      refreshHistory();
    });
  }

  function toggleHistory() {
    let ul: any = document.querySelector(".historyList");
    if (ul != null) {
      if (
        ul.style.display === "none" ||
        ul.style.display === "" ||
        ul.style.display === null
      ) {
        ul.style.display = "block";
        ul.parentElement.style.height = ul.parentElement.scrollHeight + "px";
      } else {
        ul.parentElement.style.height = "50px";
        setTimeout(() => {
          ul.style.display = "none";
        }, 300);
      }
    }
  }

  function refreshHistory() {
    invoke("get_generator_history", { masterPassword }).then((res: any) => {
      generatedPasswords.set(res);
        let ul: any = document.querySelector(".historyList");
        if (ul != null) {
          ul.parentElement.style.height = "50px";
          setTimeout(() => {
            ul.style.display = "none";
          }, 300);
        }
    });
  }
</script>

<main class="container">
  <h1>{$_("generate.base")}</h1>
  <div class="passwordField">
    <p>{$generatedPassword}</p>
    <button
      type="button"
      class="btn btn-icon"
      on:click={() => navigator.clipboard.writeText($generatedPassword)}
      ><FaCopy /></button
    >
  </div>
  <form on:submit={handleSubmit}>
    <div class="form-group">
      <label for="length">{$_("generate.length")}</label>
      <input
        type="number"
        id="length"
        name="length"
        min="1"
        max="120"
        value="10"
        required
      />
    </div>
    <div class="form-group">
      <label for="uppercase">{$_("generate.uppercase")}</label>
      <input
        type="number"
        id="uppercase"
        name="uppercase"
        min="0"
        max="120"
        value="1"
        required
      />
    </div>
    <div class="form-group">
      <label for="lowercase">{$_("generate.lowercase")}</label>
      <input
        type="number"
        id="lowercase"
        name="lowercase"
        min="0"
        max="120"
        value="1"
        required
      />
    </div>
    <div class="form-group">
      <label for="numbers">{$_("generate.numbers")}</label>
      <input
        type="number"
        id="numbers"
        name="numbers"
        min="0"
        max="120"
        value="1"
        required
      />
    </div>
    <div class="form-group">
      <label for="symbols">{$_("generate.symbols")}</label>
      <input
        type="number"
        id="symbols"
        name="symbols"
        min="0"
        max="120"
        value="1"
        required
      />
    </div>
    <button type="submit" id="genSubmit" class="btn btn-primary"
      >{$_("generate.generate")}</button
    >
    <button type="button" class="btn" on:click={closeWindow}
      >{$_("settings.close")}</button
    >
  </form>
  <div class="history">
    <button type="button" class="btn btn-icon btn-primary" on:click={() => toggleHistory()}
      ><FaHistory /></button
    >
    <ul class="historyList">
      {#if $generatedPasswords.length === 0}
        <li>
          <p>{$_("start.placeholder.nopasswords")}</p>
        </li>
      {:else}
        <li>
          <button
            type="button"
            class="btn btn-icon btn-danger"
            on:click={() => clearHistory()}
          >
            <FaTrash />
          </button>
        </li>
      {/if}
      {#each $generatedPasswords as password}
        <li>
          <p>{password[0]}</p>
          <br>
          <span style="color: #888888;">{password[1]}</span>
          <div class="btnGroup">
            <button
              type="button"
              class="btn btn-icon"
              on:click={() => navigator.clipboard.writeText(password[0])}
              ><FaCopy /></button
            >
            <button
              type="button"
              class="btn btn-icon btn-danger"
              on:click={() => {
                invoke("delete_generator_history", {
                  masterPassword,
                  password: password[0],
                }).then(() => {
                  refreshHistory();
                });
              }}><FaTrash /></button
            >
          </div>
        </li>
      {/each}
    </ul>
  </div>
</main>

<style>

main {
    min-width: 400px;
}

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

  .history {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    margin-top: 20px;
    transition: height 0.3s;
    overflow: hidden;
    height: 50px;
  }

  .historyList {
    display: none;
    list-style: none;
    padding: 0;
    line-height: 1.5;
    padding: 5px;
    background-color: var(--card-background-color);
    border-radius: 10px;
  }

  .historyList li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #888;
  }

  .btnGroup {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
</style>
