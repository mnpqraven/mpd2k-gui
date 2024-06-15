<script>
	import { rpc } from '$lib/ipc';
	import { onMount } from 'svelte';

	onMount(async () => {
        await rpc.init();

		rpc.load_music.on((e) => {
			console.log('yo from frontend', e);
            // update stuff from init
		});
	});
</script>

<h1>Welcome to SvelteKitarst</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
