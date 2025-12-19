<script lang="ts">
	import { onMount } from 'svelte';
	import { commands } from '$lib/bindings';
	import type { HeaderInfo, PlayersWindow } from '$lib/bindings';

	let headerInfo = $state<HeaderInfo | null>(null);
	let dpsData = $state<PlayersWindow | null>(null);
	let dpsHistory = $state<Array<{ timestamp: number; totalDps: number; totalDmg: number }>>([]);
	let updateInterval: ReturnType<typeof setInterval> | undefined;
	const MAX_HISTORY = 60; // Keep 60 data points

	async function updateData() {
		try {
			const headerResult = await commands.getHeaderInfo();
			if (headerResult.status === 'ok') {
				headerInfo = headerResult.data;
				
				// Add to history
				dpsHistory = [...dpsHistory, {
					timestamp: Date.now(),
					totalDps: headerInfo.totalDps,
					totalDmg: headerInfo.totalDmg
				}].slice(-MAX_HISTORY);
			}
			
			dpsData = await commands.getDpsPlayerWindow();
		} catch (error) {
			console.error('Failed to fetch data:', error);
		}
	}

	onMount(() => {
		updateData();
		updateInterval = setInterval(updateData, 1000);
		
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

	const maxDps = $derived(dpsHistory.reduce((max, d) => Math.max(max, d.totalDps), 1));
	const chartHeight = $derived(200);
	const chartWidth = $derived(800);
	
	function getChartPath(data: typeof dpsHistory): string {
		if (data.length < 2) return '';
		
		const points = data.map((d, i) => {
			const x = (i / (MAX_HISTORY - 1)) * chartWidth;
			const y = chartHeight - (d.totalDps / maxDps) * chartHeight;
			return `${x},${y}`;
		});
		
		return `M ${points.join(' L ')}`;
	}

	async function clearData() {
		try {
			dpsHistory = [];
			await commands.resetEncounter();
		} catch (error) {
			console.error('Failed to reset encounter:', error);
			// Even if reset fails, we've cleared the UI data
		}
	}
</script>

<div class="charts-container p-6 space-y-6">
	<div class="header-section">
		<h1 class="text-3xl font-bold mb-4">📈 Live DPS Charts</h1>
		<p class="text-gray-600 mb-6">Real-time damage per second visualization and analysis</p>
		
		<div class="action-buttons flex gap-4 mb-6">
			<button 
				onclick={clearData}
				class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition"
			>
				🔄 Reset Encounter
			</button>
		</div>
	</div>

	{#if headerInfo && dpsHistory.length > 0}
		<div class="chart-section bg-white rounded-lg shadow-lg p-6">
			<h2 class="text-2xl font-bold mb-4">DPS Over Time</h2>
			<div class="chart-container relative bg-gray-50 rounded border border-gray-200 p-4">
				<svg viewBox="0 0 {chartWidth} {chartHeight}" class="w-full h-auto">
					<!-- Grid lines -->
					<defs>
						<pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
							<path d="M 40 0 L 0 0 0 40" fill="none" stroke="#e5e7eb" stroke-width="1"/>
						</pattern>
					</defs>
					<rect width={chartWidth} height={chartHeight} fill="url(#grid)" />
					
					<!-- Chart line -->
					<path
						d={getChartPath(dpsHistory)}
						fill="none"
						stroke="url(#gradient)"
						stroke-width="3"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
					
					<!-- Gradient -->
					<defs>
						<linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="0%">
							<stop offset="0%" style="stop-color:#3b82f6;stop-opacity:1" />
							<stop offset="100%" style="stop-color:#8b5cf6;stop-opacity:1" />
						</linearGradient>
					</defs>
					
					<!-- Data points -->
					{#each dpsHistory as point, i}
						<circle
							cx={(i / (MAX_HISTORY - 1)) * chartWidth}
							cy={chartHeight - (point.totalDps / maxDps) * chartHeight}
							r="3"
							fill="#3b82f6"
						/>
					{/each}
				</svg>
				
				<div class="chart-labels flex justify-between text-sm text-gray-600 mt-2">
					<span>Start</span>
					<span class="font-semibold">Current DPS: {formatNumber(headerInfo.totalDps)}</span>
					<span>Now</span>
				</div>
			</div>
		</div>

		<!-- Player comparison bars -->
		{#if dpsData && dpsData.playerRows.length > 0}
			<div class="comparison-section bg-white rounded-lg shadow-lg p-6">
				<h2 class="text-2xl font-bold mb-4">Player DPS Comparison</h2>
				<div class="bars-container space-y-3">
					{#each dpsData.playerRows.slice(0, 10) as player, index}
						{@const topPlayerDps = dpsData.playerRows[0]?.valuePerSec || 1}
						<div class="player-bar">
							<div class="flex justify-between items-center mb-1">
								<div class="flex items-center gap-2">
									<span class="rank-badge w-6 h-6 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 text-white text-xs flex items-center justify-center font-bold">
										{index + 1}
									</span>
									<span class="font-medium">{player.name}</span>
									<span class="text-xs text-gray-500">{player.className}</span>
								</div>
								<span class="text-sm font-semibold">{formatNumber(player.valuePerSec)}/s</span>
							</div>
							<div class="progress-bar w-full h-8 bg-gray-200 rounded-lg overflow-hidden relative">
								<div 
									class="h-full bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 transition-all duration-500 flex items-center justify-end pr-3"
									style="width: {(player.valuePerSec / topPlayerDps) * 100}%"
								>
									<span class="text-white text-xs font-semibold">
										{formatNumber(player.totalValue)} total
									</span>
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Stats summary -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<div class="stat-box bg-gradient-to-br from-blue-500 to-blue-600 text-white p-6 rounded-lg shadow">
				<div class="text-sm opacity-90">Peak DPS</div>
				<div class="text-2xl font-bold">{formatNumber(maxDps)}</div>
			</div>
			<div class="stat-box bg-gradient-to-br from-purple-500 to-purple-600 text-white p-6 rounded-lg shadow">
				<div class="text-sm opacity-90">Average DPS</div>
				<div class="text-2xl font-bold">
					{formatNumber(dpsHistory.reduce((sum, d) => sum + d.totalDps, 0) / dpsHistory.length)}
				</div>
			</div>
			<div class="stat-box bg-gradient-to-br from-pink-500 to-pink-600 text-white p-6 rounded-lg shadow">
				<div class="text-sm opacity-90">Total Damage</div>
				<div class="text-2xl font-bold">{formatNumber(headerInfo.totalDmg)}</div>
			</div>
		</div>
	{:else}
		<div class="empty-state text-center py-16 bg-gray-50 rounded-lg">
			<div class="text-6xl mb-4">📊</div>
			<h2 class="text-2xl font-bold mb-2">No Data Yet</h2>
			<p class="text-gray-600">Start combat to see real-time DPS charts and analytics</p>
		</div>
	{/if}
</div>

<style>
	.charts-container {
		max-width: 1400px;
		margin: 0 auto;
	}
	
	.stat-box {
		transition: transform 0.2s;
	}
	
	.stat-box:hover {
		transform: translateY(-2px);
	}
	
	.progress-bar {
		box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
	}
</style>
