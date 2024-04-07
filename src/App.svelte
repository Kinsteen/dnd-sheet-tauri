<script>
import { Window } from '@tauri-apps/api/window';
import { Webview } from '@tauri-apps/api/webview';
  import Card from './lib/Card.svelte'
  import Radio from './lib/Radio.svelte';

  import { invoke } from "@tauri-apps/api/core"
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { window } from '@tauri-apps/api';

  let abilitiesData = []
  let skillsData = []
  let counters = []
  let health = undefined;

  async function loadAbilities() {
    abilitiesData = await invoke("get_abilities_data")
    console.log(abilitiesData)
  }

  async function loadSkills() {
    skillsData = await invoke("get_skills_data")
    console.log(skillsData)
  }

  async function loadCounters() {
    counters = await invoke("get_counters")
    console.log(counters)
  }

  async function loadHealth() {
    health = await invoke("get_health")
    console.log(health)
  }

  loadAbilities()
  loadSkills()
  loadCounters()
  loadHealth()
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
      <button on:click={() => {
        const window = new Window('label')
        window.once('tauri://created', () => {
          console.log("created")
        })
        window.once('tauri://error', e => {
          console.log(e)
        })
        const w = new Webview(window, 'theUniqueLabel', {
          url: '/create-character.html',
          x: 0,
          y: 0,
          width: 500,
          height:500
        });

        w.once('tauri://created', function () {
          console.log("wv created")
        });
        w.once('tauri://error', function (e) {
                  console.log(e)
        });

        w.position()

        // webview.window.show().then(() => {
        //   console.log("view showed")
        // }).catch((e) => {
        //   console.error(e)
        // })
      }}>Create character page</button>
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
          {#if health}
          <div>{health.current}/{health.max}</div>
          {/if}
          <button>+</button>
        </div>
      </Card>
      {#each counters as counter }
        <Card title={counter.name.replaceAll("_", " ")}>
          {counter.used}/{counter.max_uses}
        </Card>
      {/each}
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

  .container {
    display: flex;
    flex-direction: column;
    grid-template-columns: 1fr;
  }

  .ability {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;
    gap: .25rem;
    height: 100%;
  }

  .ability > .modifier {
    font-size: xx-large;
    line-height: 2rem;
  }

  .ability > .total {
    font-size: medium;
    line-height: 1rem;
  }

  @media (min-width: 650px) {
    .abilities-grid {
      grid-template-columns: 1fr;
      width: 10rem;
      height: 100vh;
    }

    .container {
      /* display: flex; grid doesn't work? */
      grid-template-columns: 2fr;
      flex-direction: row;
    }

    .main-info {
      width: fill-available;
      width: -webkit-fill-available;
    }
  }

  @media (max-height: 800px) {
      .ability {
        flex-direction: row;
        height: 100%;
      }

      .ability > .modifier {
        font-size: x-large;
        line-height: 0;
      }

      .ability > .total {
        font-size: small;
      }
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

  @media (min-width: 650px) {
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
