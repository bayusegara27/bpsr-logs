<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import SettingsSwitch from './settings-switch.svelte';
	import { api } from '$lib/api';
	import { onMount } from 'svelte';

	let isWebMode = $state(false);
	let autoRefresh = $state(true);
	let refreshInterval = $state(1000);
	let compactMode = $state(false);
	let showAnimations = $state(true);
	
	// Port configuration (informational - actual ports are set in Rust)
	let preferredStaticPort = $state(1420);
	let preferredApiPort = $state(3000);
	let showPortInfo = $state(false);

	onMount(() => {
		// Detect if running in web browser
		isWebMode = !api.isRunningInTauri();
		
		// Load settings from localStorage
		const savedSettings = localStorage.getItem('bpsr-web-settings');
		if (savedSettings) {
			try {
				const settings = JSON.parse(savedSettings);
				autoRefresh = settings.autoRefresh ?? true;
				refreshInterval = settings.refreshInterval ?? 1000;
				compactMode = settings.compactMode ?? false;
				showAnimations = settings.showAnimations ?? true;
				preferredStaticPort = settings.preferredStaticPort ?? 1420;
				preferredApiPort = settings.preferredApiPort ?? 3000;
			} catch (e) {
				console.error('Failed to load web settings:', e);
			}
		}
	});

	function saveSettings() {
		const settings = {
			autoRefresh,
			refreshInterval,
			compactMode,
			showAnimations,
			preferredStaticPort,
			preferredApiPort
		};
		localStorage.setItem('bpsr-web-settings', JSON.stringify(settings));
	}

	// Watch for changes and save
	$effect(() => {
		saveSettings();
		
		// Apply compact mode class to body
		if (compactMode) {
			document.body.classList.add('compact-mode');
		} else {
			document.body.classList.remove('compact-mode');
		}
		
		// Toggle animations
		if (!showAnimations) {
			document.body.classList.add('no-animations');
		} else {
			document.body.classList.remove('no-animations');
		}
	});

	function handleStaticPortChange(event: Event) {
		const target = event.target as HTMLInputElement;
		preferredStaticPort = parseInt(target.value);
	}

	function handleApiPortChange(event: Event) {
		const target = event.target as HTMLInputElement;
		preferredApiPort = parseInt(target.value);
	}

	function handleIntervalChange(event: Event) {
		const target = event.target as HTMLInputElement;
		refreshInterval = parseInt(target.value);
	}
</script>

