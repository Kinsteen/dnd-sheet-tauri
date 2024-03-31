<script>
const SLOTS = $$props.$$slots
export let title = ""
export let bottomText = ""
</script>

<div class="card-standard-border">
  {#if title.length > 0}
    <div class="card-standard-title">{title}</div>
  {:else if SLOTS.title}
    <div class="card-standard-title">
      <slot name="title"></slot>
    </div>
  {/if}

  <slot></slot>

  {#if bottomText.length > 0}
    <div class="card-standard-bottom-text">{bottomText}</div>
  {:else if SLOTS.bottomText}
    <div class="card-standard-bottom-text">
      <slot name="bottomText"></slot>
    </div>
  {/if}
</div>

<style>
  .card-standard-border {
    position: relative;
    border: .125rem solid black;
    padding: 1rem;
    margin-top: 1rem;
    margin-bottom: .25rem;
  }

  .card-standard-border:has(.card-standard-title) {
    padding-top: 1.5rem;
  }

  .card-standard-title {
    position: absolute;
    top: calc(-1rem - .125rem);
    left: .75rem;
    background-color: var(--main-background-color);
    text-transform: uppercase;
    font-weight: bold;
    border: .125rem solid black;
    padding: .25rem .75rem;
  }

  .card-standard-bottom-text {
    background-color: var(--main-background-color);
    font-size: small;
    position: absolute;
    bottom: calc(-.50rem);
    right: .75rem;
    padding: 0 .50rem;
  }
</style>
