<script lang="ts">
	import * as Card from "$lib/components/ui/card/index.js";
	import { Button } from "$lib/components/ui/button/index.js";

	let videoEl: HTMLVideoElement;
	let canvasEl: HTMLCanvasElement;
	let capturedImage: string | null = $state(null);
	let streaming = $state(false);
	let stream: MediaStream | null = null;

	async function startCamera() {
		stream = await navigator.mediaDevices.getUserMedia({ video: true });
		videoEl.srcObject = stream;
		videoEl.play();
		streaming = true;
	}

	function stopCamera() {
		stream?.getTracks().forEach(t => t.stop());
		videoEl.srcObject = null;
		streaming = false;
	}

	function capture() {
		const ctx = canvasEl.getContext("2d")!;
		canvasEl.width = videoEl.videoWidth;
		canvasEl.height = videoEl.videoHeight;
		ctx.drawImage(videoEl, 0, 0);
		capturedImage = canvasEl.toDataURL("image/png");
		stopCamera();
	}

	function reset() {
		capturedImage = null;
	}
</script>

<div class="flex h-screen items-center justify-center gap-8 p-12 bg-background">
	<!-- Left Card: Kamera -->
	<Card.Root class="flex-1 max-w-xl h-[520px] flex flex-col shadow-lg">
		<Card.Header class="pb-3">
			<Card.Title class="flex items-center gap-2 text-xl">
				<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/>
					<circle cx="12" cy="13" r="3"/>
				</svg>
				Kamera
			</Card.Title>
			<Card.Description>Foto temporär — wird nicht gespeichert</Card.Description>
		</Card.Header>
		<Card.Content class="flex-1 flex flex-col items-center justify-center gap-4">
			{#if capturedImage}
				<img src={capturedImage} alt="Screenshot" class="rounded-lg object-contain max-h-80 w-full" />
				<Button size="lg" variant="outline" onclick={reset}>
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2">
						<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/>
						<path d="M3 3v5h5"/>
					</svg>
					Neu aufnehmen
				</Button>
			{:else if streaming}
				<!-- svelte-ignore element_invalid_self_closing_tag -->
				<video bind:this={videoEl} class="rounded-lg w-full max-h-80 object-cover bg-black" muted></video>
				<Button size="lg" onclick={capture}>
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2">
						<circle cx="12" cy="12" r="10"/>
						<circle cx="12" cy="12" r="3" fill="currentColor"/>
					</svg>
					Aufnehmen
				</Button>
			{:else}
				<div class="flex flex-col items-center gap-6 text-muted-foreground">
					<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="opacity-30">
						<path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/>
						<circle cx="12" cy="13" r="3"/>
					</svg>
					<Button size="lg" onclick={startCamera}>
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2">
							<path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/>
							<circle cx="12" cy="13" r="3"/>
						</svg>
						Kamera öffnen
					</Button>
				</div>
			{/if}
		</Card.Content>
	</Card.Root>

	<!-- Right Card: Platzhalter -->
	<Card.Root class="flex-1 max-w-xl h-[520px] flex flex-col shadow-lg">
		<Card.Header class="pb-3">
			<Card.Title class="text-xl">Rechts</Card.Title>
			<Card.Description>Platzhalter</Card.Description>
		</Card.Header>
		<Card.Content class="flex-1 flex items-center justify-center text-muted-foreground">
			<div class="flex flex-col items-center gap-3 opacity-40">
				<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
					<rect width="18" height="18" x="3" y="3" rx="2"/>
					<path d="M3 9h18M9 21V9"/>
				</svg>
				<span class="text-sm">Noch nichts hier</span>
			</div>
		</Card.Content>
	</Card.Root>
</div>

<canvas bind:this={canvasEl} class="hidden"></canvas>
