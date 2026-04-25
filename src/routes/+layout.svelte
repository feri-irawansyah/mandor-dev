<script lang="ts">
    import "./layout.css";
    import { onMount } from "svelte";
    import { open } from "@tauri-apps/plugin-dialog";
    import { invoke } from "@tauri-apps/api/core";
    import TreeNode from "../lib/components/TreeNode.svelte";

    const { children } = $props();

    let projectPath = $state("");
    let tree = $state([]);

    let sidebarWidth = $state(320);
    let resizing = $state(false);

    function startResize() {
        resizing = true;
    }

    function stopResize() {
        resizing = false;
    }

    function move(e: any) {
        if (!resizing) return;

        sidebarWidth = Math.max(220, Math.min(700, e.clientX));
    }

    async function importProject() {
        const selected = await open({
            directory: true,
            multiple: false,
            title: "Pilih Project",
        });

        if (!selected) return;

        projectPath = selected as string;

        await invoke("import_project", {
            path: projectPath,
        });

        await loadTree();
    }

    async function loadTree() {
        tree = await invoke("get_project_tree", {
            path: projectPath,
        });
    }

    async function loadLastProject() {
        const config: any = await invoke("get_app_config");

        if (config?.last_project) {
            projectPath = config.last_project;
            await loadTree();
        }
    }

    onMount(() => {
        loadLastProject();
    });
</script>

<svelte:window on:mousemove={move} on:mouseup={stopResize} />

<div class="h-screen flex bg-zinc-950 text-white">
    <!-- Sidebar -->
    <div
        class="border-r border-zinc-800 flex flex-col shrink-0"
        style="width:{sidebarWidth}px"
    >
        <div class="p-4 border-b border-zinc-800">
            <button
                onclick={importProject}
                class="w-full bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg"
            >
                Import Project
            </button>
        </div>

        <div class="p-3 text-sm text-zinc-400 truncate">
            {projectPath || "Belum ada project"}
        </div>

        <div class="overflow-auto flex-1 px-2 pb-4 whitespace-nowrap">
            {#each tree as item}
                <TreeNode node={item} />
            {/each}
        </div>
    </div>

    <!-- Resize Bar -->
    <a
        href={null}
        aria-label="resize"
        class="w-1 cursor-col-resize bg-zinc-800 hover:bg-blue-500"
        onmousedown={startResize}
    ></a>

    <!-- Main -->
    <div class="flex-1 flex flex-col min-h-0 p-8">
        <h1 class="text-3xl font-bold mb-4">Mandor Dev 👷</h1>
        <p>Welcome to Mandor Dev! gue siap Lo suruh-suruh</p>

        <div class="flex gap-4 mt-3">
            <a href="/" class="text-zinc-200 py-2 px-4 bg-blue-500 rounded-md"
                >Home</a
            >
            <a
                href="/migrations"
                class="text-zinc-200 py-2 px-4 bg-pink-500 rounded-md"
                >Migrations</a
            >
            <a
                href="/crud-feature"
                class="text-zinc-200 py-2 px-4 bg-teal-500 rounded-md"
                >Crud Feature</a
            >
        </div>

        <div class="flex-1 min-h-0 overflow-auto mt-4">
            {@render children()}
        </div>
    </div>
</div>
