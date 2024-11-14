<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { RustResponse, UserType } from '../types';

  const FAILED_TO_PARSE: RustResponse<string> = {
    status: 'failed',
    data: 'failed to parse JSON',
  };

  let name = $state('');
  let user: RustResponse<UserType | string> | undefined = $state();
  let loginname = $state('');
  let password = $state('');
  let newUser: RustResponse<UserType | string> | undefined = $state();
  let loginnameValidate = $state('');
  let passwordValidate = $state('');
  let validateStatus: RustResponse<string> | undefined = $state();

  async function getUser() {
    const data: string = await invoke('get_user', { name: name.toLowerCase().trim() });
    try {
      user = JSON.parse(data);
    } catch {
      user = FAILED_TO_PARSE;
    }
  }
  async function createUser() {
    const data: string = await invoke('create_user', {
      name: loginname.toLowerCase().trim(),
      password,
    });
    try {
      newUser = JSON.parse(data);
    } catch {
      newUser = FAILED_TO_PARSE;
    }
  }
  async function verifyUser() {
    const data: string = await invoke('verify_user', {
      name: loginnameValidate,
      password: passwordValidate,
    });
    try {
      validateStatus = JSON.parse(data);
    } catch {
      validateStatus = FAILED_TO_PARSE;
    }
  }

  $inspect(user, newUser, validateStatus);
</script>

<div class="container">
  <h1>Welcome to Tauri!</h1>
  <div class="wrapper">
    <form class="row" onsubmit={createUser}>
      <input
        id="name-input"
        title="John is created on migration"
        placeholder="Enter a name..."
        bind:value={name}
      />
      <button type="button" onclick={getUser}>Get User</button>
    </form>
    {#if user && typeof user === 'object' && user.status === 'success'}
      <p class="success">User found</p>
    {/if}
    {#if user && typeof user === 'object' && user.status !== 'success'}
      <p class="error">Failed to find user</p>
    {/if}
  </div>

  <div class="wrapper">
    <form class="row" onsubmit={createUser}>
      <input id="name-input" placeholder="Enter a name..." bind:value={loginname} />
      <input
        id="password-input"
        type="password"
        placeholder="Enter a password..."
        bind:value={password}
      />
      <button type="submit">Create new user</button>
    </form>
    {#if newUser && typeof newUser === 'object' && newUser.status === 'success'}
      <p class="success">User created successfully</p>
    {/if}
    {#if newUser && typeof newUser === 'object' && newUser.status !== 'success'}
      <p class="error">Failed to create user</p>
    {/if}
  </div>

  <div class="wrapper">
    <form class="row" onsubmit={verifyUser}>
      <input
        id="name-validate-input"
        placeholder="Enter a name..."
        bind:value={loginnameValidate}
      />
      <input
        id="password-validate-input"
        placeholder="Enter a password..."
        bind:value={passwordValidate}
        type="password"
      />
      <button type="submit">Validate user</button>
    </form>
    {#if validateStatus && typeof validateStatus === 'object' && validateStatus.status === 'success'}
      <p class="success">User validated</p>
    {:else if validateStatus && typeof validateStatus === 'object' && validateStatus.status !== 'success'}
      <p class="error">Failed to validate</p>
    {/if}
  </div>
</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #d3d3d3;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .wrapper {
    display: flex;
    margin: 0 auto;
    gap: 20px;
    justify-content: center;
    align-items: baseline;
  }

  .row {
    margin: 20px 0;
  }

  .error {
    background: rgba(250, 0, 0, 0.2);
    border-radius: 8px;
    padding: 0.6em 1.2em;
  }
  .success {
    background: rgba(0, 250, 0, 0.2);
    border-radius: 8px;
    padding: 0.6em 1.2em;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    width: 200px;
    height: 40px;
    margin: 0 auto;
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
