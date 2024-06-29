<script>
  import { invoke } from "@tauri-apps/api/core"
  import { writable } from 'svelte/store';
  import { _ } from 'svelte-i18n';
  import Icon from 'mdi-svelte';
  import { mdiArrowLeft } from '@mdi/js';

  let homebrews = {}
  let homebrewsChecked = []

  let classes = []
  let races = []
  let backgrounds = []
  let abilities = [
    {name: "strength"},
    {name: "dexterity"},
    {name: "constitution"},
    {name: "intelligence"},
    {name: "wisdom"},
    {name: "charisma"},
  ]
  let skillsClass = []
  let skillsRace = []
  let skillsBackground = []
  let numPickClass = 0
  let numPickRace = 0
  let numPickBackground = 0
  let skills = []

  let pickedSkillsClass = []
  let pickedSkillsRace = []
  let pickedSkillsBackground = []

  let characterName = ''
  let className = 'undefined'
  let raceName = 'undefined'
  let backgroundName = 'undefined'
  let healthMean = 'mean'
  let abilitiesValues = {}
  let calculatedAbilitiesValues = {}

  async function loadClasses() {
    classes = await invoke("get_available_classes", {homebrews: homebrewsChecked})
  }
  
  async function loadRaces() {
    races = await invoke("get_available_races", {homebrews: homebrewsChecked})
  }

  async function loadBackgrounds() {
    backgrounds = await invoke("get_available_backgrounds", {homebrews: homebrewsChecked})
  }

  function loadSkills() {
    invoke("get_available_skills", {
      className: className,
      raceName: raceName,
      backgroundName: backgroundName,
      homebrews: homebrewsChecked
    }).then(r => {
      skillsClass = r[0].skills
      skillsRace = r[1].skills
      skillsBackground = r[2].skills
      numPickClass = r[0].num_to_pick
      numPickRace = r[1].num_to_pick
      numPickBackground = r[2].num_to_pick
    })
  }

  async function loadHomebrews() {
    homebrews = await invoke('get_homebrews')
    console.log(homebrews)
  }

  loadHomebrews()
  loadClasses()
  loadRaces()
  loadBackgrounds()
  loadSkills()

  const errorMessage = writable('')

  // TODO standard array/point buy helper
  // TODO clear checkboxes and arrays when changing race/class
</script>

