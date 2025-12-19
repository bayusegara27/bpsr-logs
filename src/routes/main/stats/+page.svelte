<script lang="ts">
	import { onMount } from 'svelte';
	import { commands } from '$lib/bindings';
	import { api } from '$lib/api';
	import type { HeaderInfo, PlayersWindow } from '$lib/bindings';
	import { api } from '$lib/api';

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

<div class="stats-container p-6 space-y-6">
	<div class="header-section">
		<h1 class="text-3xl font-bold mb-4">📊 Live Statistics & Dashboard</h1>
		<p class="text-gray-600 mb-6">Real-time combat statistics and performance metrics</p>
		
		<div class="action-buttons flex gap-4 mb-6">
			<button 
				onclick={exportData}
				class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
			>
				💾 Export Data
			</button>
			<button 
				onclick={shareToClipboard}
				class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition relative"
			>
				📋 Share to Clipboard
				{#if copySuccess}
					<span class="absolute -top-8 left-1/2 -translate-x-1/2 bg-green-800 text-white px-3 py-1 rounded text-sm whitespace-nowrap">
						✓ Copied!
					</span>
				{/if}
			</button>
		</div>
	</div>

	{#if headerInfo}
		<div class="stats-grid grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
			<div class="stat-card bg-gradient-to-br from-blue-500 to-blue-600 text-white p-6 rounded-lg shadow-lg">
				<div class="stat-label text-sm opacity-90">Total DPS</div>
				<div class="stat-value text-3xl font-bold">{formatNumber(headerInfo.totalDps)}</div>
				<div class="stat-unit text-xs opacity-75">damage per second</div>
			</div>
			
			<div class="stat-card bg-gradient-to-br from-purple-500 to-purple-600 text-white p-6 rounded-lg shadow-lg">
				<div class="stat-label text-sm opacity-90">Total Damage</div>
				<div class="stat-value text-3xl font-bold">{formatNumber(headerInfo.totalDmg)}</div>
				<div class="stat-unit text-xs opacity-75">total damage dealt</div>
			</div>
			
			<div class="stat-card bg-gradient-to-br from-green-500 to-green-600 text-white p-6 rounded-lg shadow-lg">
				<div class="stat-label text-sm opacity-90">Duration</div>
				<div class="stat-value text-2xl font-bold">{formatTime(headerInfo.elapsedMs)}</div>
				<div class="stat-unit text-xs opacity-75">combat time</div>
			</div>
			
			<div class="stat-card bg-gradient-to-br from-orange-500 to-orange-600 text-white p-6 rounded-lg shadow-lg">
				<div class="stat-label text-sm opacity-90">Active Players</div>
				<div class="stat-value text-3xl font-bold">{dpsData?.playerRows.length || 0}</div>
				<div class="stat-unit text-xs opacity-75">in combat</div>
			</div>
		</div>
	{:else}
		<div class="loading-state text-center py-12">
			<div class="text-xl">⏳ Loading statistics...</div>
			<p class="text-gray-600 mt-2">Waiting for combat data</p>
		</div>
	{/if}

	{#if dpsData && dpsData.playerRows.length > 0}
		<div class="dps-section bg-white rounded-lg shadow-lg p-6">
			<h2 class="text-2xl font-bold mb-4">💥 DPS Rankings</h2>
			<div class="table-responsive overflow-x-auto">
				<table class="w-full">
					<thead class="bg-gray-100">
						<tr>
							<th class="px-4 py-2 text-left">Rank</th>
							<th class="px-4 py-2 text-left">Player</th>
							<th class="px-4 py-2 text-left">Class</th>
							<th class="px-4 py-2 text-right">Total Damage</th>
							<th class="px-4 py-2 text-right">DPS</th>
							<th class="px-4 py-2 text-right">Crit %</th>
							<th class="px-4 py-2 text-right">Contribution</th>
						</tr>
					</thead>
					<tbody>
						{#each dpsData.playerRows as player, index}
							<tr class="border-b hover:bg-gray-50 transition">
								<td class="px-4 py-3 font-semibold">#{index + 1}</td>
								<td class="px-4 py-3 font-medium">{player.name}</td>
								<td class="px-4 py-3">
									<span class="px-2 py-1 bg-blue-100 text-blue-800 rounded text-sm">
										{player.className}
									</span>
								</td>
								<td class="px-4 py-3 text-right">{formatNumber(player.totalValue)}</td>
								<td class="px-4 py-3 text-right font-semibold">{formatNumber(player.valuePerSec)}/s</td>
								<td class="px-4 py-3 text-right">{(player.critRate * 100).toFixed(1)}%</td>
								<td class="px-4 py-3 text-right">
									<div class="flex items-center justify-end gap-2">
										<div class="w-24 h-2 bg-gray-200 rounded-full overflow-hidden">
											<div 
												class="h-full bg-gradient-to-r from-blue-500 to-purple-600"
												style="width: {player.valuePct * 100}%"
											></div>
										</div>
										<span class="text-sm">{(player.valuePct * 100).toFixed(1)}%</span>
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
		<div class="heal-section bg-white rounded-lg shadow-lg p-6">
			<h2 class="text-2xl font-bold mb-4">💚 Healing Rankings</h2>
			<div class="table-responsive overflow-x-auto">
				<table class="w-full">
					<thead class="bg-gray-100">
						<tr>
							<th class="px-4 py-2 text-left">Rank</th>
							<th class="px-4 py-2 text-left">Player</th>
							<th class="px-4 py-2 text-left">Class</th>
							<th class="px-4 py-2 text-right">Total Healing</th>
							<th class="px-4 py-2 text-right">HPS</th>
							<th class="px-4 py-2 text-right">Contribution</th>
						</tr>
					</thead>
					<tbody>
						{#each healData.playerRows as player, index}
							<tr class="border-b hover:bg-gray-50 transition">
								<td class="px-4 py-3 font-semibold">#{index + 1}</td>
								<td class="px-4 py-3 font-medium">{player.name}</td>
								<td class="px-4 py-3">
									<span class="px-2 py-1 bg-green-100 text-green-800 rounded text-sm">
										{player.className}
									</span>
								</td>
								<td class="px-4 py-3 text-right">{formatNumber(player.totalValue)}</td>
								<td class="px-4 py-3 text-right font-semibold">{formatNumber(player.valuePerSec)}/s</td>
								<td class="px-4 py-3 text-right">
									<div class="flex items-center justify-end gap-2">
										<div class="w-24 h-2 bg-gray-200 rounded-full overflow-hidden">
											<div 
												class="h-full bg-gradient-to-r from-green-400 to-green-600"
												style="width: {player.valuePct * 100}%"
											></div>
										</div>
										<span class="text-sm">{(player.valuePct * 100).toFixed(1)}%</span>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{/if}
	
	<div class="info-section bg-blue-50 border border-blue-200 rounded-lg p-4">
		<h3 class="text-lg font-semibold mb-2">ℹ️ How to Share</h3>
		<p class="text-gray-700">
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
		transition: transform 0.2s;
	}
	
	.stat-card:hover {
		transform: translateY(-2px);
	}
</style>
