<script>
  import Card from "$lib/Card.svelte";
import { invoke } from "@tauri-apps/api/core";
  import { _ } from 'svelte-i18n';
  import Icon from 'mdi-svelte';
  import { mdiArrowLeft } from '@mdi/js';


  let homebrews = {
    palg: {
      classes: [
      {name: "Cleric Light"},
      {name: "Random"},
    ],
    races: [
      {name: "Wasdf"},
      {name: "Azevdf"},
    ],
    skills: [
      {name: "Crafting"},
    ],
    backgrounds: [
      {name: "Test bg"},
    ]
    }
  }

  let sheets = [
    {
      name: "Emily Solis",
      classes: ["light_cleric", "barbarian"],
      totalLevel: 5
    }
  ]

  async function loadSheets() {
    sheets = await invoke('get_sheets')
  }

  async function loadHomebrews() {
    homebrews = await invoke('get_homebrews')
    console.log(homebrews)
  }

  loadHomebrews()
  loadSheets()
</script>

<div>
  <div class="header">
    <button on:click={() => {
      window.history.back();
    }}><Icon path={mdiArrowLeft} size={1.3} /></button>
    <h1>{$_("settings.title")}</h1>
  </div>

  <!-- <h2>Homebrew content</h2>
  <div class="homebrew-holder">
    {#each Object.keys(homebrews) as key}
    <div class="homebrew-content">
      <h3>{key}</h3>
      <b>Classes:</b>
      <ul>
        {#each homebrews[key].classes as cl}
          <li>{cl}</li>
        {/each}
      </ul>
      <b>Races:</b>
      <ul>
        {#each homebrews[key].races as race}
          <li>{race}</li>
        {/each}
      </ul>
      <b>Skills:</b>
      <ul>
        {#each homebrews[key].skills as skill}
          <li>{skill}</li>
        {/each}
      </ul>
      <b>Backgrounds:</b>
      <ul>
        {#each homebrews[key].backgrounds as background}
          <li>{background}</li>
        {/each}
      </ul>
    </div>
  {/each}
  </div> -->
  <h2>{$_("settings.characters")}</h2>
  {#each sheets as sheet}
    <div style="padding: 1rem">
      <Card title={sheet.character_name}>
        <div class="character-card">
          <span>{$_(`classes.${sheet.classes[0].name}`)}, {sheet.total_level}</span>
          <span>HP: 12/18</span>
          <div class="button-row">
            <div>
              <button on:click={() => {
                invoke('change_sheet', {name: sheet.character_name})
              }}>Load</button>
              <button>Edit</button>
            </div>
            <button>Delete</button>
          </div>
        </div>
      </Card>
    </div>
  {/each}
  <a href="/create-character">Create a character</a>

  <h2>{$_("settings.homebrews")}</h2>
  {#each Object.keys(homebrews) as key}
  <div style="padding: 1rem">
    <Card title={$_(`homebrew.${key}`)}>
      <div class="homebrew-card">
        <div>
          <b>Classes:</b><!-- TODO translation -->
          <ul>
            {#each homebrews[key].classes as cl}
              <li>{cl}</li>
            {/each}
          </ul>
          <b>Races:</b>
          <ul>
            {#each homebrews[key].races as race}
              <li>{race}</li>
            {/each}
          </ul>
          <b>Skills:</b>
          <ul>
            {#each homebrews[key].skills as skill}
              <li>{skill}</li>
            {/each}
          </ul>
          <b>Backgrounds:</b>
          <ul>
            {#each homebrews[key].backgrounds as background}
              <li>{background}</li>
            {/each}
          </ul>
        </div>
        <div>
          Used by:
          <ul>
            <li>Placeholder 1</li><!-- TODO -->
            <li>Placeholder 2</li>
          </ul>
        </div>
      </div>
    </Card>
  </div>

  {/each}
</div>

<style>
  h2 {
    margin: 0rem 1rem;
  }

  .header {
    display: flex;
    gap: 1em;
  }

  .character-card {
    display: flex;
    flex-direction: column;
  }

  .button-row {
    display: flex;
    justify-content: space-between;
  }

  .homebrew-card {
    display: flex;
  }
</style>
