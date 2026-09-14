<script lang="ts">
	import { onDestroy } from "svelte";
	import { t } from "$lib/i18n";
	import type { InstanceDto } from "$lib/types/types";
	import { createMarketState } from "$lib/state/marketState.svelte";
	import type { ContentType } from "$lib/types/market";
	import MarketFilterPanel from "$lib/components/market/MarketFilterPanel.svelte";
	import MarketItem from "$lib/components/market/MarketItem.svelte";
	import MarketDetail from "$lib/components/market/MarketDetail.svelte";
	import MarketEmptyState from "$lib/components/market/MarketEmptyState.svelte";
	import MarketLayout from "$lib/components/market/MarketLayout.svelte";

	interface Props {
		instance: InstanceDto;
		contentType?: ContentType;
	}

	let { instance, contentType = "mods" }: Props = $props();

	function init() {
		return createMarketState(instance, contentType);
	}
	const state = init();

	onDestroy(() => {
		try {
			state.destroy();
		} catch (e) {
			console.error("[Market] destroy error:", e);
		}
	});

	const emptyState = $derived.by(() => {
		if (
			state.filters.query.trim() ||
			(state.filters.source === "local"
				? state.filters.localSource !== "all"
				: state.filters.category !== null)
		) {
			return {
				title: t("market.empty.searchTitle"),
				subtitle: t("market.empty.searchSubtitle"),
			};
		}
		if (state.filters.source === "local") {
			return {
				title: t("market.empty.localTitle"),
				subtitle: t("market.empty.localSubtitle"),
			};
		}
		return {
			title: t("market.empty.marketTitle"),
			subtitle: t("market.empty.marketSubtitle"),
		};
	});
</script>

<div class="market-root">
	<MarketLayout
		items={state.items}
		itemCount={state.itemCount}
		getItem={state.getItem}
		onRangeNeeded={state.ensureRange}
		total={state.total}
		resultsRevision={state.resultsRevision}
		selectedId={state.selectedProject?.id ?? null}
		detailTitle={state.selectedProject?.title ?? ""}
		loading={state.loading}
		loadingMore={state.loadingMore}
		hasMore={state.hasMore}
		error={state.error}
		onClose={() => state.selectProject(null)}
		onRetry={state.retry}
		onLoadMore={state.loadMore}
	>
		{#snippet filterPanel()}
			<MarketFilterPanel
				filters={state.filters}
				{contentType}
				active={state.selectedProject === null}
				onSourceChange={state.setSource}
				onQueryChange={state.setQuery}
				onSearch={state.refresh}
				onSortChange={state.setSort}
				onCategoryChange={state.setCategory}
				onLocalSortChange={state.setLocalSort}
				onLocalSourceChange={state.setLocalSource}
				onClearFilters={state.clearFilters}
			/>
		{/snippet}

		{#snippet emptySnippet()}
			<MarketEmptyState
				title={emptyState.title}
				subtitle={emptyState.subtitle}
			/>
			<div class="empty-actions">
				{#if state.filters.query}
					<button type="button" onclick={() => state.setQuery("")}
						>{t("market.filter.clearSearch")}</button
					>
				{/if}
				{#if state.filters.source === "local" ? state.filters.localSource !== "all" : state.filters.category !== null}
					<button type="button" onclick={state.clearFilters}
						>{t("market.browse.clearFilters")}</button
					>
				{/if}
			</div>
		{/snippet}

		{#snippet itemSnippet(project)}
			<MarketItem
				{project}
				selected={project.id === state.selectedId}
				onSelect={() => state.selectProject(project.id)}
				onInstall={state.filters.source !== "local"
					? () => state.selectProject(project.id)
					: undefined}
			/>
		{/snippet}

		{#snippet detailSnippet()}
			{#if state.selectedProject}
				{@const project = state.selectedProject}
				<MarketDetail
					{project}
					source={state.filters.source}
					{contentType}
					detail={state.detail}
					selectedVersion={state.selectedVersion}
					isVersionCompatible={state.isVersionCompatible}
					onVersionSelect={state.setSelectedVersion}
					onPrepareInstall={() => {
						const version = state.selectedVersion;
						if (!version) throw new Error("No version selected");
						return state.prepareInstall(project, version);
					}}
					onInstallQueue={(queue) =>
						state.confirmInstall(project, queue)}
					onUninstall={() => state.uninstall(project)}
					onToggleEnabled={() => state.toggleEnabled(project)}
					onClose={() => state.selectProject(null)}
				/>
			{/if}
		{/snippet}
	</MarketLayout>
</div>

<style>
	.empty-actions {
		display: flex;
		gap: 10px;
		flex-wrap: wrap;
		justify-content: center;
	}
	.empty-actions button {
		padding: 8px 14px;
		border: 1px solid var(--border);
		border-radius: var(--border-radius-sm);
		background: var(--surface-card);
		color: var(--text-primary);
		font: inherit;
		font-size: 0.8rem;
		cursor: pointer;
	}
	.empty-actions button:focus-visible {
		outline: 2px solid var(--accent);
		outline-offset: 2px;
	}
	.empty-actions button:hover {
		border-color: var(--accent);
	}
	.market-root {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		background: var(--bg-main);
	}
</style>
