<script lang="ts">
	import { onMount } from 'svelte';
	import { api } from '$lib/api';
	import type { HeaderInfo, PlayersWindow } from '$lib/bindings';

	let headerInfo = $state<HeaderInfo | null>(null);
	let dpsData = $state<PlayersWindow | null>(null);
	let dpsHistory = $state<Array<{ timestamp: number; totalDps: number; totalDmg: number }>>([]);
	let updateInterval: ReturnType<typeof setInterval> | undefined;
	const MAX_HISTORY = 60; // Keep 60 data points

	async function updateData() {
		try {
			const headerResult = await api.getHeaderInfo();
			if (headerResult.status === 'ok') {
				headerInfo = headerResult.data;
				
				// Add to history
				dpsHistory = [...dpsHistory, {
					timestamp: Date.now(),
					totalDps: headerInfo.totalDps,
					totalDmg: headerInfo.totalDmg
				}].slice(-MAX_HISTORY);
			}
			
			dpsData = await api.getDpsPlayerWindow();
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
			await api.resetEncounter();
		} catch (error) {
			console.error('Failed to reset encounter:', error);
			// Even if reset fails, we've cleared the UI data
		}
	}
</script>

<div class="charts-container p-6 space-y-6 animate-in fade-in duration-300">
	<div class="header-section">
		<h1 class="text-3xl font-bold mb-2 bg-gradient-to-r from-primary to-chart-2 bg-clip-text text-transparent">📈 Live DPS Charts</h1>
		<p class="text-muted-foreground mb-6">Real-time damage per second visualization and analysis</p>
		
		<div class="action-buttons flex flex-wrap gap-3 mb-6">
			<button 
				onclick={clearData}
				class="flex items-center gap-2 px-4 py-2 bg-destructive text-destructive-foreground rounded-lg hover:bg-destructive/90 transition-all hover:scale-105 shadow-md"
			>
				<span>🔄</span>
				<span>Reset Encounter</span>
			</button>
		</div>
	</div>

	{#if headerInfo && dpsHistory.length > 0}
		<div class="chart-section bg-card rounded-xl shadow-xl border border-border p-6 animate-in slide-in-from-bottom-4 duration-500">
			<h2 class="text-2xl font-bold mb-4">DPS Over Time</h2>
			<div class="chart-container relative bg-muted/30 rounded-lg border border-border p-6 backdrop-blur-sm">
				<svg viewBox="0 0 {chartWidth} {chartHeight}" class="w-full h-auto">
					<!-- Grid lines -->
					<defs>
						<pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
							<path d="M 40 0 L 0 0 0 40" fill="none" stroke="currentColor" stroke-width="0.5" opacity="0.1"/>
						</pattern>
					</defs>
					<rect width={chartWidth} height={chartHeight} fill="url(#grid)" />
					
					<!-- Chart line with glow -->
					<defs>
						<filter id="glow">
							<feGaussianBlur stdDeviation="2" result="coloredBlur"/>
							<feMerge>
								<feMergeNode in="coloredBlur"/>
								<feMergeNode in="SourceGraphic"/>
							</feMerge>
						</filter>
					</defs>
					
					<path
						d={getChartPath(dpsHistory)}
						fill="none"
						stroke="url(#gradient)"
						stroke-width="4"
						stroke-linecap="round"
						stroke-linejoin="round"
						filter="url(#glow)"
					/>
					
					<!-- Gradient -->
					<defs>
						<linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="0%">
							<stop offset="0%" style="stop-color:oklch(var(--chart-1));stop-opacity:1" />
							<stop offset="50%" style="stop-color:oklch(var(--chart-2));stop-opacity:1" />
							<stop offset="100%" style="stop-color:oklch(var(--chart-3));stop-opacity:1" />
						</linearGradient>
					</defs>
					
					<!-- Data points -->
					{#each dpsHistory as point, i}
						<circle
							cx={(i / (MAX_HISTORY - 1)) * chartWidth}
							cy={chartHeight - (point.totalDps / maxDps) * chartHeight}
							r="4"
							fill="oklch(var(--chart-1))"
							class="animate-in fade-in duration-200"
							style="animation-delay: {i * 20}ms"
						/>
					{/each}
				</svg>
				
				<div class="chart-labels flex justify-between text-sm text-muted-foreground mt-4">
					<span class="font-medium">Start</span>
					<span class="font-bold text-foreground bg-accent/50 px-3 py-1 rounded-full">Current DPS: {formatNumber(headerInfo.totalDps)}</span>
					<span class="font-medium">Now</span>
				</div>
			</div>
		</div>

		<!-- Player comparison bars -->
		{#if dpsData && dpsData.playerRows.length > 0}
			<div class="comparison-section bg-card rounded-xl shadow-xl border border-border p-6 animate-in slide-in-from-bottom-6 duration-700">
				<h2 class="text-2xl font-bold mb-4">Player DPS Comparison</h2>
				<div class="bars-container space-y-4">
					{#each dpsData.playerRows.slice(0, 10) as player, index}
						{@const topPlayerDps = dpsData.playerRows[0]?.valuePerSec || 1}
						<div class="player-bar group">
							<div class="flex justify-between items-center mb-2">
								<div class="flex items-center gap-3">
									<span class="rank-badge w-9 h-9 rounded-full bg-gradient-to-br from-chart-1 via-chart-2 to-chart-3 text-white text-sm flex items-center justify-center font-bold shadow-lg group-hover:scale-110 transition-transform">
										{index + 1}
									</span>
									<span class="font-semibold text-base">{player.name}</span>
									<span class="text-sm text-muted-foreground px-2 py-1 bg-muted rounded-md">{player.className}</span>
								</div>
								<span class="text-sm font-bold text-chart-1">{formatNumber(player.valuePerSec)}/s</span>
							</div>
							<div class="progress-bar w-full h-10 bg-muted/50 rounded-xl overflow-hidden relative border border-border shadow-inner group-hover:shadow-md transition-all">
								<div 
									class="h-full bg-gradient-to-r from-chart-1 via-chart-2 to-chart-3 transition-all duration-700 ease-out flex items-center justify-end pr-4 relative overflow-hidden"
									style="width: {(player.valuePerSec / topPlayerDps) * 100}%"
								>
									<div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/10 to-transparent animate-shimmer"></div>
									<span class="text-white text-sm font-bold drop-shadow-lg relative z-10">
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
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4 animate-in slide-in-from-bottom-8 duration-900">
			<div class="stat-box group bg-gradient-to-br from-chart-1 to-chart-2 text-white p-6 rounded-xl shadow-lg hover:shadow-2xl transition-all duration-300">
				<div class="text-sm opacity-90 font-medium">Peak DPS</div>
				<div class="text-3xl font-bold mt-2 group-hover:scale-110 transition-transform">{formatNumber(maxDps)}</div>
			</div>
			<div class="stat-box group bg-gradient-to-br from-chart-2 to-chart-3 text-white p-6 rounded-xl shadow-lg hover:shadow-2xl transition-all duration-300">
				<div class="text-sm opacity-90 font-medium">Average DPS</div>
				<div class="text-3xl font-bold mt-2 group-hover:scale-110 transition-transform">
					{formatNumber(dpsHistory.reduce((sum, d) => sum + d.totalDps, 0) / dpsHistory.length)}
				</div>
			</div>
			<div class="stat-box group bg-gradient-to-br from-chart-3 to-chart-4 text-white p-6 rounded-xl shadow-lg hover:shadow-2xl transition-all duration-300">
				<div class="text-sm opacity-90 font-medium">Total Damage</div>
				<div class="text-3xl font-bold mt-2 group-hover:scale-110 transition-transform">{formatNumber(headerInfo.totalDmg)}</div>
			</div>
		</div>
	{:else}
		<div class="empty-state text-center py-20 bg-card rounded-xl border border-border shadow-lg">
			<div class="text-7xl mb-4 animate-bounce">📊</div>
			<h2 class="text-2xl font-bold mb-2">No Data Yet</h2>
			<p class="text-muted-foreground">Start combat to see real-time DPS charts and analytics</p>
		</div>
	{/if}
</div>

<style>
	.charts-container {
		max-width: 1400px;
		margin: 0 auto;
	}
	
	.stat-box {
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}
	
	.stat-box:hover {
		transform: translateY(-4px) scale(1.02);
	}
	
	.progress-bar {
		box-shadow: inset 0 2px 6px rgba(0, 0, 0, 0.15);
	}
	
	@keyframes shimmer {
		0% {
			transform: translateX(-100%);
		}
		100% {
			transform: translateX(100%);
		}
	}
	
	.animate-shimmer {
		animation: shimmer 3s infinite;
	}
</style>
