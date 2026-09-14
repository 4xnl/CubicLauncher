import { invoke } from "@tauri-apps/api/core";

/** The abort listener lives exactly as long as the native read-only request. */
export async function invokeMarket<T>(
	command: string,
	args: Record<string, unknown>,
	signal: AbortSignal,
): Promise<T> {
	signal.throwIfAborted();
	const requestId = crypto.randomUUID();
	const cancel = () => {
		void invoke("cancel_market_request", { requestId }).catch(
			console.error,
		);
	};
	signal.addEventListener("abort", cancel, { once: true });
	try {
		const result = await invoke<T>("market_request", {
			requestId,
			request: { command, args },
		});
		signal.throwIfAborted();
		return result;
	} finally {
		signal.removeEventListener("abort", cancel);
	}
}
