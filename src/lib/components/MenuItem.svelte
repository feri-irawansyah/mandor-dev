<script lang="ts">
    import { page } from "$app/state";
    import ChevronUpDown from "$lib/assets/icons/ChevronUpDown.svelte";
    import DatabaseIcon from "$lib/assets/icons/DatabaseIcon.svelte";
    import HomeIcon from "$lib/assets/icons/HomeIcon.svelte";

    const patchName = $derived(page.url.pathname);

    // State buat toggle sub-menu migrations
    let isMigrationsOpen = $state(false);

    const toggleMigrations = (e: any) => {
        e.preventDefault(); // Biar gak langsung navigasi kalau lo mau klik buat buka doang
        isMigrationsOpen = !isMigrationsOpen;
    };
</script>

<div class="flex flex-col p-4 space-y-2">
    <a
        href="/"
        class="flex items-center cursor-pointer w-full transition-colors {patchName ===
        '/'
            ? 'text-sky-500'
            : 'text-zinc-400 hover:text-white'}"
    >
        <div class="flex gap-2">
            <HomeIcon iconClass="size-5" />
            <span>Beranda</span>
        </div>
        <ChevronUpDown iconClass="size-4 ml-auto" />
    </a>

    <div class="flex flex-col mb-4">
        <button
            onclick={toggleMigrations}
            class="flex items-center cursor-pointer w-full transition-colors {patchName.startsWith(
                '/migrations',
            )
                ? 'text-sky-500'
                : 'text-zinc-400 hover:text-white'}"
        >
            <div class="flex gap-2">
                <DatabaseIcon iconClass="size-5" />
                <span>Migrations</span>
            </div>
            <div
                class="ml-auto transition-transform duration-200 {isMigrationsOpen
                    ? 'rotate-180'
                    : ''}"
            >
                <ChevronUpDown iconClass="size-4" />
            </div>
        </button>

        {#if isMigrationsOpen}
            <div
                class="flex flex-col ml-7 mt-2 space-y-2 border-l border-zinc-700/50"
            >
                <a
                    href="/migrations/docs"
                    class="pl-4 py-1 text-sm transition-colors {patchName ===
                    '/migrations/docs'
                        ? 'text-sky-500 border-l border-sky-500 -ml-px'
                        : 'text-zinc-500 hover:text-white'}"
                >
                    Panduan
                </a>
                <a
                    href="/migrations"
                    class="pl-4 py-1 text-sm transition-colors {patchName ===
                    '/migrations'
                        ? 'text-sky-500 border-l border-sky-500 -ml-px'
                        : 'text-zinc-500 hover:text-white'}"
                >
                    Kerja
                </a>
            </div>
        {/if}
    </div>
</div>
