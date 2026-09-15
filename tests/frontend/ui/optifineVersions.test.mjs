import { describe, expect, test } from "bun:test";
import {
	getInstalledMcVersions,
	getInstalledLoaderVersions,
	parseInstalledVersion,
} from "../../../src/lib/utils/versionUtils.ts";

describe("installed OptiFine versions", () => {
	test("keeps OptiFine out of Vanilla and groups releases for instance selection", () => {
		const versions = [
			"1.20.1",
			"1.20.1-OptiFine_HD_U_I6",
			"1.20.1-OptiFine_HD_U_I5",
			"1.21-OptiFine_HD_U_J1_pre9",
		];
		const installed = getInstalledMcVersions(versions);
		expect([...installed.vanilla]).toEqual(["1.20.1"]);
		expect([...installed.optifine]).toEqual(versions.slice(1));
		const loaders = getInstalledLoaderVersions(versions);
		expect([...loaders.get("optifine:1.20.1")]).toEqual([
			"HD_U_I6",
			"HD_U_I5",
		]);
		expect([...loaders.get("optifine:1.21")]).toEqual(["HD_U_J1_pre9"]);
		expect(parseInstalledVersion(versions[1]).loader).toBe("optifine");
	});
});
