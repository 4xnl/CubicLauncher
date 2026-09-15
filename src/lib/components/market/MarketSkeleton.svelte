<div class="market-skeleton" aria-hidden="true">
	<div class="skeleton-header">
		<span class="skeleton-icon"></span>
		<div class="skeleton-identity">
			<span class="skeleton-line title"></span>
			<span class="skeleton-line author"></span>
		</div>
	</div>
	<div class="skeleton-description">
		<span class="skeleton-line"></span>
		<span class="skeleton-line short"></span>
	</div>
	<div class="skeleton-footer">
		<span class="skeleton-line author"></span>
		<span class="skeleton-button"></span>
	</div>
</div>

<style>
	.market-skeleton {
		display: flex;
		flex-direction: column;
		gap: 14px;
		height: 100%;
		box-sizing: border-box;
		padding: 14px;
		border: 1px solid var(--border);
		border-radius: var(--border-radius-sm);
		background: var(--surface-selected);
	}
	.skeleton-header,
	.skeleton-footer {
		display: flex;
		align-items: center;
		gap: 10px;
	}
	.skeleton-icon,
	.skeleton-line,
	.skeleton-button {
		display: block;
		background: var(--surface-hover);
		border-radius: var(--border-radius-sm);
	}
	.skeleton-icon {
		width: 48px;
		height: 48px;
		flex-shrink: 0;
	}
	.skeleton-identity,
	.skeleton-description {
		display: flex;
		flex-direction: column;
		gap: 8px;
		flex: 1;
	}
	.skeleton-line {
		height: 9px;
		width: 100%;
	}
	.skeleton-line.title {
		height: 13px;
		width: 80%;
	}
	.skeleton-line.author {
		width: 40%;
	}
	.skeleton-line.short {
		width: 70%;
	}
	.skeleton-footer {
		justify-content: space-between;
		padding-top: 10px;
		border-top: 1px solid var(--border);
	}
	.skeleton-button {
		width: 64px;
		height: 28px;
	}
</style>
