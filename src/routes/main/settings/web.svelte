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
	let darkMode = $state(true);

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
				darkMode = settings.darkMode ?? true;
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
			darkMode
		};
		localStorage.setItem('bpsr-web-settings', JSON.stringify(settings));
	}

	function handleAutoRefreshChange(value: boolean) {
		autoRefresh = value;
		saveSettings();
	}

	function handleCompactModeChange(value: boolean) {
		compactMode = value;
		saveSettings();
		// Apply compact mode class to body
		if (compactMode) {
			document.body.classList.add('compact-mode');
		} else {
			document.body.classList.remove('compact-mode');
		}
	}

	function handleAnimationsChange(value: boolean) {
		showAnimations = value;
		saveSettings();
		// Toggle animations
		if (!showAnimations) {
			document.body.classList.add('no-animations');
		} else {
			document.body.classList.remove('no-animations');
		}
	}

	function handleIntervalChange(event: Event) {
		const target = event.target as HTMLInputElement;
		refreshInterval = parseInt(target.value);
		saveSettings();
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
					checked={autoRefresh}
					onchange={handleAutoRefreshChange}
				/>
			</div>

			{#if autoRefresh}
				<div class="setting-item ml-6 p-4 bg-muted/30 rounded-lg">
					<label class="block mb-2">
						<span class="text-sm font-medium">Refresh Interval</span>
						<span class="text-xs text-muted-foreground ml-2">{refreshInterval}ms</span>
					</label>
					<input
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
					checked={compactMode}
					onchange={handleCompactModeChange}
				/>
			</div>

			<div class="setting-item">
				<SettingsSwitch
					label="Show Animations"
					description="Enable smooth transitions and animations"
					checked={showAnimations}
					onchange={handleAnimationsChange}
				/>
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
