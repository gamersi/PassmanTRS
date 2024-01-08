import { addMessages, locale, init, getLocaleFromNavigator} from "svelte-i18n";
import { get } from "svelte/store";

import en from "./en.json";
import de from "./de.json";

export const availableLocales = [
    "en",
    "de"
]

export function i18nInit() {
    addMessages("en", en);
    addMessages("de", de);

    let navLocale = getLocaleFromNavigator();
    if (navLocale) {
        navLocale = navLocale.split("-")[0];
    } else {
        navLocale = "en";
    }

    init({
        fallbackLocale: "en",
        initialLocale: navLocale,
    });
}

export function setLocale(newLocale: string) {
    if (availableLocales.includes(newLocale)) {
        locale.set(newLocale);
    }
}

export function getLocale(): string {
    return get(locale) || "en";
}