<script>
  import { Window } from "@tauri-apps/api/window";
  import { Webview } from "@tauri-apps/api/webview";
  import Card from "$lib/Card.svelte";
  import Radio from "$lib/Radio.svelte";
  import { open } from '@tauri-apps/plugin-shell';
  import { appDataDir } from '@tauri-apps/api/path';
  import { listen } from '@tauri-apps/api/event'

  import { invoke } from "@tauri-apps/api/core";

  let abilitiesData = [];
  let skillsData = [];
  let counters = [];
  let health = undefined;

  let characterName = "Emily Solis"
  let classes = [{name: "Light Cleric", level: 3}]
  let race = "Godwalker"
  let totalLevel = 5

  async function loadInfos() {
    let data = await invoke("get_basic_data");
    characterName = data.character_name
    classes = data.classes
    race = data.race
    totalLevel = data.total_level
  }

  async function loadAbilities() {
    abilitiesData = await invoke("get_abilities_data");
  }

  async function loadSkills() {
    skillsData = await invoke("get_skills_data");
  }

  async function loadCounters() {
    counters = await invoke("get_counters");
  }

  async function loadHealth() {
    health = await invoke("get_health");
  }

  async function loadSheet() {
    loadInfos()
    loadAbilities()
    loadSkills()
    loadCounters()
    loadHealth()
  }

  loadSheet()

  listen('reload-sheet', (event) => {
    console.log("Reloading sheet because backend said so")
    loadSheet()
  }).then(() => {
  }).catch((e) => {
    console.error(e)
  })
</script>

<div>
  <a href="/settings">Settings</a>
</div>

<main class="container">
  <div style="margin: 0rem 1rem; flex-basis: 100%">
    <Card title="Basic Infos">
      <div style="display: grid; grid-template-columns: 1fr 1fr; column-gap: 10%;">
        <div style="display:flex; flex-direction: column; justify-content: center;align-items: center; border-right: 2px solid black;">
          <div style="display:flex; flex-direction: column">
            <span style="font-weight: bold; font-size: x-large;">{characterName}</span>
            <span style="font-style: italic; font-size: smaller;">Character name</span>
          </div>
        </div>
        <div style="display: grid;">
          <div style="grid-column: 2; grid-row: 1 / span 2; display: flex; flex-direction: column; justify-content: center; align-items: center;">
            <span style="font-weight: bold; font-size: xx-large;">{totalLevel}</span>
            <span style="font-style: italic; font-size: smaller;">Level</span>
          </div>
          <div style="grid-column: 1; grid-row: 1; display: flex; flex-direction: column">
            {#each classes as cl}
              <span style="font-weight: bold; text-transform: capitalize;">{cl.name.replaceAll("_", " ")} {cl.level}</span>
            {/each}
            <span style="font-style: italic; font-size: smaller;">Classes</span>
          </div>
          <div style="grid-column: 1; grid-row: 2; display: flex; flex-direction: column">
            <span style="font-weight: bold; text-transform: capitalize;">{race}</span>
            <span style="font-style: italic; font-size: smaller;">Race</span>
          </div>
        </div>
      </div>
    </Card>
  </div>

  <div class="abilities-grid">
    {#each abilitiesData as ability}
      <Card>
        <div slot="title">{ability.name.substring(0, 3)}</div>
        <div class="ability">
          <span class="modifier">{ability.modifier}</span>
          <span class="total">{ability.total}</span>
        </div>
        <div slot="bottomText"><i>{ability.name}</i></div>
      </Card>
    {/each}
  </div>
  <div class="main-info">
    <div class="main-column">
      <Card title="Skills">
        {#each skillsData as skill}
          <Radio
            checked={skill.proficient}
            modifier={skill.modifier}
            text={skill.name}
          ></Radio>
        {/each}
      </Card>
      <button
        on:click={() => {
          const window = new Window("label");
          const w = new Webview(window, "theUniqueLabel", {
            url: "/create-character",
            x: 0,
            y: 0,
            width: 500,
            height: 500,
          });
        }}>Create character page</button
      >
      <button on:click={() => {
        appDataDir().then(path => {
          console.log(path)
          open(path)
        })
      }}>Open dir</button>
    </div>
    <div class="main-column">
      <Card title="Saving throws">
        {#each abilitiesData as ability}
          <Radio
            checked={ability.saving_throw}
            modifier={ability.saving_throw_modifier}
            text={ability.name}
          ></Radio>
        {/each}
      </Card>
      <Card title="Health">
        <div class="health-main">
          <button on:click={() => {
            invoke('change_health', {value: -1}).then(() => {
              loadHealth()
            })
          }}>-</button>
          {#if health}
            <div>{health.current}/{health.max}</div>
          {/if}
          <button on:click={() => {
            invoke('change_health', {value: 1}).then(() => {
              loadHealth()
            })
          }}>+</button>
        </div>
      </Card>
      {#each counters as counter}
        <Card title={counter.name.replaceAll("_", " ")}>
          <div class="health-main">
          <button on:click={() => {
            invoke('change_counter', {name: counter.name, value: -1}).then(() => {
              loadCounters()
            })
          }}>-</button>
          {counter.used}/{counter.max_uses}
          <button on:click={() => {
            invoke('change_counter', {name: counter.name, value: 1}).then(() => {
              loadCounters()
            })
          }}>+</button>
          </div>
        </Card>
      {/each}
    </div>
  </div>
  <div class="footer"></div>
</main>

<style>
  .abilities-grid {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    width: 100%;
    gap: 1rem;
    margin: 1rem;
  }

  .container {
    display: flex;
    flex-wrap: wrap;
    height: 100%;
  }

  .ability {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    align-items: center;
  }

  .ability > .modifier {
    font-size: xx-large;
  }

  .ability > .total {
    font-size: medium;
  }

  @media (width >= 650px) {
    .abilities-grid {
      align-self: stretch;
      grid-template-columns: 1fr;
      width: 8rem;
    }

    .main-info {
      /* width: fill-available;
      width: -webkit-fill-available; */
    }
  }

  /* @container (height <= 600px) {
    .ability {
      flex-direction: row;
    }

    .ability > .modifier {
      font-size: x-large;
      line-height: 0;
    }

    .ability > .total {
      font-size: small;
    }
  } */

  .footer {
    display: block;
    position: fixed;
    left: 0;
    bottom: 0;
    width: 100%;
    height: 6rem;
    background-color: #ccc;
  }

  .main-info {
    flex-grow: 1;
    padding-bottom: 7rem;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    margin: 1rem;
  }

  .main-column {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  @media (width >= 650px) {
    .footer {
      display: none;
    }

    .main-info {
      padding-bottom: 0;
    }
  }

  .health-main {
    display: flex;
    width: 100%;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  .health-main > button {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 1.75rem;
    width: 1.75rem;

    border: 2px solid black;
    background-color: white;
    cursor: pointer;
  }

  .health-main > div {
    font-size: x-large;
  }
</style>
