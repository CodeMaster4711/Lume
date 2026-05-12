<script lang="ts">
	import * as Card from "$lib/components/ui/card/index.js";
	import { Button } from "$lib/components/ui/button/index.js";
	import { createWorker } from "tesseract.js";

	type State = "idle" | "preview" | "ocr" | "done" | "error";

	let state: State = $state("idle");
	let screenshot: string | null = $state(null);
	let ocrText: string = $state("");
	let ocrProgress: number = $state(0);
	let copied = $state(false);
	let errorMsg = $state("");
	let fileInput: HTMLInputElement;

	const isTauri = typeof window !== "undefined" && "__TAURI__" in window;

	async function takeScreenshot() {
		if (isTauri) {
			await tauriScreenshot();
		} else {
			await browserScreenshot();
		}
	}

	async function tauriScreenshot() {
		try {
			const { invoke } = await import("@tauri-apps/api/core");
			const b64: string = await invoke("take_screenshot");
			screenshot = `data:image/png;base64,${b64}`;
			state = "preview";
		} catch (e: unknown) {
			const msg = e instanceof Error ? e.message : String(e);
			console.error("take_screenshot error:", e);
			if (msg.toLowerCase().includes("cancel")) return;
			errorMsg = msg;
			state = "error";
		}
	}

	async function browserScreenshot() {
		try {
			const stream = await navigator.mediaDevices.getDisplayMedia({ video: true, audio: false });
			const track = stream.getVideoTracks()[0];
			const capture = new ImageCapture(track);
			const bitmap = await capture.grabFrame();
			track.stop();
			const canvas = document.createElement("canvas");
			canvas.width = bitmap.width;
			canvas.height = bitmap.height;
			canvas.getContext("2d")!.drawImage(bitmap, 0, 0);
			screenshot = canvas.toDataURL("image/png");
			state = "preview";
		} catch {
			// user cancelled
		}
	}

	function openFilePicker() {
		fileInput.click();
	}

	function onFileChange(e: Event) {
		const file = (e.target as HTMLInputElement).files?.[0];
		if (!file) return;
		const reader = new FileReader();
		reader.onload = () => {
			screenshot = reader.result as string;
			state = "preview";
		};
		reader.readAsDataURL(file);
		// reset so same file can be picked again
		(e.target as HTMLInputElement).value = "";
	}

	async function runOcr() {
		if (!screenshot) return;
		state = "ocr";
		ocrProgress = 0;

		const worker = await createWorker("deu+eng", 1, {
			logger: (m: { status: string; progress: number }) => {
				if (m.status === "recognizing text") {
					ocrProgress = Math.round(m.progress * 100);
				}
			},
		});

		const result = await worker.recognize(screenshot);
		ocrText = result.data.text.trim();
		await worker.terminate();
		state = "done";
	}

	function reset() {
		state = "idle";
		screenshot = null;
		ocrText = "";
		ocrProgress = 0;
		copied = false;
		errorMsg = "";
	}

	async function copyText() {
		await navigator.clipboard.writeText(ocrText);
		copied = true;
		setTimeout(() => (copied = false), 2000);
	}
</script>

<!-- hidden file input -->
<input
	bind:this={fileInput}
	type="file"
	accept="image/*"
	class="hidden"
	onchange={onFileChange}
/>

