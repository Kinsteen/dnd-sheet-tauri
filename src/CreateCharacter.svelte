<script>
  import { invoke } from "@tauri-apps/api/core"
  import { writable } from 'svelte/store';

  let classes = []
  let races = []
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
  let numPickClass = 0
  let numPickRace = 0
  let skills = []
  let allSkills = []

  let tempChecked = {}
  let floatingSkills = {} // key is concat, sorted

  async function loadClasses() {
    classes = await invoke("get_available_classes")
  }
  
  async function loadRaces() {
    races = await invoke("get_available_races")
  }

  function loadSkills(className, raceName) {
    invoke("get_available_skills", {
      className: className,
      raceName: raceName
    }).then(r => {
      skills = []
      r[2].skills.forEach(skillName => {
        skills.push({name: skillName, from: []})
      });
      skillsClass = r[0].skills
      skillsClass.forEach(skill => {
        let idx = skills.findIndex(s => s.name == skill)
        skills[idx].from.push("class")
      });
      skillsRace = r[1].skills
      skillsRace.forEach(skill => {
        let idx = skills.findIndex(s => s.name == skill)
        skills[idx].from.push("race")
      });
      numPickClass = r[0].num_to_pick
      numPickRace = r[1].num_to_pick
    })
  }

  loadClasses()
  loadRaces()
  loadSkills("none", "none")
  // loadAbilities()

  let characterName = ''
  let className = 'undefined'
  let raceName = 'undefined'
  let healthMean = 'mean'
  let abilitiesValues = {}
  let calculatedAbilitiesValues = {}
  let skillsCheckedClass = []
  let skillsCheckedRace = []
  let skillsCheckedBackground = []

  const errorMessage = writable('')

  // TODO standard array/point buy helper
  // TODO clear checkboxes and arrays when changing race/class
</script>

<div>
  <div>
    <label for="character-name-input">Character name:</label>
    <input bind:value={characterName} type="text" id="character-name-input" />
  </div>
  <div>
    <label for="class">Pick a class:</label>
    <select on:change={e => {
      loadSkills(e.target.value, raceName)
    }} bind:value={className} name="class" id="class">
      <option value="undefined">Choose a class...</option>
      {#each classes as cl}
        <option value={cl.name}>{cl.name}</option>
      {/each}
    </select>
  </div>

  <div>
    <label for="race">Pick a race:</label>
    <select on:change={e => {
      loadSkills(className, e.target.value)
      // TODO update ability scores
    }} bind:value={raceName} name="race" id="race">
      <option value="undefined">Choose a race...</option>
      {#each races as r}
        <option value={r.name}>{r.name}</option>
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
    Abilities:
    {#each abilities as ability}
      <div>
        {ability.name}:
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
        {#if calculatedAbilitiesValues[ability.name]}
        {calculatedAbilitiesValues[ability.name].total} ({calculatedAbilitiesValues[ability.name].modifier})
        {/if}
      </div>
    {/each}
  </div>
  <div>
    Skills from your class: (pick {numPickClass})
    {#each skillsClass as skill}
      <div>
        <input type="checkbox" bind:checked={tempChecked[skill]} />
        <span class="skill-class">{skill}</span>
      </div>
    {/each}
    Skills from your race: (pick {numPickRace})
    {#each skillsRace as skill}
    <div>
      <input type="checkbox" bind:checked={tempChecked[skill]} />
      <span class="skill-race">{skill}</span>
    </div>
  {/each}
  </div>
  {#if $errorMessage.length > 0}
    <div>
      {$errorMessage}
    </div>
  {/if}
  <button on:click={() => {
    // console.log(Object.fromEntries(Object.entries(skillsChecked).filter(([k,v]) => console.log(v))))
    // invoke("create_sheet", {
    //   characterName: characterName,
    //   class: className,
    //   race: raceName,
    //   healthSystemMean: healthMean == "mean",
    //   abilities: abilitiesValues,
    //   skills: ["test1", "test2"]
    // }).then(() => {
    //   errorMessage.set('')
    // }).catch(e => {
    //   errorMessage.set(e)
    // })
  }}>Create & load character</button>
</div>

<style>
  span.skill- {
    font-style: italic;
  }
  span.skill-class {
    color: red;
    font-weight: bold;
  }

  span.skill-race {
    color: blue;
    font-weight: bold;
  }
  span.skill-class-race {
    color: purple;
    font-weight: bold;
  }
</style>
