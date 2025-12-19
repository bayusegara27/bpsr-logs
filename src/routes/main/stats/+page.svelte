<script lang="ts">
	import { onMount } from 'svelte';
	import { api } from '$lib/api';
	import type { HeaderInfo, PlayersWindow } from '$lib/bindings';

	let headerInfo = $state<HeaderInfo | null>(null);
	let dpsData = $state<PlayersWindow | null>(null);
	let healData = $state<PlayersWindow | null>(null);
	let updateInterval: ReturnType<typeof setInterval> | undefined;

	async function updateStats() {
		try {
			const headerResult = await api.getHeaderInfo();
			if (headerResult.status === 'ok') {
				headerInfo = headerResult.data;
			}
			
			dpsData = await api.getDpsPlayerWindow();
			healData = await api.getHealPlayerWindow();
		} catch (error) {
			console.error('Failed to fetch stats:', error);
		}
	}

	onMount(() => {
		updateStats();
		updateInterval = setInterval(updateStats, 1000);
		
		return () => {
			if (updateInterval) {
				clearInterval(updateInterval);
			}
		};
	});

	function formatNumber(num: number): string {
		if (num >= 1_000_000_000) {
			return (num / 1_000_000_000).toFixed(2) + 'B';
		} else if (num >= 1_000_000) {
			return (num / 1_000_000).toFixed(2) + 'M';
		} else if (num >= 1_000) {
			return (num / 1_000).toFixed(2) + 'K';
		}
		return num.toFixed(0);
	}

	function formatTime(ms: number): string {
		const seconds = Math.floor(ms / 1000);
		const minutes = Math.floor(seconds / 60);
		const hours = Math.floor(minutes / 60);
		
		if (hours > 0) {
			return `${hours}h ${minutes % 60}m ${seconds % 60}s`;
		} else if (minutes > 0) {
			return `${minutes}m ${seconds % 60}s`;
		}
		return `${seconds}s`;
	}

	function exportData() {
		const exportObj = {
			timestamp: new Date().toISOString(),
			header: headerInfo,
			dps: dpsData,
			heal: healData
		};
		
		const dataStr = JSON.stringify(exportObj, null, 2);
		const dataBlob = new Blob([dataStr], { type: 'application/json' });
		const url = URL.createObjectURL(dataBlob);
		const link = document.createElement('a');
		link.href = url;
		link.download = `bpsr-stats-${Date.now()}.json`;
		link.click();
		URL.revokeObjectURL(url);
	}

	let copySuccess = $state(false);

	function shareToClipboard() {
		if (!headerInfo || !dpsData) return;
		
		let shareText = `🎮 BPSR Logs Statistics\n\n`;
		shareText += `⏱️ Duration: ${formatTime(headerInfo.elapsedMs)}\n`;
		shareText += `💥 Total Damage: ${formatNumber(headerInfo.totalDmg)}\n`;
		shareText += `📊 Total DPS: ${formatNumber(headerInfo.totalDps)}\n\n`;
		shareText += `👥 Top 5 Players:\n`;
		
		dpsData.playerRows.slice(0, 5).forEach((player, i) => {
			shareText += `${i + 1}. ${player.name} (${player.className}): ${formatNumber(player.totalValue)} @ ${formatNumber(player.valuePerSec)}/s\n`;
		});
		
		navigator.clipboard.writeText(shareText).then(() => {
			copySuccess = true;
			setTimeout(() => { copySuccess = false; }, 2000);
		}).catch((err) => {
			console.error('Failed to copy:', err);
		});
	}
</script>

