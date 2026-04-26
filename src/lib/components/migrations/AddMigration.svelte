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
            if (migrationContext.selectedAction === "add") {
                const tags = migrationTagsInput
                    .split(",")
                    .map((x) => x.trim())
                    .filter(Boolean);

                const result = await invoke("run_add_migration", {
                    projectPath: appConfig.projectPath,
                    name: migrationName,
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
    <div class="mb-4">
        <label class="block mb-1 text-sm text-zinc-400" for="migrationName">
            Migration Name
        </label>

        <input
            id="migrationName"
            type="text"
            required
            autocomplete="off"
            bind:value={migrationName}
            placeholder="AddUsersTable"
            class="w-full bg-zinc-900 border border-zinc-700 rounded px-3 py-2"
        />
    </div>

    <div class="mb-4">
        <label class="block mb-1 text-sm text-zinc-400" for="migrationTags">
            Tags (optional)
        </label>

        <input
            type="text"
            id="migrationTags"
            bind:value={migrationTagsInput}
            placeholder="Core, Master"
            class="w-full bg-zinc-900 border border-zinc-700 rounded px-3 py-2"
        />
    </div>

    <button
        type="submit"
        disabled={loading}
        class="px-4 py-2 mt-3 bg-blue-600 hover:bg-blue-700 rounded text-white disabled:opacity-50"
    >
        {loading ? "Running..." : "Execute"}
    </button>
</form>
