<script lang="ts">
    import Sidebar from "$lib/components/Sidebar.svelte";
    import { SvelteToast } from "@zerodevx/svelte-toast";
    import { onMount, setContext } from "svelte";
    import "./layout.css";
    import { invoke } from "@tauri-apps/api/core";
    import RefreshIcon from "$lib/assets/icons/RefreshIcon.svelte";

    const { children } = $props();

    let appConfig = $state({ projectPath: "" });
    setContext("appConfig", appConfig);

    let sidebarWidth = $state(320);
    let resizing = $state(false);

    let currentBranch = $state("");
    let branches = $state<string[]>([]);
    let selectedBranch = $state("");

    let loadingBranch = $state(false);
    let loadingBranches = $state(false);
    let refreshing = $state(false);
    let switching = $state(false);

    let initialized = false;

    function startResize() {
        resizing = true;
    }

    function stopResize() {
        resizing = false;
    }

    function move(e: MouseEvent) {
        if (!resizing) return;

        sidebarWidth = Math.max(220, Math.min(700, e.clientX));
    }

    async function loadGitBranch() {
        if (!appConfig.projectPath) return;

        loadingBranch = true;

        try {
            currentBranch = await invoke<string>("get_git_branch", {
                projectPath: appConfig.projectPath,
            });

            selectedBranch = currentBranch;
        } catch {
            currentBranch = "";
        } finally {
            loadingBranch = false;
        }
    }

    async function loadBranches() {
        if (!appConfig.projectPath) return;

        loadingBranches = true;

        try {
            const result = await invoke<string[]>("get_git_branches", {
                projectPath: appConfig.projectPath,
            });

            branches = result;

            if (!selectedBranch && result.length > 0) {
                selectedBranch = result[0];
            }
        } catch {
            branches = [];
        } finally {
            loadingBranches = false;
        }
    }

    async function loadGitData() {
        await loadGitBranch();
        await loadBranches();
    }

    async function refreshBranches() {
        if (!appConfig.projectPath) return;

        refreshing = true;

        try {
            await invoke("git_fetch", {
                projectPath: appConfig.projectPath,
            });

            await loadGitData();
        } finally {
            refreshing = false;
        }
    }

    async function switchBranch(value: string) {
        if (!value || value === currentBranch) return;

        switching = true;

        try {
            selectedBranch = value;

            await invoke("git_checkout", {
                projectPath: appConfig.projectPath,
                branch: value,
            });

            await loadGitData();
        } catch (err) {
            alert(`Gagal boss : ${String(err)}`);
            selectedBranch = currentBranch;
        } finally {
            switching = false;
        }
    }

    onMount(() => {
        const timer = setInterval(async () => {
            if (appConfig.projectPath && !initialized) {
                initialized = true;
                clearInterval(timer);
                await loadGitData();
            }
        }, 300);

        return () => clearInterval(timer);
    });
</script>

<svelte:window on:mousemove={move} on:mouseup={stopResize} />

<SvelteToast />

<div class="h-screen flex bg-zinc-950 text-white">
    <!-- Sidebar -->
    <Sidebar {sidebarWidth} />

    <!-- Resize -->
    <a
        href={null}
        aria-label="resize"
        class="w-1 cursor-col-resize bg-zinc-800 hover:bg-blue-500"
        onmousedown={startResize}
    ></a>

    <!-- Main -->
    <div class="flex-1 flex flex-col min-h-0">
        <!-- Header -->
        <div class="flex items-center gap-3 px-4 h-15 border-b border-zinc-800">
            <!-- Refresh -->
            <button
                onclick={refreshBranches}
                disabled={refreshing || switching}
                class="p-0 rounded hover:bg-zinc-800 disabled:opacity-40 cursor-pointer"
            >
                <RefreshIcon iconClass="size-6" />
            </button>

            <!-- Current Branch -->
            {#if loadingBranch}
                <div class="text-sm text-zinc-400">Loading branch...</div>
            {:else if currentBranch}
                <div class="flex items-center text-sm text-green-400">
                    <img
                        src="/code-branch.png"
                        alt="Git Branch"
                        class="w-5 h-5 mr-2 rounded-sm bg-red-200"
                    />

                    <span>{currentBranch}</span>
                </div>
            {/if}

            <!-- Branch Select -->
            {#if loadingBranches}
                <div class="text-sm text-zinc-400">Loading branches...</div>
            {:else}
                <select
                    bind:value={selectedBranch}
                    disabled={switching || refreshing}
                    onchange={(e: any) => switchBranch(e.target.value)}
                    class="bg-zinc-900 text-white px-3 py-1 rounded disabled:opacity-50"
                >
                    {#each branches as b}
                        <option value={b}>
                            {b}
                        </option>
                    {/each}
                </select>
            {/if}

            <!-- Status -->
            {#if switching}
                <div class="text-sm text-yellow-400">
                    👷 Mandor lagi mindahin branch...
                </div>
            {/if}

            {#if refreshing}
                <div class="text-sm text-cyan-400">
                    👷 Mandor lagi narik update...
                </div>
            {/if}
        </div>

        <!-- Hero -->
        <h1 class="text-3xl font-bold py-2 px-4">Mandor Dev 👷</h1>

        <p class="px-4">Welcome to Mandor Dev! gue siap Lo suruh-suruh</p>

        <div class="border border-zinc-600 my-4"></div>

        <!-- Content -->
        <div class="flex-1 min-h-0 overflow-auto px-4">
            {@render children()}
        </div>
    </div>
</div>
