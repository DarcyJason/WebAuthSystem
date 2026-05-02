<script lang="ts">
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import {ModeWatcher} from "mode-watcher";
    import { Toaster } from '$lib/components/ui/sonner/index';

    let {children} = $props();
</script>

<svelte:head>
    <link href={favicon} rel="icon"/>
</svelte:head>

<ModeWatcher/>
<Toaster/>
{@render children()}
