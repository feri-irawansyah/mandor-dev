<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { getContext, onMount } from "svelte";

    import PlayIcon from "$lib/assets/icons/PlayIcon.svelte";
    import StopIcon from "$lib/assets/icons/StopIcon.svelte";
    import TrashIcon from "$lib/assets/icons/TrashIcon.svelte";

    const appConfig: any = getContext("appConfig");

    let frontendLogs = $state<string[]>([]);
    let backendLogs = $state<string[]>([]);

    let frontendRunning = $state(false);
    let backendRunning = $state(false);

    function parseLog(line: string) {
        const urlRegex = /(https?:\/\/[^\s]+)/g;

        return line.replace(
            urlRegex,
            '<a href="$1" target="_blank" class="text-blue-400 underline">$1</a>',
        );
    }

    async function runFrontend() {
        try {
            await invoke("run_frontend", {
                projectPath: `${appConfig.projectPath}\\src\\Atlas\\ClientApp`,
            });

            frontendRunning = true;
        } catch (err) {
            frontendLogs = [...frontendLogs, `💥 ${String(err)}`];
        }
    }

    async function stopFrontend() {
        try {
            await invoke("stop_frontend");
            frontendRunning = false;
        } catch (err) {
            frontendLogs = [...frontendLogs, `💥 ${String(err)}`];
        }
    }

    function clearFrontend() {
        frontendLogs = [];
    }

    async function checkStatus() {
        try {
            const current = await invoke<boolean>("check_frontend_status");
            const backendCurrent = await invoke<boolean>(
                "check_backend_status",
            );

            // cuma update kalau berubah
            if (current !== frontendRunning) {
                frontendRunning = current;

                if (current) {
                    frontendLogs = [
                        ...frontendLogs,
                        "🟢 Frontend masih kerja bos...",
                    ];
                } else {
                    frontendLogs = [
                        ...frontendLogs,
                        "🔴 Frontend berhenti kerja.",
                    ];
                }
            }

            if (backendCurrent !== backendRunning) {
                backendRunning = backendCurrent;

                if (backendCurrent) {
                    backendLogs = [
                        ...backendLogs,
                        "🟢 Backend masih kerja bos...",
                    ];
                } else {
                    backendLogs = [
                        ...backendLogs,
                        "🔴 Backend berhenti kerja.",
                    ];
                }
            }
        } catch {
            if (frontendRunning) {
                frontendRunning = false;

                frontendLogs = [...frontendLogs, "💥 Frontend tumbang bos."];
            }

            if (backendRunning) {
                backendRunning = false;

                backendLogs = [...backendLogs, "💥 Backend tumbang bos."];
            }
        }
    }

    async function runBackend() {
        try {
            await invoke("run_backend", {
                projectPath: `${appConfig.projectPath}\\src\\Atlas`,
            });

            backendRunning = true;
        } catch (err) {
            backendLogs = [...backendLogs, `💥 ${String(err)}`];
        }
    }

    async function stopBackend() {
        try {
            await invoke("stop_backend");
            backendRunning = false;
        } catch (err) {
            backendLogs = [...backendLogs, `💥 ${String(err)}`];
        }
    }

    function clearBackend() {
        backendLogs = [];
    }

    onMount(() => {
        const init = async () => {
            await listen("frontend-log", (event) => {
                frontendLogs = [...frontendLogs, String(event.payload)];
            });

            await listen("backend-log", (event) => {
                backendLogs = [...backendLogs, String(event.payload)];
            });

            await checkStatus();
        };

        init();

        const timer = setInterval(() => {
            checkStatus();
        }, 2000);

        return () => clearInterval(timer);
    });
</script>

<div
    class="flex w-full h-150 border border-zinc-800 rounded-xl overflow-hidden"
>
    <!-- FRONTEND -->
    <div class="w-1/2 border-r border-zinc-800 flex flex-col bg-black">
        <!-- Header -->
        <div
            class="p-2 flex justify-between items-center border-b border-zinc-800"
        >
            <h2 class="text-2xl text-white">Frontend</h2>

            <div class="flex">
                <button
                    class="p-2 cursor-pointer disabled:opacity-30"
                    onclick={runFrontend}
                    disabled={frontendRunning}
                >
                    <PlayIcon iconClass="size-5 text-white" />
                </button>

                <button
                    class="p-2 cursor-pointer disabled:opacity-30"
                    onclick={stopFrontend}
                    disabled={!frontendRunning}
                >
                    <StopIcon iconClass="size-5 text-white" />
                </button>

                <button class="p-2 cursor-pointer" onclick={clearFrontend}>
                    <TrashIcon iconClass="size-4 text-white" />
                </button>
            </div>
        </div>

        <!-- Logs -->
        <div
            class="flex-1 overflow-auto p-3 font-mono text-sm text-green-400 space-y-1"
        >
            {#each frontendLogs as log}
                <div>
                    {@html parseLog(log)}
                </div>
            {/each}
        </div>
    </div>

    <!-- BACKEND -->
    <div class="w-1/2 flex flex-col bg-black">
        <!-- Header -->
        <div
            class="p-2 flex justify-between items-center border-b border-zinc-800"
        >
            <h2 class="text-2xl text-white">Backend</h2>

            <div class="flex">
                <button
                    class="p-2 cursor-pointer disabled:opacity-30"
                    onclick={runBackend}
                    disabled={backendRunning}
                >
                    <PlayIcon iconClass="size-5 text-white" />
                </button>

                <button
                    class="p-2 cursor-pointer disabled:opacity-30"
                    onclick={stopBackend}
                    disabled={!backendRunning}
                >
                    <StopIcon iconClass="size-5 text-white" />
                </button>

                <button class="p-2 cursor-pointer" onclick={clearBackend}>
                    <TrashIcon iconClass="size-4 text-white" />
                </button>
            </div>
        </div>

        <!-- Logs -->
        <div
            class="flex-1 overflow-auto p-3 font-mono text-sm text-cyan-400 space-y-1"
        >
            {#each backendLogs as log}
                <div>
                    {@html parseLog(log)}
                </div>
            {/each}
        </div>
    </div>
</div>
