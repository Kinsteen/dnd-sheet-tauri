<script>
  import { Window } from "@tauri-apps/api/window";
  import { Webview } from "@tauri-apps/api/webview";
  import Card from "$lib/Card.svelte";
  import Radio from "$lib/Radio.svelte";
  import { open } from '@tauri-apps/plugin-shell';
  import { appDataDir } from '@tauri-apps/api/path';
  import { listen } from '@tauri-apps/api/event'
  import { _, locale, locales} from 'svelte-i18n';
    import Icon from 'mdi-svelte';
    import { mdiCog, mdiTranslate } from "@mdi/js";

  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

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

  onMount(() => {
    // TODO test after onMount
    listen('reload-sheet', (event) => {
      console.log("Reloading sheet because backend said so")
      loadSheet()
    }).then(() => {
    }).catch((e) => {
      console.error(e)
    })
  })

</script>

<div style="display: flex; align-items: center; justify-content: space-between;">
  <h1 style="text-transform: uppercase;">DnD Tracker</h1>
  <div style="display: flex; align-items: center; gap: .25rem">
    <button popovertarget="translations-popover"><Icon path={mdiTranslate} color="black" /></button>
    <div popover="auto" id="translations-popover" style="position:absolute; margin-right: .25rem; margin-left: auto; top: 3rem">
      {#each $locales as l}
      <button on:click={() => {
        $locale = l
      }}>{l}</button>
      {/each}
    </div>
    <a href="/settings"><Icon path={mdiCog} color="black" /></a>
  </div>
</div>

<main class="container">
  <div class="header">
    <Card title={$_("header_title")}>
      <div style="display: grid; grid-template-columns: 1fr 1fr; column-gap: 10%;">
        <div style="display:flex; flex-direction: column; justify-content: center;align-items: center; border-right: 2px solid black;">
          <div style="display:flex; flex-direction: column">
            <span style="font-weight: bold; font-size: x-large;">{characterName}</span>
            <span style="font-style: italic; font-size: smaller;">{$_("character_name")}</span>
          </div>
        </div>
        <div style="display: grid;">
          <div style="grid-column: 2; grid-row: 1 / span 2; display: flex; flex-direction: column; justify-content: center; align-items: center;">
            <span style="font-weight: bold; font-size: xx-large;">{totalLevel}</span>
            <span style="font-style: italic; font-size: smaller;">{$_("level")}</span>
          </div>
          <div style="grid-column: 1; grid-row: 1; display: flex; flex-direction: column">
            {#each classes as cl}
              <span style="font-weight: bold;">{$_(`classes.${cl.name}`)} {cl.level}</span>
            {/each}
            <span style="font-style: italic; font-size: smaller;">{$_("classes_name")}</span>
          </div>
          <div style="grid-column: 1; grid-row: 2; display: flex; flex-direction: column">
            <span style="font-weight: bold;">{$_(`races.${race}`)}</span>
            <span style="font-style: italic; font-size: smaller;">{$_("race")}</span>
          </div>
        </div>
      </div>
    </Card>
  </div>

  <div class="abilities-grid">
    {#each abilitiesData as ability}
      <Card>
        <div slot="title">{$_(`abilities.${ability.name}`).substring(0, 3)}</div>
        <div class="ability">
          <span class="modifier">{ability.modifier}</span>
          <span class="total">{ability.total}</span>
        </div>
        <div slot="bottomText"><i>{$_(`abilities.${ability.name}`).toLowerCase()}</i></div>
      </Card>
    {/each}
  </div>
  <div class="main-info">
    <div class="main-column">
      <div class="skills">
        <Card title={$_("skills_name")}>
          {#each skillsData.sort((a, b) => $_(`skills.${a.name}`).localeCompare($_(`skills.${b.name}`))) as skill}
            <Radio
              checked={skill.proficient}
              modifier={skill.modifier}
              text={$_(`skills.${skill.name}`)}
            ></Radio>
          {/each}
        </Card>
      </div>
      <!-- <button
        on:click={() => {
          const window = new Window("label");
          const w = new Webview(window, "theUniqueLabel", {
            url: "/create-character",
            x: 0,
            y: 0,
            width: 500,
            height: 500,
          });
        }}>{$_("create_char")}</button
      > -->
      <a href="/create-character">{$_("create_char")}</a>
      <button on:click={() => {
        appDataDir().then(path => {
          console.log(path)
          open(path)
        })
      }}>{$_("open_data_dir")}</button>
    </div>
    <div class="main-column">
      <Card title={$_('saving_throws')}>
        {#each abilitiesData as ability}
          <Radio
            checked={ability.saving_throw}
            modifier={ability.saving_throw_modifier}
            text={$_(`abilities.${ability.name}`)}
          ></Radio>
        {/each}
      </Card>
      <Card title={$_("health")}>
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
        <Card title={$_(`counters.${counter.name}`)}>
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
  <div class="footer">
    <button>Main info</button>
    <button>Spells</button>
    <button>Inventory</button>
  </div>
</main>

<style>
  .abilities-grid {
    background-color: #fff;
    z-index: 1;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    width: 100%;
    gap: 1rem;
    padding: 1rem;
  }

  .container {
    display: grid;
    grid-template-columns: 1fr;
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
    line-height: 1;
  }

  .ability > .total {
    font-size: medium;
  }

  .footer {
    position: sticky;
    bottom: 0;
    width: 100%;
    height: 4rem;
    background-color: #ccc;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    padding: 1rem;
    z-index: 1;
  }

  .main-info {
    flex-grow: 1;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
    margin: 1rem;
    /* max-width: 54rem; */
  }

  .main-column {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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

  .header {
    margin: 0rem 1rem;
  }

  @media (width > 800px) {
    .container {
      grid-template-columns: 10rem 1fr;
    }

    .header {
      grid-column: 1 / span 2;
    }

    .footer {
      grid-column: 1 / span 2;
    }

    .main-info {
      max-width: 60rem;
    }

    .abilities-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
