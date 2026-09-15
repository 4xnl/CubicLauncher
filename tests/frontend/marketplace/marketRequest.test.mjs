import { expect, test } from "bun:test";

test("native market bridge unregisters abort listeners on completion, cancellation and failure", () => {
	// Isolate the Tauri mock from suites that exercise the real API wrappers.
	const result = Bun.spawnSync([
		process.execPath,
		new URL("./fixtures/marketRequest.mjs", import.meta.url).pathname,
	]);
	expect(result.exitCode, result.stderr.toString()).toBe(0);
});