<Tabs.Content value="web">
	<div class="space-y-6 p-6 bg-card rounded-xl border border-border">
		<div class="header-section mb-6">
			<h2 class="text-2xl font-bold mb-2">🌐 Web Settings</h2>
			<p class="text-sm text-muted-foreground">
				Configure settings specific to web browser mode
			</p>
			{#if isWebMode}
				<div class="mt-3 px-3 py-2 bg-chart-2/10 border border-chart-2/30 rounded-lg inline-flex items-center gap-2">
					<span class="text-chart-2">✓</span>
					<span class="text-sm font-medium">Running in Web Browser Mode</span>
				</div>
			{/if}
		</div>

		<div class="settings-group space-y-4">
			<div class="setting-item">
				<SettingsSwitch
					label="Auto Refresh"
					description="Automatically refresh statistics in real-time"
					bind:checked={autoRefresh}
				/>
			</div>

			{#if autoRefresh}
				<div class="setting-item ml-6 p-4 bg-muted/30 rounded-lg">
					<label for="refresh-interval" class="block mb-2">
						<span class="text-sm font-medium">Refresh Interval</span>
						<span class="text-xs text-muted-foreground ml-2">{refreshInterval}ms</span>
					</label>
					<input
						id="refresh-interval"
						type="range"
						min="500"
						max="5000"
						step="100"
						value={refreshInterval}
						oninput={handleIntervalChange}
						class="w-full h-2 bg-muted rounded-lg appearance-none cursor-pointer accent-primary"
					/>
					<div class="flex justify-between text-xs text-muted-foreground mt-1">
						<span>Fast (500ms)</span>
						<span>Slow (5000ms)</span>
					</div>
				</div>
			{/if}

			<div class="setting-item">
				<SettingsSwitch
					label="Compact Mode"
					description="Reduce padding and spacing for a more compact interface"
					bind:checked={compactMode}
				/>
			</div>

			<div class="setting-item">
				<SettingsSwitch
					label="Show Animations"
					description="Enable smooth transitions and animations"
					bind:checked={showAnimations}
				/>
			</div>

			<div class="divider my-6 border-t border-border"></div>

			<div class="port-configuration-section">
				<h3 class="text-lg font-semibold mb-3 flex items-center gap-2">
					<span>🔌</span>
					<span>Port Configuration</span>
				</h3>
				<p class="text-xs text-muted-foreground mb-4">
					Configure preferred ports for web access. Note: These are reference values. 
					The actual ports used are logged when the app starts and may differ if preferred ports are in use.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<div class="port-setting p-4 bg-muted/30 rounded-lg">
						<label for="static-port" class="block mb-2">
							<span class="text-sm font-medium">Static File Server Port (Frontend)</span>
							<span class="text-xs text-muted-foreground ml-2">Default: 1420</span>
						</label>
						<input
							id="static-port"
							type="number"
							min="1024"
							max="65535"
							value={preferredStaticPort}
							oninput={handleStaticPortChange}
							class="w-full px-3 py-2 bg-background border border-border rounded-md text-sm"
						/>
						<p class="text-xs text-muted-foreground mt-1">
							Fallback range: 1420-1430
						</p>
					</div>

					<div class="port-setting p-4 bg-muted/30 rounded-lg">
						<label for="api-port" class="block mb-2">
							<span class="text-sm font-medium">HTTP API Server Port</span>
							<span class="text-xs text-muted-foreground ml-2">Default: 3000</span>
						</label>
						<input
							id="api-port"
							type="number"
							min="1024"
							max="65535"
							value={preferredApiPort}
							oninput={handleApiPortChange}
							class="w-full px-3 py-2 bg-background border border-border rounded-md text-sm"
						/>
						<p class="text-xs text-muted-foreground mt-1">
							Fallback range: 3000-3010
						</p>
					</div>
				</div>

				<button
					onclick={() => showPortInfo = !showPortInfo}
					class="mt-3 text-sm text-primary hover:underline flex items-center gap-1"
				>
					<span>{showPortInfo ? '▼' : '▶'}</span>
					<span>Show Connection Info & Logs</span>
				</button>

				{#if showPortInfo}
					<div class="connection-info mt-4 p-4 bg-accent/20 border border-accent/50 rounded-lg animate-in slide-in-from-top-2 duration-200">
						<h4 class="text-sm font-semibold mb-2">📡 Connection Information</h4>
						<div class="space-y-2 text-xs">
							<div class="info-item">
								<span class="font-medium text-chart-1">Frontend URL:</span>
								<code class="ml-2 px-2 py-1 bg-muted rounded">http://localhost:{preferredStaticPort}</code>
								<p class="text-muted-foreground mt-1 ml-4">Access the web interface here</p>
							</div>
							<div class="info-item">
								<span class="font-medium text-chart-2">API Endpoint:</span>
								<code class="ml-2 px-2 py-1 bg-muted rounded">http://localhost:{preferredApiPort}/api</code>
								<p class="text-muted-foreground mt-1 ml-4">Backend API for data access</p>
							</div>
							<div class="divider my-3 border-t border-border/50"></div>
							<div class="info-item">
								<span class="font-medium">📝 Log Location:</span>
								<p class="text-muted-foreground mt-1">Check application logs to see actual ports used:</p>
								<ul class="list-disc list-inside ml-4 mt-2 space-y-1 text-muted-foreground">
									<li><code class="text-xs bg-muted px-1 rounded">Static file server starting on http://0.0.0.0:XXXX</code></li>
									<li><code class="text-xs bg-muted px-1 rounded">HTTP API server starting on http://0.0.0.0:XXXX</code></li>
								</ul>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<div class="divider my-6 border-t border-border"></div>

			<div class="info-section bg-accent/20 border border-accent/50 rounded-lg p-4">
				<h3 class="text-sm font-semibold mb-2 flex items-center gap-2">
					<span>💡</span>
					<span>Performance Tips</span>
				</h3>
				<ul class="text-xs text-muted-foreground space-y-1">
					<li>• Use a higher refresh interval (slower) to reduce CPU usage</li>
					<li>• Enable compact mode for better viewing on smaller screens</li>
					<li>• Disable animations if experiencing performance issues</li>
					<li>• Settings are saved locally in your browser</li>
				</ul>
			</div>

			<div class="info-section bg-primary/10 border border-primary/30 rounded-lg p-4">
				<h3 class="text-sm font-semibold mb-2 flex items-center gap-2">
					<span>🔗</span>
					<span>Tunnel Access</span>
				</h3>
				<p class="text-xs text-muted-foreground">
					When accessing via cloudflared, ngrok, or localtunnel, all features work through 
					the HTTP API. The desktop app must remain running on port 1420 and 3000.
				</p>
			</div>
		</div>
	</div>
</Tabs.Content>

<style>
	:global(body.compact-mode) {
		--spacing: 0.125rem;
	}
	
	:global(body.no-animations *) {
		animation: none !important;
		transition: none !important;
	}
</style>
