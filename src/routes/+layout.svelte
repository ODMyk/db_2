<script>
	import { ModeWatcher } from 'mode-watcher';
	import '../app.css';
	import { Toaster } from '$lib/components/ui/sonner';
</script>

<ModeWatcher defaultMode="system"/>

<Toaster richColors position="top-center" closeButton />

<slot />
