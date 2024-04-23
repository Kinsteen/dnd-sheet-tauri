import "$lib/i18n"; // Import to initialize. Important :)
import { locale, waitLocale } from "svelte-i18n";

export const prerender = true;
export const ssr = false;

export const load = async () => {
  locale.set(window.navigator.language);
  await waitLocale();
};
