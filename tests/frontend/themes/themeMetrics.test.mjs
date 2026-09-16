import { afterEach, beforeEach, expect, mock, test } from "bun:test";
import { observeThemeMetrics } from "../../../src/lib/utils/themeMetrics.ts";

let originals, observers, target, stop;

class Element {
	children = [];
	style = { cssText: "" };
	width = "52px";
	setAttribute = mock();
	appendChild(child) {
		this.children.push(child);
		child.parent = this;
	}
	remove() {
		this.parent.children = this.parent.children.filter(
			(child) => child !== this,
		);
	}
}

beforeEach(() => {
	stop = undefined;
	observers = [];
	target = new Element();
	originals = new Map();
	const globals = {
		document: { createElement: () => new Element() },
		getComputedStyle: (node) => ({ width: node.width }),
		ResizeObserver: class {
			constructor(callback) {
				this.notify = callback;
				observers.push(this);
			}
			observe = mock();
			disconnect = mock();
		},
	};
	for (const [key, value] of Object.entries(globals)) {
		originals.set(key, Object.getOwnPropertyDescriptor(globalThis, key));
		Object.defineProperty(globalThis, key, {
			value,
			configurable: true,
			writable: true,
		});
	}
});

afterEach(() => {
	stop?.();
	for (const [key, descriptor] of originals) {
		if (descriptor) Object.defineProperty(globalThis, key, descriptor);
		else delete globalThis[key];
	}
});

test("theme/font changes update pixel measurements without duplicate notifications", () => {
	const changed = mock();
	stop = observeThemeMetrics(
		target,
		{
			row: { variable: "--sidebar-row-height", fallback: 52 },
			gap: {
				variable: "--sidebar-row-gap",
				fallback: 8,
				allowZero: true,
			},
		},
		changed,
	);
	const [row, gap] = target.children[0].children;
	expect(changed).toHaveBeenLastCalledWith({ row: 52, gap: 52 });
	row.width = "80.5px";
	gap.width = "0px";
	observers[0].notify();
	expect(changed).toHaveBeenLastCalledWith({ row: 80.5, gap: 0 });
	observers[0].notify();
	expect(changed).toHaveBeenCalledTimes(2);
	// Removing a custom override restores the CSS fallback, without remounting.
	row.width = "52px";
	gap.width = "8px";
	observers[0].notify();
	expect(changed).toHaveBeenLastCalledWith({ row: 52, gap: 8 });
});

test("invalid and zero row sizes fall back before virtual-list division", () => {
	const changed = mock();
	stop = observeThemeMetrics(
		target,
		{
			row: { variable: "--resource-row-height", fallback: 130 },
		},
		changed,
	);
	const row = target.children[0].children[0];
	for (const value of ["0px", "auto", "-10px", "NaN", "Infinity"]) {
		row.width = value;
		observers[0].notify();
		expect(changed).toHaveBeenLastCalledWith({ row: 130 });
	}
	row.width = "160px";
	observers[0].notify();
	expect(changed).toHaveBeenLastCalledWith({ row: 160 });
});

test("unmount removes probes and ignores already queued observer callbacks", () => {
	const changed = mock();
	stop = observeThemeMetrics(
		target,
		{
			row: { variable: "--version-row-height", fallback: 78 },
		},
		changed,
	);
	const probe = target.children[0].children[0];
	stop();
	stop = undefined;
	expect(target.children).toHaveLength(0);
	expect(observers[0].disconnect).toHaveBeenCalledTimes(1);
	probe.width = "200px";
	observers[0].notify();
	expect(changed).toHaveBeenCalledTimes(1);
});
