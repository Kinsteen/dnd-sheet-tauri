<script>
  import Card from './lib/Card.svelte'
  import Radio from './lib/Radio.svelte';

  import { invoke } from "@tauri-apps/api/tauri"

  let abilitiesData = []
  let skillsData = []

  async function loadAbilities() {
    abilitiesData = await invoke("get_abilities_data")
  }

  async function loadSkills() {
    skillsData = await invoke("get_skills_data")
  }

  loadAbilities()
  loadSkills()
</script>

<main class="container">
  <div class="abilities-grid">
    {#each abilitiesData as ability }
      <Card>
        <div slot="title">{ability.name.substring(0,3)}</div>
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
        {#each skillsData as skill }
          <Radio checked={skill.proficient} modifier={skill.modifier} text={skill.name}></Radio>
        {/each}
      </Card>
      <button on:click={loadAbilities}>Load abilities</button>
    </div>
    <div class="main-column">
      <Card title="Saving throws">
        {#each abilitiesData as ability }
          <Radio checked={ability.saving_throw} modifier={ability.saving_throw_modifier} text={ability.name}></Radio>
        {/each}
      </Card>
      <Card title="Health">
        <div class="health-main">
          <button>-</button>
          <div>15/15</div>
          <button>+</button>
        </div>
      </Card>
      <Card>aaa</Card>
    </div>
  </div>
  <div class="footer">

  </div>
</main>

<style>
  .abilities-grid {
    /* TODO maybe? */
    /* position: sticky;
    top: 0; */
    background-color: #fff;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 1rem;
    padding: 1rem;
    width: 100%;
  }

  @media (min-width: 900px) {
    .abilities-grid {
      display: grid;
      grid-template-columns: 1fr;
      gap: 1rem;
      padding: 1rem;
      width: 12rem;
    }
  }

  .ability {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: .5rem;
  }

  .ability > .modifier {
    font-size: xx-large;
    line-height: 2rem;
  }

  .ability > .total {
    font-size: medium;
    line-height: 1rem;
  }

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
    padding: .75rem;
    padding-bottom: 7rem;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .main-column {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  @media (min-width: 900px) {
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
