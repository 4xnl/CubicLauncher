<script lang="ts">
	import { onMount, onDestroy, untrack, type Snippet } from "svelte";
	import type { MarketProject } from "$lib/types/market";
	import MarketSkeleton from "./MarketSkeleton.svelte";

	let {
		count,
		getItem,
		busy,
		active,
		onRangeNeeded,
		onLoadMore,
		children,
	}: {
		count: number;
		getItem: (index: number) => MarketProject | null | undefined;
		busy: boolean;
		active: boolean;
		onRangeNeeded: (first: number, last: number) => void;
		onLoadMore: () => void;
		children: Snippet<[MarketProject]>;
	} = $props();
	const rowHeight = 224;
	let container: HTMLDivElement;
	let width = $state(0);
	let height = $state(0);
	let scrollTop = $state(0);
	let frame: number | undefined;
	const columns = $derived(
		Math.max(1, Math.min(4, Math.floor((width - 8 + 12) / 292))),
	);
	const rowCount = $derived(Math.ceil(count / columns));
	const firstRow = $derived(
		Math.max(0, Math.floor(scrollTop / rowHeight) - 2),
	);
	const lastRow = $derived(
		Math.min(rowCount, Math.ceil((scrollTop + height) / rowHeight) + 2),
	);
	const rows = $derived(
		Array.from(
			{ length: Math.max(0, lastRow - firstRow) },
			(_, i) => firstRow + i,
		),
	);

	$effect(() => {
		if (busy || !active || !count) return;
		const first = firstRow * columns;
		const last = Math.min(count - 1, lastRow * columns - 1);
		untrack(() => onRangeNeeded(first, last));
	});

	function handleScroll() {
		if (frame !== undefined) return;
		frame = requestAnimationFrame(() => {
			frame = undefined;
			scrollTop = container.scrollTop;
			if (active && container.scrollHeight - scrollTop - height < 500)
				onLoadMore();
		});
	}
	onMount(() => {
		const observer = new ResizeObserver(([entry]) => {
			width = entry.contentRect.width;
			height = entry.contentRect.height;
			scrollTop = container.scrollTop;
		});
		observer.observe(container);
		return () => observer.disconnect();
	});
	onDestroy(() => {
		if (frame !== undefined) cancelAnimationFrame(frame);
	});
</script>

<div
	class="market-grid"
	bind:this={container}
	onscroll={handleScroll}
	style:--row-height={`${rowHeight}px`}
>
	<div class="grid-space" style:height={`${rowCount * rowHeight}px`}>
		{#each rows as row (row)}
			<div
				class="grid-row"
				style:--columns={columns}
				style:transform={`translateY(${row * rowHeight}px)`}
			>
				{#each Array.from({ length: columns }, (_, i) => row * columns + i) as index (index)}
					{#if index < count}
						{@const project = getItem(index)}
						{#if project}
							{#key project.id}{@render children(project)}{/key}
						{:else}
							<div
								class="placeholder"
								class:duplicate={project === null}
								aria-hidden="true"
							>
								{#if project !== null}<MarketSkeleton />{/if}
							</div>
						{/if}
					{/if}
				{/each}
			</div>
		{/each}
	</div>
</div>

<style>
	.market-grid {
		height: 100%;
		overflow-y: auto;
		overflow-anchor: none;
	}
	.grid-space {
		position: relative;
		width: 100%;
	}
	.grid-row {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		box-sizing: border-box;
		display: grid;
		grid-template-columns: repeat(var(--columns), minmax(0, 1fr));
		gap: 12px;
		height: calc(var(--row-height) - 12px);
		padding-right: 8px;
	}
	.placeholder {
		min-width: 0;
		min-height: 0;
	}
	.placeholder.duplicate {
		visibility: hidden;
	}
</style>
