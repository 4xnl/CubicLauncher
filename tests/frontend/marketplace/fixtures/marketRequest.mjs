import assert from "node:assert/strict";
import { mock } from "bun:test";

let resolve, reject;
const calls = [];
mock.module("@tauri-apps/api/core", () => ({
	invoke: (command, args) => {
		calls.push({ command, args });
		if (command === "cancel_market_request") {
			reject("MARKET_CANCELLED");
			return Promise.resolve();
		}
		return new Promise((yes, no) => {
			resolve = yes;
			reject = no;
		});
	},
}));
const { invokeMarket } =
	await import("../../../../src/lib/api/marketRequest.ts");

for (const mode of ["success", "error", "cancel"]) {
	const controller = new AbortController();
	let listeners = 0;
	const add = controller.signal.addEventListener.bind(controller.signal);
	const remove = controller.signal.removeEventListener.bind(
		controller.signal,
	);
	controller.signal.addEventListener = (...args) => {
		listeners++;
		return add(...args);
	};
	controller.signal.removeEventListener = (...args) => {
		listeners--;
		return remove(...args);
	};
	const request = invokeMarket(
		"get_modrinth_project",
		{ projectId: "test" },
		controller.signal,
	);
	assert.equal(listeners, 1);
	if (mode === "success") {
		resolve({ id: "test" });
		assert.deepEqual(await request, { id: "test" });
	} else {
		const checked = assert.rejects(request);
		if (mode === "cancel") controller.abort();
		else reject(new Error("HTTP failed"));
		await checked;
	}
	assert.equal(listeners, 0);
	const count = calls.length;
	controller.abort();
	assert.equal(calls.length, count);
}

const stopped = new AbortController();
stopped.abort();
const count = calls.length;
await assert.rejects(invokeMarket("get_modrinth_project", {}, stopped.signal));
assert.equal(calls.length, count);
