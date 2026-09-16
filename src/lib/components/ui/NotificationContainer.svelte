<script lang="ts">
	import { launcherStore } from "$lib/state/state.svelte";
	import NotificationToast from "./NotificationToast.svelte";
</script>

<div
	class="notification-container"
	class:prominent={launcherStore.settings.prominent_notifications}
>
	{#each launcherStore.notifications as notification (notification.id)}
		<NotificationToast
			{notification}
			prominent={launcherStore.settings.prominent_notifications}
		/>
	{/each}
</div>

<style>
	.notification-container {
		position: fixed;
		top: 1.5rem;
		right: 1.5rem;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 15px;
		z-index: 9999;
		pointer-events: none;
		max-width: calc(100vw - 3rem);
	}

	.notification-container.prominent {
		top: 3rem;
		left: 50%;
		right: auto;
		transform: translateX(-50%);
		width: 504px;
		max-width: calc(100vw - 24px);
		max-height: calc(100dvh - 4rem);
		box-sizing: border-box;
		padding: 12px;
		align-items: center;
		gap: 10px;
		overflow-y: auto;
		overscroll-behavior: contain;
		pointer-events: auto;
	}

	.notification-container.prominent:empty {
		padding: 0;
	}

	.notification-container :global(.notification-toast) {
		pointer-events: all;
	}
</style>
