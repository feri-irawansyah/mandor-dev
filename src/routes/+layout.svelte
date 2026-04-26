<script lang="ts">
    import Sidebar from "$lib/components/Sidebar.svelte";
    import { SvelteToast } from "@zerodevx/svelte-toast";
    import { setContext } from "svelte";
    import "./layout.css";

    const { children } = $props();

    let appConfig = $state({ projectPath: "" });

    setContext("appConfig", appConfig);

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
</script>

<svelte:window on:mousemove={move} on:mouseup={stopResize} />

<SvelteToast />

<div class="h-screen flex bg-zinc-950 text-white">
    <!-- Sidebar -->
    <Sidebar {sidebarWidth} />
    <!-- Resize Bar -->
    <a
        href={null}
        aria-label="resize"
        class="w-1 cursor-col-resize bg-zinc-800 hover:bg-blue-500"
        onmousedown={startResize}
    ></a>

    <!-- Main -->
    <div class="flex-1 flex flex-col min-h-0">
        <h1 class="text-3xl font-bold py-2 px-4">Mandor Dev 👷</h1>
        <p class="px-4">Welcome to Mandor Dev! gue siap Lo suruh-suruh</p>
        <div class="border border-zinc-600 my-4"></div>

        <div class="flex-1 min-h-0 overflow-auto px-4">
            {@render children()}
        </div>
    </div>
</div>
