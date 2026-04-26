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
    let branches = $state<string[]>([]);
    let selectedBranch = $state("");

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

    async function loadGitBranch() {
        try {
            return await invoke("get_git_branch", {
                projectPath: appConfig.projectPath,
            });
        } catch (err) {
            console.error("Failed to get git branch:", err);
            return null;
        }
    }

    async function loadBranches(): Promise<string[]> {
        try {
            const result = await invoke<string[]>("get_git_branches", {
                projectPath: appConfig.projectPath,
            });

            branches = result;
            return result;
        } catch {
            branches = [];
            return [];
        }
    }

    async function refreshBranches() {
        await invoke("git_fetch", {
            projectPath: appConfig.projectPath,
        });

        window.location.reload();
    }

    async function switchBranch(value: string) {
        await invoke("git_checkout", {
            projectPath: appConfig.projectPath,
            branch: value,
        });

        window.location.reload();
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
        <div class="flex items-center gap-2">
            <button
                onclick={refreshBranches}
                class="p-2 rounded hover:bg-zinc-800 cursor-pointer"
            >
                <RefreshIcon iconClass="size-6" />
            </button>
            {#await loadGitBranch()}
                <div class="text-sm text-zinc-400">Loading branch...</div>
            {:then branch}
                <div class="flex items-center text-sm text-green-400">
                    <img
                        src="/code-branch.png"
                        alt="Git Branch"
                        class="inline-block w-5 h-5 mr-1 bg-red-300 rounded-sm"
                    />
                    <span>{branch}</span>
                </div>
            {/await}

            {#await loadBranches()}
                <div class="text-sm text-zinc-400">Loading branches...</div>
            {:then loadedBranches}
                <select
                    onchange={(e: any) => {
                        const value = e.target.value;
                        if (value) {
                            switchBranch(value);
                        }
                    }}
                    class="bg-zinc-900 text-white px-2 py-1 rounded"
                >
                    {#each loadedBranches as b}
                        <option value={b}>{b}</option>
                    {/each}
                </select>
            {/await}
        </div>

        <h1 class="text-3xl font-bold py-2 px-4">Mandor Dev 👷</h1>
        <p class="px-4">Welcome to Mandor Dev! gue siap Lo suruh-suruh</p>
        <div class="border border-zinc-600 my-4"></div>

        <div class="flex-1 min-h-0 overflow-auto px-4">
            {@render children()}
        </div>
    </div>
</div>
