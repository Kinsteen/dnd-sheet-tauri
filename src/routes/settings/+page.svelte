<script>
  import { invoke } from "@tauri-apps/api/core";


  let homebrews = [
    {
      name: "Prédestinés à la Gloire",
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
    },
    {
      name: "Prédestinés à la Gloire",
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
    },
  ]

  let sheets = [
    {
      name: "Emily Solis",
      classes: ["light_cleric", "barbarian"],
      totalLevel: 5
    }
  ]

  async function loadSheets() {
    sheets = await invoke('get_sheets')
    console.log(sheets)
  }
  // loadHomebrew() // From Rust
  loadSheets() // From Rust
</script>

<div>
  <button on:click={() => {
    window.history.back();
  }}>Back</button>
  <h1>Settings</h1>
  <h2>Homebrew content</h2>
  <div class="homebrew-holder">
    {#each homebrews as homebrew}
    <div class="homebrew-content">
      <h3>{homebrew.name}</h3>
      <b>Classes:</b>
      <ul>
        {#each homebrew.classes as cl}
          <li>{cl.name}</li>
        {/each}
      </ul>
      <b>Races:</b>
      <ul>
        {#each homebrew.races as race}
          <li>{race.name}</li>
        {/each}
      </ul>
      <b>Skills:</b>
      <ul>
        {#each homebrew.skills as skill}
          <li>{skill.name}</li>
        {/each}
      </ul>
      <b>Backgrounds:</b>
      <ul>
        {#each homebrew.backgrounds as background}
          <li>{background.name}</li>
        {/each}
      </ul>
    </div>
  {/each}
  </div>
  <h2>Character sheets</h2>
  <ul>
    {#each sheets as sheet}
      <li>{sheet.character_name} ({sheet.classes[0].name}, {sheet.total_level}) <button on:click={() => {
        invoke('change_sheet', {name: sheet.character_name})
      }}>Load</button></li>
    {/each}
  </ul>
</div>

<style>
  .homebrew-holder {
    display: flex;
  }

  .homebrew-content {
    background-color: rgb(212, 212, 212);
    border: 2px solid black;
    border-radius: 1rem;
    width: fit-content;
    padding: 1rem;
    margin: 1rem;
  }
</style>