<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    const { node } = $props();

    let open = $state(false);
    let loading = $state(false);

    async function toggle() {
        if (!node.is_dir) return;

        open = !open;

        if (open && !node.loaded) {
            loading = true;

            node.children = await invoke("get_folder_children", {
                path: node.path,
            });

            node.loaded = true;
            loading = false;
        }
    }
</script>

<div class="text-sm">
    <a
        href={null}
        class="cursor-pointer hover:bg-zinc-800 px-2 py-1 rounded flex gap-2 items-center"
        onclick={toggle}
    >
        {#if node.is_dir}
            <span>{open ? "📂" : "📁"}</span>
        {:else}
            <span>📄</span>
        {/if}

        <span>{node.name}</span>
    </a>

    {#if open}
        <div class="ml-4 border-l border-zinc-800 pl-2">
            {#if loading}
                <div class="text-xs text-zinc-500 py-1">Loading...</div>
            {/if}

            {#each node.children as child}
                <!-- svelte-ignore svelte_self_deprecated -->
                <svelte:self node={child} />
            {/each}
        </div>
    {/if}
</div>
