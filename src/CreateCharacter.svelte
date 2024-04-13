<script>
  import { invoke } from "@tauri-apps/api/core"
  import { writable } from 'svelte/store';

  let classes = []
  let races = []
  let abilities = [{name: "strength"}, {name: "dexterity"}]
  let skillsClass = []
  let skillsRace = []
  let numPickClass = 0
  let numPickRace = 0
  let skills = []

  async function loadClasses() {
    classes = await invoke("get_available_classes")
    console.log(classes)
  }
  
  async function loadRaces() {
    races = await invoke("get_available_races")
    console.log(races)
  }

  async function loadAbilities() {
    abilities = await invoke("get_available_abilities")
    console.log(abilities)
  }

  function loadSkills(className, raceName) {
    invoke("get_available_skills", {
      className: className,
      raceName: raceName
    }).then(r => {
      skills = []
      skillsClass = r[0].skills
      skillsClass.forEach(skill => {
        skills.push({name: skill, from: "class"})
      });
      skillsRace = r[1].skills
      skillsRace.forEach(skill => {
        skills.push({name: skill, from: "race"})
      });
      numPickClass = r[0].num_to_pick
      numPickRace = r[1].num_to_pick
    })
  }

  loadClasses()
  loadRaces()
  // loadAbilities()

  let characterName = ''
  let className = 'undefined'
  let raceName = 'undefined'
  let healthMean = 'mean'
  let abilitiesValues = {}
  let skillsChecked = []

  const errorMessage = writable('')

  // TODO standard array/point buy helper
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
        <input bind:value={abilitiesValues[ability.name]} type="number" min=0 max=20 />
      </div>
    {/each}
  </div>
  <div>
    Skills:
    {#if skillsRace.length == 0 && skillsClass.length == 0}
      <p>Pick a class and race, and skills you can pick will show up here.</p>
    {/if}
    {#each skills as skill, i}
      <div>
        <input bind:value={skillsChecked}
        type="checkbox"
        disabled={false}/>
        <span class="skill-{skill.from}">{skill.name}</span> (from {skill.from})
      </div>
    {/each}
  </div>
  {#if $errorMessage.length > 0}
    <div>
      {$errorMessage}
    </div>
  {/if}
  <button on:click={() => {
    console.log(skillsChecked)
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
  span.skill-class {
    color: red;
  }

  span.skill-race {
    color: blue;
  }
</style>