<div class="flex h-screen items-center justify-center gap-8 p-12 bg-background">

	<!-- Left Card: Screenshot + OCR -->
	<Card.Root class="flex-1 max-w-xl h-[520px] flex flex-col shadow-lg">
		<Card.Header class="pb-3">
			<Card.Title class="flex items-center gap-2 text-xl">
				<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/>
					<circle cx="12" cy="13" r="3"/>
				</svg>
				Screenshot → Text
			</Card.Title>
			<Card.Description>Bildschirm aufnehmen · Bild hochladen · OCR</Card.Description>
		</Card.Header>

		<Card.Content class="flex-1 flex flex-col items-center justify-center gap-4 overflow-hidden">

			{#if state === "idle"}
				<div class="flex flex-col items-center gap-6 w-full">
					<svg xmlns="http://www.w3.org/2000/svg" width="72" height="72" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" class="opacity-25 text-foreground">
						<path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/>
						<circle cx="12" cy="13" r="3"/>
					</svg>
					<div class="flex flex-col gap-3 w-full max-w-xs">
						<Button size="lg" onclick={takeScreenshot} class="w-full">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2">
								<path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/>
								<circle cx="12" cy="13" r="3"/>
							</svg>
							{isTauri ? "Bildschirm auswählen" : "Bildschirm aufnehmen"}
						</Button>
						<Button size="lg" variant="outline" onclick={openFilePicker} class="w-full">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2">
								<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
								<polyline points="14 2 14 8 20 8"/>
								<line x1="12" y1="18" x2="12" y2="12"/>
								<line x1="9" y1="15" x2="15" y2="15"/>
							</svg>
							Bild hochladen
						</Button>
					</div>
				</div>

			{:else if state === "preview"}
				<img src={screenshot} alt="Screenshot" class="rounded-lg object-contain max-h-64 w-full border border-border" />
				<div class="flex gap-3">
					<Button size="lg" onclick={runOcr}>
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2">
							<path d="M3 7V5a2 2 0 0 1 2-2h2"/>
							<path d="M17 3h2a2 2 0 0 1 2 2v2"/>
							<path d="M21 17v2a2 2 0 0 1-2 2h-2"/>
							<path d="M7 21H5a2 2 0 0 1-2-2v-2"/>
						</svg>
						Text extrahieren
					</Button>
					<Button size="lg" variant="outline" onclick={reset}>Verwerfen</Button>
				</div>

			{:else if state === "ocr"}
				<div class="flex flex-col items-center gap-6 w-full">
					<img src={screenshot} alt="Screenshot" class="rounded-lg object-contain max-h-48 w-full border border-border opacity-60" />
					<div class="w-full flex flex-col items-center gap-3">
						<div class="flex items-center gap-2 text-muted-foreground text-sm">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="animate-spin">
								<path d="M21 12a9 9 0 1 1-6.219-8.56"/>
							</svg>
							OCR läuft… {ocrProgress}%
						</div>
						<div class="w-full h-2 bg-secondary rounded-full overflow-hidden">
							<div
								class="h-full bg-primary rounded-full transition-all duration-300"
								style="width: {ocrProgress}%"
							></div>
						</div>
					</div>
				</div>

			{:else if state === "done"}
				<div class="flex flex-col items-center gap-4">
					<img src={screenshot} alt="Screenshot" class="rounded-lg object-contain max-h-48 w-full border border-border" />
					<Button size="lg" variant="outline" onclick={reset}>Neu</Button>
				</div>

			{:else if state === "error"}
				<div class="flex flex-col items-center gap-4 text-destructive text-sm text-center">
					<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="opacity-60">
						<circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
					</svg>
					<p class="text-muted-foreground">{errorMsg}</p>
					<Button size="sm" variant="outline" onclick={reset}>Zurück</Button>
				</div>
			{/if}

		</Card.Content>
	</Card.Root>

	<!-- Right Card: OCR Result -->
	<Card.Root class="flex-1 max-w-xl h-[520px] flex flex-col shadow-lg">
		<Card.Header class="pb-3">
			<Card.Title class="flex items-center gap-2 text-xl">
				<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
					<polyline points="14 2 14 8 20 8"/>
					<line x1="8" y1="13" x2="16" y2="13"/>
					<line x1="8" y1="17" x2="16" y2="17"/>
					<polyline points="10 9 9 9 8 9"/>
				</svg>
				Erkannter Text
			</Card.Title>
			{#if ocrText}
				<Card.Description>{ocrText.length} Zeichen</Card.Description>
			{:else}
				<Card.Description>Text erscheint hier nach OCR</Card.Description>
			{/if}
		</Card.Header>
		<Card.Content class="flex-1 flex flex-col gap-3 overflow-hidden">
			{#if ocrText}
				<div class="flex justify-end">
					<Button size="sm" variant="outline" onclick={copyText}>
						{#if copied}
							<svg xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-1"><path d="M20 6 9 17l-5-5"/></svg>
							Kopiert
						{:else}
							<svg xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-1"><rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>
							Kopieren
						{/if}
					</Button>
				</div>
				<textarea
					readonly
					class="flex-1 w-full min-h-0 resize-none rounded-md border border-border bg-muted/50 p-3 text-sm font-mono text-foreground focus:outline-none"
					value={ocrText}
				></textarea>
			{:else}
				<div class="flex-1 flex flex-col items-center justify-center gap-3 opacity-30 text-foreground">
					<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
						<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
						<polyline points="14 2 14 8 20 8"/>
					</svg>
					<span class="text-sm">Noch kein Text erkannt</span>
				</div>
			{/if}
		</Card.Content>
	</Card.Root>

</div>
