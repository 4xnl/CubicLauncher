import { launcherStore } from "$lib/state/state.svelte";
import es from "./es.json";
import en from "./en.json";

type DictValue = string | { [key: string]: DictValue };
const dicts: Record<string, DictValue> = { es, en };

const cache = new Map<string, string>();
let lastLang = "";

export function t(
	key: string,
	params?: Record<string, string | number>,
): string {
	const lang = launcherStore.settings?.language || "es";
	if (lang !== lastLang) {
		cache.clear();
		lastLang = lang;
	}
	if (!params) {
		const cached = cache.get(key);
		if (cached !== undefined) return cached;
	}
	const dict = dicts[lang] || dicts["es"];
	if (!dict || typeof dict === "string") return key;
	let value: DictValue = dict;
	for (const k of key.split(".")) {
		if (typeof value === "string") return key;
		value = value[k];
		if (value === undefined) return key;
	}
	const result = typeof value === "string" ? value : key;
	if (!params) {
		cache.set(key, result);
		return result;
	}
	return result.replace(/\{(\w+)\}/g, (_, name) =>
		String(params[name] ?? `{${name}}`),
	);
}