<div class="stats-container p-6 space-y-6 animate-in fade-in duration-300">
	<div class="header-section">
		<h1 class="text-3xl font-bold mb-2 bg-gradient-to-r from-primary to-chart-2 bg-clip-text text-transparent">📊 Live Statistics Dashboard</h1>
		<p class="text-muted-foreground mb-6">Real-time combat statistics and performance metrics</p>
		
		<div class="action-buttons flex flex-wrap gap-3 mb-6">
			<button 
				onclick={exportData}
				class="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-all hover:scale-105 shadow-md"
			>
				<span>💾</span>
				<span>Export Data</span>
			</button>
			<button 
				onclick={shareToClipboard}
				class="flex items-center gap-2 px-4 py-2 bg-chart-2 text-white rounded-lg hover:bg-chart-2/90 transition-all hover:scale-105 shadow-md relative"
			>
				<span>📋</span>
				<span>Share to Clipboard</span>
				{#if copySuccess}
					<span class="absolute -top-10 left-1/2 -translate-x-1/2 bg-chart-2 text-white px-3 py-2 rounded-lg text-sm whitespace-nowrap shadow-lg animate-in fade-in slide-in-from-bottom-2 duration-200">
						✓ Copied!
					</span>
				{/if}
			</button>
		</div>
	</div>

	{#if headerInfo}
		<div class="stats-grid grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 animate-in slide-in-from-bottom-4 duration-500">
			<div class="stat-card group bg-gradient-to-br from-chart-1 to-chart-2 text-white p-6 rounded-xl shadow-lg hover:shadow-2xl transition-all duration-300">
				<div class="stat-label text-sm opacity-90 font-medium">Total DPS</div>
				<div class="stat-value text-4xl font-bold mt-2 group-hover:scale-110 transition-transform">{formatNumber(headerInfo.totalDps)}</div>
				<div class="stat-unit text-xs opacity-75 mt-1">damage per second</div>
			</div>
			
			<div class="stat-card group bg-gradient-to-br from-chart-2 to-chart-3 text-white p-6 rounded-xl shadow-lg hover:shadow-2xl transition-all duration-300">
				<div class="stat-label text-sm opacity-90 font-medium">Total Damage</div>
				<div class="stat-value text-4xl font-bold mt-2 group-hover:scale-110 transition-transform">{formatNumber(headerInfo.totalDmg)}</div>
				<div class="stat-unit text-xs opacity-75 mt-1">total damage dealt</div>
			</div>
			
			<div class="stat-card group bg-gradient-to-br from-chart-3 to-chart-4 text-white p-6 rounded-xl shadow-lg hover:shadow-2xl transition-all duration-300">
				<div class="stat-label text-sm opacity-90 font-medium">Duration</div>
				<div class="stat-value text-3xl font-bold mt-2 group-hover:scale-110 transition-transform">{formatTime(headerInfo.elapsedMs)}</div>
				<div class="stat-unit text-xs opacity-75 mt-1">combat time</div>
			</div>
			
			<div class="stat-card group bg-gradient-to-br from-chart-4 to-chart-5 text-white p-6 rounded-xl shadow-lg hover:shadow-2xl transition-all duration-300">
				<div class="stat-label text-sm opacity-90 font-medium">Active Players</div>
				<div class="stat-value text-4xl font-bold mt-2 group-hover:scale-110 transition-transform">{dpsData?.playerRows.length || 0}</div>
				<div class="stat-unit text-xs opacity-75 mt-1">in combat</div>
			</div>
		</div>
	{:else}
		<div class="loading-state text-center py-16 bg-card rounded-xl border border-border animate-pulse">
			<div class="text-2xl mb-2">⏳</div>
			<div class="text-xl font-semibold">Loading statistics...</div>
			<p class="text-muted-foreground mt-2">Waiting for combat data</p>
		</div>
	{/if}

	{#if dpsData && dpsData.playerRows.length > 0}
		<div class="dps-section bg-card rounded-xl shadow-xl border border-border p-6 animate-in slide-in-from-bottom-6 duration-700">
			<h2 class="text-2xl font-bold mb-4 flex items-center gap-2">
				<span class="text-3xl">💥</span>
				<span>DPS Rankings</span>
			</h2>
			<div class="table-responsive overflow-x-auto rounded-lg">
				<table class="w-full">
					<thead class="bg-muted/50 border-b border-border">
						<tr>
							<th class="px-4 py-3 text-left font-semibold">Rank</th>
							<th class="px-4 py-3 text-left font-semibold">Player</th>
							<th class="px-4 py-3 text-left font-semibold">Class</th>
							<th class="px-4 py-3 text-right font-semibold">Total Damage</th>
							<th class="px-4 py-3 text-right font-semibold">DPS</th>
							<th class="px-4 py-3 text-right font-semibold">Crit %</th>
							<th class="px-4 py-3 text-right font-semibold">Contribution</th>
						</tr>
					</thead>
					<tbody>
						{#each dpsData.playerRows as player, index}
							<tr class="border-b border-border hover:bg-accent/50 transition-colors">
								<td class="px-4 py-3">
									<span class="inline-flex items-center justify-center w-8 h-8 rounded-full bg-gradient-to-br from-chart-1 to-chart-2 text-white font-bold text-sm">
										{index + 1}
									</span>
								</td>
								<td class="px-4 py-3 font-medium">{player.name}</td>
								<td class="px-4 py-3">
									<span class="px-3 py-1 bg-chart-2/20 text-chart-2 rounded-full text-sm font-medium border border-chart-2/30">
										{player.className}
									</span>
								</td>
								<td class="px-4 py-3 text-right font-semibold">{formatNumber(player.totalValue)}</td>
								<td class="px-4 py-3 text-right font-bold text-chart-1">{formatNumber(player.valuePerSec)}/s</td>
								<td class="px-4 py-3 text-right">{(player.critRate * 100).toFixed(1)}%</td>
								<td class="px-4 py-3 text-right">
									<div class="flex items-center justify-end gap-2">
										<div class="w-24 h-2.5 bg-muted rounded-full overflow-hidden">
											<div 
												class="h-full bg-gradient-to-r from-chart-1 via-chart-2 to-chart-3 transition-all duration-500"
												style="width: {player.valuePct * 100}%"
											></div>
										</div>
										<span class="text-sm font-medium min-w-[3rem]">{(player.valuePct * 100).toFixed(1)}%</span>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{/if}

	{#if healData && healData.playerRows.length > 0}
		<div class="heal-section bg-card rounded-xl shadow-xl border border-border p-6 animate-in slide-in-from-bottom-8 duration-900">
			<h2 class="text-2xl font-bold mb-4 flex items-center gap-2">
				<span class="text-3xl">💚</span>
				<span>Healing Rankings</span>
			</h2>
			<div class="table-responsive overflow-x-auto rounded-lg">
				<table class="w-full">
					<thead class="bg-muted/50 border-b border-border">
						<tr>
							<th class="px-4 py-3 text-left font-semibold">Rank</th>
							<th class="px-4 py-3 text-left font-semibold">Player</th>
							<th class="px-4 py-3 text-left font-semibold">Class</th>
							<th class="px-4 py-3 text-right font-semibold">Total Healing</th>
							<th class="px-4 py-3 text-right font-semibold">HPS</th>
							<th class="px-4 py-3 text-right font-semibold">Contribution</th>
						</tr>
					</thead>
					<tbody>
						{#each healData.playerRows as player, index}
							<tr class="border-b border-border hover:bg-accent/50 transition-colors">
								<td class="px-4 py-3">
									<span class="inline-flex items-center justify-center w-8 h-8 rounded-full bg-gradient-to-br from-chart-3 to-chart-4 text-white font-bold text-sm">
										{index + 1}
									</span>
								</td>
								<td class="px-4 py-3 font-medium">{player.name}</td>
								<td class="px-4 py-3">
									<span class="px-3 py-1 bg-chart-3/20 text-chart-3 rounded-full text-sm font-medium border border-chart-3/30">
										{player.className}
									</span>
								</td>
								<td class="px-4 py-3 text-right font-semibold">{formatNumber(player.totalValue)}</td>
								<td class="px-4 py-3 text-right font-bold text-chart-3">{formatNumber(player.valuePerSec)}/s</td>
								<td class="px-4 py-3 text-right">
									<div class="flex items-center justify-end gap-2">
										<div class="w-24 h-2.5 bg-muted rounded-full overflow-hidden">
											<div 
												class="h-full bg-gradient-to-r from-chart-3 to-chart-4 transition-all duration-500"
												style="width: {player.valuePct * 100}%"
											></div>
										</div>
										<span class="text-sm font-medium min-w-[3rem]">{(player.valuePct * 100).toFixed(1)}%</span>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{/if}
	
	<div class="info-section bg-accent/30 border border-accent rounded-xl p-5 backdrop-blur-sm">
		<h3 class="text-lg font-semibold mb-2 flex items-center gap-2">
			<span>ℹ️</span>
			<span>How to Share</span>
		</h3>
		<p class="text-sm text-muted-foreground leading-relaxed">
			Click "Share to Clipboard" to copy your statistics and share them with others. 
			The data includes your DPS rankings, healing stats, and combat duration.
			You can also export full data as JSON for detailed analysis.
		</p>
	</div>
</div>

<style>
	.stats-container {
		max-width: 1400px;
		margin: 0 auto;
	}
	
	.stat-card {
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}
	
	.stat-card:hover {
		transform: translateY(-4px) scale(1.02);
	}
	
	@keyframes pulse-subtle {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.8;
		}
	}
	
	.loading-state {
		animation: pulse-subtle 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
</style>
