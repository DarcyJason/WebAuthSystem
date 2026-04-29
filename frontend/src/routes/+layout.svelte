<script lang="ts">
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import {ModeWatcher} from "mode-watcher";
    import { Toaster } from '$lib/components/ui/sonner/index';
    import { accessToken } from '$lib/authStore';

    let {data, children} = $props();

    $effect(() => {
        if (data.accessToken) {
            accessToken.set(data.accessToken);
        }
    });
</script>

<svelte:head>
    <link href={favicon} rel="icon"/>
</svelte:head>

<ModeWatcher/>
<Toaster/>
{@render children()}
