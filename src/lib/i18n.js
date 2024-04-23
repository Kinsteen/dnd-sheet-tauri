import { init, getLocaleFromNavigator, register } from "svelte-i18n";

register("en", () => import("../lang/en.json"));
register("fr", () => import("../lang/fr.json"));

init({
  fallbackLocale: "fr",
  initialLocale: getLocaleFromNavigator(),
});
