<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { getContext, onMount } from "svelte";
    import VirtualList from "@sveltejs/svelte-virtual-list";

    import PlayIcon from "$lib/assets/icons/PlayIcon.svelte";
    import StopIcon from "$lib/assets/icons/StopIcon.svelte";
    import TrashIcon from "$lib/assets/icons/TrashIcon.svelte";

    const appConfig: any = getContext("appConfig");

    let frontendLogs = $state<string[]>([]);
    let backendLogs = $state<string[]>([]);

    let frontendRunning = $state(false);
    let backendRunning = $state(false);

    let checking = false;

    const MAX_LOG = 5000;

    function appendFrontend(msg: string) {
        frontendLogs = [...frontendLogs, msg].slice(-MAX_LOG);
    }

    function appendBackend(msg: string) {
        backendLogs = [...backendLogs, msg].slice(-MAX_LOG);
    }

    // function parseLog(line: string) {
    //     const escaped = line
    //         .replace(/&/g, "&amp;")
    //         .replace(/</g, "&lt;")
    //         .replace(/>/g, "&gt;");

    //     const urlRegex = /(https?:\/\/[^\s]+)/g;

    //     return escaped.replace(
    //         urlRegex,
    //         '<a href="$1" target="_blank" class="text-blue-400 underline">$1</a>',
    //     );
    // }
    function parseLog(line: string) {
        const escaped = line
            .replace(/&/g, "&amp;")
            .replace(/</g, "&lt;")
            .replace(/>/g, "&gt;");

        const lower = line.toLowerCase();

        let colorClass = "";

        if (
            lower.includes("error") ||
            lower.includes("failed") ||
            lower.includes("exception") ||
            lower.includes("panic") ||
            lower.includes("cannot") ||
            lower.includes("denied") ||
            lower.includes("exit code 1") ||
            lower.includes("💥")
        ) {
            colorClass = "text-red-400";
        } else if (lower.includes("warn") || lower.includes("warning")) {
            colorClass = "text-yellow-400";
        } else if (
            lower.includes("ready") ||
            lower.includes("started") ||
            lower.includes("running") ||
            lower.includes("compiled") ||
            lower.includes("success") ||
            lower.includes("🟢")
        ) {
            colorClass = "text-green-400";
        }

        const urlRegex = /(https?:\/\/[^\s]+)/g;

        const html = escaped.replace(
            urlRegex,
            '<a href="$1" target="_blank" class="text-blue-400 underline">$1</a>',
        );

        return `<span class="${colorClass}">${html}</span>`;
    }

    async function runFrontend() {
        try {
            await invoke("run_frontend", {
                projectPath: `${appConfig.projectPath}\\src\\Atlas\\ClientApp`,
            });

            frontendRunning = true;
            appendFrontend("🟢 Frontend mulai kerja bos.");
        } catch (err) {
            appendFrontend(`💥 ${String(err)}`);
        }
    }

    async function stopFrontend() {
        try {
            await invoke("stop_frontend");
            frontendRunning = false;
            appendFrontend("🛑 Frontend dibubarin bos.");
        } catch (err) {
            appendFrontend(`💥 ${String(err)}`);
        }
    }

    function clearFrontend() {
        frontendLogs = [];
    }

    async function runBackend() {
        try {
            await invoke("run_backend", {
                projectPath: `${appConfig.projectPath}\\src\\Atlas`,
            });

            backendRunning = true;
            appendBackend("🟢 Backend mulai kerja bos.");
        } catch (err) {
            appendBackend(`💥 ${String(err)}`);
        }
    }

    async function stopBackend() {
        try {
            await invoke("stop_backend");
            backendRunning = false;
            appendBackend("🛑 Backend dibubarin bos.");
        } catch (err) {
            appendBackend(`💥 ${String(err)}`);
        }
    }

    function clearBackend() {
        backendLogs = [];
    }

    async function checkStatus() {
        if (checking) return;

        checking = true;

        try {
            const [front, back] = await Promise.all([
                invoke<boolean>("check_frontend_status"),
                invoke<boolean>("check_backend_status"),
            ]);

            if (front !== frontendRunning) {
                frontendRunning = front;

                appendFrontend(
                    front
                        ? "🟢 Frontend masih kerja bos..."
                        : "🔴 Frontend berhenti kerja.",
                );
            }

            if (back !== backendRunning) {
                backendRunning = back;

                appendBackend(
                    back
                        ? "🟢 Backend masih kerja bos..."
                        : "🔴 Backend berhenti kerja.",
                );
            }
        } catch {
            if (frontendRunning) {
                frontendRunning = false;
                appendFrontend("💥 Frontend tumbang bos.");
            }

            if (backendRunning) {
                backendRunning = false;
                appendBackend("💥 Backend tumbang bos.");
            }
        } finally {
            checking = false;
        }
    }

    onMount(() => {
        let unlistenFront: any;
        let unlistenBack: any;

        const init = async () => {
            unlistenFront = await listen("frontend-log", (event) => {
                appendFrontend(String(event.payload));
            });

            unlistenBack = await listen("backend-log", (event) => {
                appendBackend(String(event.payload));
            });

            await checkStatus();
        };

        init();

        const timer = setInterval(checkStatus, 2000);

        return () => {
            clearInterval(timer);

            if (unlistenFront) unlistenFront();
            if (unlistenBack) unlistenBack();
        };
    });
</script>

<div
    class="flex w-full h-150 border border-zinc-800 rounded-xl overflow-hidden"
>
    <!-- FRONTEND -->
    <div class="w-1/2 border-r border-zinc-800 flex flex-col bg-black">
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

        <div
            class="flex-1 overflow-hidden p-2 text-green-400 font-mono text-sm"
        >
            <VirtualList items={frontendLogs} itemHeight={24} let:item>
                <div class="px-1 whitespace-pre-wrap break-all">
                    {@html parseLog(item)}
                </div>
            </VirtualList>
        </div>
    </div>

    <!-- BACKEND -->
    <div class="w-1/2 flex flex-col bg-black">
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

        <div class="flex-1 overflow-hidden p-2 text-cyan-400 font-mono text-sm">
            <VirtualList items={backendLogs} itemHeight={24} let:item>
                <div class="px-1 whitespace-pre-wrap break-all">
                    {@html parseLog(item)}
                </div>
            </VirtualList>
        </div>
    </div>
</div>
