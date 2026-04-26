<script lang="ts">
    import AddMigration from "$lib/components/migrations/AddMigration.svelte";
    import ShowMigration from "$lib/components/migrations/ShowMigration.svelte";
    import { setContext } from "svelte";

    let migrationContext = $state({
        selectedAction: "add",
        output: "",
    });

    setContext("migrationContext", migrationContext);
</script>

<h2 class="text-2xl font-bold mb-4">Migrations</h2>

<div class="flex space-y-4 min-w-full w-full rounded gap-3">
    <div class="flex flex-col space-y-3 w-1/2">
        <label class="block mb-1 text-sm text-zinc-400" for="action">
            Action
        </label>

        <select
            id="action"
            required
            bind:value={migrationContext.selectedAction}
            class="w-full bg-zinc-900 border border-zinc-700 rounded px-3 py-2"
        >
            <option value="add"> Add Migration </option>
            <option value="up"> Up Migration </option>
            <option value="undo"> Undo Migration </option>
            <option value="show"> Show Migrations </option>
        </select>
        {#if migrationContext.output}
            <pre
                class="bg-zinc-900 border border-zinc-800 rounded p-4 text-sm whitespace-pre-wrap w-full max-w-full overflow-auto">{migrationContext.output}</pre>
        {/if}
    </div>
    {#if migrationContext.selectedAction === "add"}
        <AddMigration />
    {:else if migrationContext.selectedAction === "up"}
        <p>Up Migration - Not implemented yet</p>
    {:else if migrationContext.selectedAction === "undo"}
        <p>Undo Migration - Not implemented yet</p>
    {:else if migrationContext.selectedAction === "show"}
        <ShowMigration />
    {/if}
</div>
