export interface LoaderDisplayItem {
	version_id: string;
	display_version: string;
	game_version: string;
	stable: boolean;
}

export function compareVersions(a: string, b: string): number {
	const aParts = a.split(".").map((n) => parseInt(n, 10) || 0);
	const bParts = b.split(".").map((n) => parseInt(n, 10) || 0);
	for (let i = 0; i < Math.max(aParts.length, bParts.length); i++) {
		const difference = (bParts[i] ?? 0) - (aParts[i] ?? 0);
		if (difference) return difference;
	}
	return b.localeCompare(a, undefined, { numeric: true });
}

export function sortLoaderVersions(
	items: LoaderDisplayItem[],
): LoaderDisplayItem[] {
	return items.sort(
		(a, b) =>
			Number(b.stable) - Number(a.stable) ||
			compareVersions(a.display_version, b.display_version),
	);
}

export function groupLoaderVersions(items: LoaderDisplayItem[]) {
	const byGame = new Map<string, LoaderDisplayItem[]>();
	for (const item of items) {
		if (!item.game_version) continue;
		const group = byGame.get(item.game_version);
		if (group) group.push(item);
		else byGame.set(item.game_version, [item]);
	}
	for (const group of byGame.values()) sortLoaderVersions(group);
	const gameVersions = [...byGame.keys()].sort(compareVersions);
	return {
		byGame,
		gameVersions,
		stableGameVersions: gameVersions.filter((mc) =>
			byGame.get(mc)!.some((v) => v.stable),
		),
	};
}

export type GroupedLoaderVersions = ReturnType<typeof groupLoaderVersions>;

/** Per-modal cache. Share pending requests; failed requests can be retried. */
export function createCatalogCache<T>() {
	const requests = new Map<string, Promise<T>>();
	return {
		get(key: string, load: () => Promise<T>, refresh = false): Promise<T> {
			const cached = requests.get(key);
			if (cached && !refresh) return cached;
			const request = Promise.resolve()
				.then(load)
				.catch((error) => {
					if (requests.get(key) === request) requests.delete(key);
					throw error;
				});
			requests.set(key, request);
			return request;
		},
		clear() {
			requests.clear();
		},
	};
}
