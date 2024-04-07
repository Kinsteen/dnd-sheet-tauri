import "./styles.css";
import CreateCharacter from "./CreateCharacter.svelte";

const app = new CreateCharacter({
  target: document.getElementById("app"),
});

export default app;
