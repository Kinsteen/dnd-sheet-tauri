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

  let tempChecked = {
    class: {},
    race: {},
    background: {},
  }

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
    <table>
      <tr>
        <th>Ability name</th>
        <th>
          Base value
        </th>
        <th>
          Calculated value
        </th>
      </tr>
    {#each abilities as ability}
      <tr>
        <td style="text-align: end;">{$_(`abilities.${ability.name}`)}</td>
        <td>
          <input bind:value={abilitiesValues[ability.name]} type="number" min=0 max=20
          on:input={(e) => {
            invoke("calculate_ability", {name: ability.name, baseValue: abilitiesValues[ability.name], className: className, raceName: raceName})
            .then(r => {
              console.log(r)
              calculatedAbilitiesValues[ability.name] = r
            })
            .catch(e => {
              console.error(e)
            })
          }}/>
        </td>
        <td>
          10
        </td>
      </tr>
    {/each}
  </table>
  </div>
  <div>
    Skills from your class: (pick {numPickClass})
    {#each skillsClass as skill}
      <div>
        <input type="checkbox" bind:checked={tempChecked["class"][skill]} />
        <span class="skill-class">{$_(`skills.${skill}`)}</span>
      </div>
    {/each}
    Skills from your race: (pick {numPickRace})
    {#each skillsRace as skill}
      <div>
        <input type="checkbox" bind:checked={tempChecked["race"][skill]} />
        <span class="skill-race">{$_(`skills.${skill}`)}</span>
      </div>
    {/each}
    Skills from your background: (pick {numPickBackground})
    {#each skillsBackground as skill}
      <div>
        <input type="checkbox" bind:checked={tempChecked["background"][skill]} />
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
    let checked = {}

    let mapChecked = (key) => {
      Object.keys(tempChecked[key]).forEach(skill => {
        let val = tempChecked[key][skill]
  
        if (!checked[key]) {
          checked[key] = []
        }

        if (val) {
          checked[key].push(skill)
        }
      })
    }

    mapChecked('class')
    mapChecked('race')
    mapChecked('background')
    // console.log(Object.fromEntries(Object.entries(skillsChecked).filter(([k,v]) => console.log(v))))
    invoke("create_sheet", {
      characterName: characterName,
      class: className,
      race: raceName,
      healthSystemMean: healthMean == "mean",
      abilities: abilitiesValues,
      skills: checked,
    }).then(() => {
      errorMessage.set('')
    }).catch(e => {
      errorMessage.set(e)
    })
  }}>Create & load character</button>
</div>

<style>
  .header {
    display: flex;
    gap: .5rem;
    margin: .75rem;

    & > button {
      border: 0;
      cursor: pointer;
      background-color: transparent;
    }
  }

  .text-input {
    border: 2px solid black;

    &:focus-visible {
      border-radius: 0;
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
