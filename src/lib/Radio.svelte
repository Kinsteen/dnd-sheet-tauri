<script>
export let checked = false;
export let modifier = "";
export let text = "";
</script>

<div class="radio">
  <div class="radio-display">
    <div class="radio-outer"></div>
    {#if checked}
      <div class="radio-inner"></div>
    {/if}
  </div>
  <div>{modifier}</div>
  <div>{text}</div>
</div>

<style>
  .radio {
    display: flex;
    flex-direction: row;
    gap: .25rem;
    margin-top: .25rem;
  }

  .radio-display {
    position: relative;
    margin-top: .125rem;
    width: 1.5rem;
    height: 1.25rem;
  }

  .radio-inner {
    position: absolute;
    top: .25rem;
    left: .25rem;
    height: .75rem;
    width: .75rem;
    background-color: #000;
    border-radius: 50%;
    display: inline-block;
  }

  .radio-outer {
    position: absolute;
    height: 1.25rem;
    width: 1.25rem;
    border: 1px solid black;
    border-radius: 50%;
    display: inline-block;
  }
</style>