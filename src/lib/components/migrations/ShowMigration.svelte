<script lang="ts">
    import toaster from "$lib/components/common/toaster";
    import { invoke } from "@tauri-apps/api/core";
    import { getContext } from "svelte";

    let migrationName = $state("");
    let migrationTagsInput = $state("");

    const migrationContext: any = getContext("migrationContext");

    let loading = $state(false);
    const appConfig: any = getContext("appConfig");

    async function runAction(e: Event) {
        e.preventDefault();
        loading = true;
        migrationContext.output = "";

        try {
            if (migrationContext.selectedAction === "show") {
                const tags = migrationTagsInput
                    .split(",")
                    .map((x) => x.trim())
                    .filter(Boolean);

                const result = await await invoke("run_show_migration", {
                    projectPath: appConfig.projectPath,
                    tags,
                });

                migrationContext.output = String(result);
            }

            migrationName = "";
            migrationTagsInput = "";

            toaster.success("Action executed successfully!");
        } catch (err: any) {
            migrationContext.output = String(err);
            toaster.error("Failed to execute action.");
        }

        loading = false;
    }
</script>

<form onsubmit={runAction} class="w-1/2">
    <button
        type="submit"
        disabled={loading}
        class="px-4 py-2 mt-6 bg-blue-600 hover:bg-blue-700 rounded text-white disabled:opacity-50"
    >
        {loading ? "Running..." : "Execute"}
    </button>
</form>
