<script>
  import { invoke } from "@tauri-apps/api/core"

  let classes = []
  let races = []
  let abilities = [{name: "strength"}, {name: "dexterity"}]
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

  async function loadSkills() {
    // TODO send class and race to calculate what's available
    skills = await invoke("get_available_skills")
    console.log(skills)
  }

  loadClasses()
  loadRaces()
  // loadAbilities()

  // TODO standard array/point buy helper
</script>

<div>
  <div>
    <label for="character-name-input">Character name:</label>
    <input type="text" id="character-name-input" />
  </div>
  <div>
    <label for="class">Pick a class:</label>
    <select name="class" id="class">
      <option value="undefined">Choose a class...</option>
      {#each classes as cl}
        <option value={cl.name}>{cl.name}</option>
      {/each}
    </select>
  </div>

  <div>
    <label for="race">Pick a race:</label>
    <select name="race" id="race">
      <option value="undefined">Choose a race...</option>
      {#each races as r}
        <option value={r.name}>{r.name}</option>
      {/each}
    </select>
  </div>
  <div>
    Health system:
    <div>
      <input id="health-mean" type="radio" />
      <label for="health-mean">Mean hit dice</label>
    </div>
    <div>
      <input id="health-rolls" type="radio" />
      <label for="health-rolls">Roll every level</label>
    </div>
  </div>
  <div>
    Abilities:
    {#each abilities as ability}
      <div>
        {ability.name}
        <input type="number" min=0 max=20 value=10/>
      </div>
    {/each}
  </div>
  <div>
    Skills:
    {#if skills.length == 0}
      <p>Pick a class and race, and skills you can pick will show up here.</p>
    {/if}
    {#each skills as skill}
      <div>
        Skill
      </div>
    {/each}
  </div>
  <button>Create & load character</button>
</div>
