<script lang="ts">
    import { getContext, onMount } from "svelte";
    import TreeNode from "./TreeNode.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import MenuItem from "./MenuItem.svelte";
    import ListBullet from "$lib/assets/icons/ListBullet.svelte";
    import BuildingOffice from "$lib/assets/icons/BuildingOffice.svelte";
    import { page } from "$app/state";

    const { sidebarWidth } = $props();

    const appConfig: any = getContext("appConfig");

    let tree = $state([]);
    let menuOpen = $state(true);

    async function importProject() {
        const selected: any = await open({
            directory: true,
            multiple: false,
            title: "Pilih Project",
        });

        if (!selected) return;

        appConfig.projectPath = selected as string;

        await invoke("import_project", {
            path: appConfig.projectPath,
        });

        await loadTree();
    }

    async function loadTree() {
        tree = await invoke("get_project_tree", {
            path: appConfig.projectPath,
        });
    }

    async function loadLastProject() {
        const config: any = await invoke("get_app_config");

        if (config?.last_project) {
            appConfig.projectPath = config.last_project;
            await loadTree();
        }
    }

    onMount(() => {
        loadLastProject();
    });

    $effect(() => {
        if (!appConfig.projectPath || appConfig.projectPath === "") {
            menuOpen = false;
        } else if (page.url.pathname === "/app") {
            menuOpen = false;
        } else {
            menuOpen = true;
        }
    });
</script>

<div class="flex">
    <div class="w-10 flex flex-col items-center py-4 border-r border-zinc-800">
        <a
            href={null}
            class="mb-4 cursor-pointer text-zinc-400 hover:text-white {menuOpen
                ? 'text-sky-500'
                : ''}"
            onclick={() => (menuOpen = true)}
        >
            <ListBullet iconClass="size-6" />
        </a>
        <a
            href="/app"
            class="mb-4 cursor-pointer text-zinc-400 hover:text-white {!menuOpen
                ? 'text-sky-500'
                : ''}"
            onclick={() => (menuOpen = false)}
        >
            <BuildingOffice iconClass="size-6" />
        </a>
    </div>
    <div
        class="border-r border-zinc-800 flex flex-col shrink-0"
        style="width:{sidebarWidth}px"
    >
        {#if menuOpen}
            <MenuItem />
        {:else}
            <div class="p-4 border-b border-zinc-800">
                <button
                    onclick={importProject}
                    class="w-full bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg"
                >
                    Import Project
                </button>
            </div>

            <div class="p-3 text-sm text-zinc-400 truncate">
                {appConfig.projectPath || "Belum ada project"}
            </div>

            <div class="overflow-auto flex-1 px-2 pb-4 whitespace-nowrap">
                {#each tree as item}
                    <TreeNode node={item} />
                {/each}
            </div>
        {/if}
    </div>
</div>
