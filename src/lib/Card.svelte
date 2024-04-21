<script>
  const SLOTS = $$props.$$slots
  export let title = ""
  export let bottomText = ""

  let titleHeight
  let card

  $: {
    if (card && card.style && titleHeight > 0) {
      // 1rem is the top margin
      // .125rem is border size
      // (titleHeight - 32)px is the "overflow" of the title
      // .5rem is the "normal" padding
      card.style.paddingTop = "calc(1rem + .125rem + " + (titleHeight - 32) + "px + .5rem)"
    }
  }
</script>

<div bind:this={card} class="card-standard-border">
  {#if title.length > 0}
    <div bind:clientHeight={titleHeight} class="card-standard-title">{title}</div>
  {:else if SLOTS.title}
    <div bind:clientHeight={titleHeight} class="card-standard-title">
      <slot name="title"></slot>
    </div>
  {/if}

  {#if SLOTS}
  <slot></slot>
  {/if}

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
    padding: .5rem;
    margin-top: 1rem;
    margin-bottom: .25rem;
  }

  .card-standard-title {
    position: absolute;
    top: calc(-1rem - .125rem);
    left: .75rem;
    max-width: 90%;
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