<div>
  <div class="header">
    <button on:click={() => {
      window.history.back();
    }}><Icon path={mdiArrowLeft} size={1.3} /></button>
    <h1>{$_("create.title")}</h1>
  </div>
  <div style="display: flex; flex-direction: column;">
    Enable homebrews:
    {#each Object.keys(homebrews) as key}
      <div>
        <input type="checkbox" value={key} bind:group={homebrewsChecked} on:change={() => {
          loadHomebrews()
          loadClasses()
          loadRaces()
          loadBackgrounds()
          loadSkills()
        }}> {key}
      </div>
    {/each}
  </div>
  <div>
    <input class="text-input" bind:value={characterName} type="text" placeholder="Character name"/>
  </div>
  <div>
    <label for="class">Pick a class:</label>
    <select bind:value={className} on:change={e => {
      loadSkills()
    }} name="class" id="class">
      <option value="undefined">Choose a class...</option>
      {#each classes as cl}
        <option value={cl.name}>{cl.name}</option>
      {/each}
    </select>
  </div>

  <div>
    <label for="race">Pick a race:</label>
    <select bind:value={raceName} on:change={e => {
      loadSkills()
    }} name="race" id="race">
      <option value="undefined">Choose a race...</option>
      {#each races as r}
        <option value={r.name}>{r.name}</option>
      {/each}
    </select>
  </div>
  <div>
    <label for="race">Pick a background:</label>
    <select bind:value={backgroundName} on:change={e => {
      loadSkills()
    }} name="background" id="background">
      <option value="undefined">Choose a background...</option>
      {#each backgrounds as b}
        <option value={b.name}>{b.name}</option>
      {/each}
    </select>
  </div>
  <div>
    Health system:
    <div>
      <input bind:group={healthMean} id="health-mean" type="radio" name="health-system" value="mean" checked />
      <label for="health-mean">Mean hit dice</label>
    </div>
    <div>
      <input bind:group={healthMean} id="health-rolls" type="radio" name="health-system" value="rolls"/>
      <label for="health-rolls">Roll every level</label>
    </div>
  </div>
  <div>
    <table class="abilities-table">
      <tr>
        <th class="ability-name">Ability name</th>
        <th class="base-value">Base value</th>
        <th class="calculated-value">Calculated value</th>
      </tr>
    {#each abilities as ability}
      <tr>
        <td style="text-align: end;">{$_(`abilities.${ability.name}`)}</td>
        <td>
          <input class="text-input" bind:value={abilitiesValues[ability.name]} type="number" min=0 max=20
          on:input={(e) => {
            invoke("calculate_ability", {name: ability.name, baseValue: abilitiesValues[ability.name], className: className, raceName: raceName})
            .then(r => {
              calculatedAbilitiesValues[ability.name] = r
            })
            .catch(e => {
              console.error(e)
            })
          }}/>
        </td>
        <td>
          {#if calculatedAbilitiesValues[ability.name]}
            {calculatedAbilitiesValues[ability.name].total}
          {:else}
            Not calculated yet
          {/if}
        </td>
      </tr>
    {/each}
  </table>
  </div>
  <div>
    Skills from your class: (pick {numPickClass})
    {#each skillsClass as skill}
      <div>
        <input type="checkbox" bind:group={pickedSkillsClass} value={skill} />
        <span class="skill-class">{$_(`skills.${skill}`)}</span>
      </div>
    {/each}
    Skills from your race: (pick {numPickRace})
    {#each skillsRace as skill}
      <div>
        <input type="checkbox" bind:group={pickedSkillsRace} value={skill} />
        <span class="skill-race">{$_(`skills.${skill}`)}</span>
      </div>
    {/each}
    Skills from your background: (pick {numPickBackground})
    {#each skillsBackground as skill}
      <div>
        <input type="checkbox" bind:group={pickedSkillsBackground} value={skill} />
        <span class="skill-background">{$_(`skills.${skill}`)}</span>
      </div>
    {/each}
  </div>
  {#if $errorMessage.length > 0}
    <div style="color: red">
      {$errorMessage}
    </div>
  {/if}
  <button on:click={() => {
    let checked = {
      class: pickedSkillsClass,
      race: pickedSkillsRace,
      background: pickedSkillsBackground
    }

    // console.log(Object.fromEntries(Object.entries(skillsChecked).filter(([k,v]) => console.log(v))))
    invoke("create_sheet", {
      characterName: characterName,
      class: className,
      race: raceName,
      healthSystemMean: healthMean == "mean",
      abilities: abilitiesValues,
      skills: checked,
      homebrews: homebrewsChecked,
    }).then(() => {
      errorMessage.set('')
    }).catch(e => {
      errorMessage.set(e)
    })
  }}>Create & load character</button>
</div>

<style>
  .text-input {
    border: 2px solid black;

    &:focus-visible {
      border-radius: 0;
    }
  }

  .abilities-table {
    width: min(100%, 30rem);
    border: 2px solid black;
    border-collapse: collapse;
    /* TODO center */

    & td {
      padding: .25rem;
    }
    
    & tr > th {
      border: 2px solid black;
    }

    & .ability-name {
      width: 10rem;
    }

    & .base-value {
      width: 0; /* Min size possible, so it's the input that tells */
    }

    & input {
      margin: auto;
      width: 3em;
    }

    & td:has(input) {
      display:flex;
      align-items: center;
    }
  }

  span.skill-class {
    color: red;
    font-weight: bold;
  }

  span.skill-race {
    color: blue;
    font-weight: bold;
  }
</style>
