<script lang="ts">
	import { onMount } from 'svelte';

	interface EncounterHistory {
		id: string;
		timestamp: number;
		duration: number;
		totalDps: number;
		totalDmg: number;
		playerCount: number;
		topPlayer: string;
	}

	let history = $state<EncounterHistory[]>([]);
	let selectedEncounter = $state<string | null>(null);

	onMount(() => {
		// Load history from localStorage
		const savedHistory = localStorage.getItem('bpsr-encounter-history');
		if (savedHistory) {
			try {
				history = JSON.parse(savedHistory);
			} catch (e) {
				console.error('Failed to load history:', e);
				history = [];
			}
		}
	});

	function formatDate(timestamp: number): string {
		return new Date(timestamp).toLocaleString('id-ID', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatDuration(ms: number): string {
		const seconds = Math.floor(ms / 1000);
		const minutes = Math.floor(seconds / 60);
		const hours = Math.floor(minutes / 60);
		
		if (hours > 0) {
			return `${hours}h ${minutes % 60}m`;
		} else if (minutes > 0) {
			return `${minutes}m ${seconds % 60}s`;
		}
		return `${seconds}s`;
	}

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

	function deleteEncounter(id: string) {
		history = history.filter(e => e.id !== id);
		localStorage.setItem('bpsr-encounter-history', JSON.stringify(history));
		if (selectedEncounter === id) {
			selectedEncounter = null;
		}
	}

	function clearHistory() {
		if (confirm('Are you sure you want to clear all history?')) {
			history = [];
			selectedEncounter = null;
			localStorage.removeItem('bpsr-encounter-history');
		}
	}

	function exportHistory() {
		const dataStr = JSON.stringify(history, null, 2);
		const dataBlob = new Blob([dataStr], { type: 'application/json' });
		const url = URL.createObjectURL(dataBlob);
		const link = document.createElement('a');
		link.href = url;
		link.download = `bpsr-history-${Date.now()}.json`;
		link.click();
		URL.revokeObjectURL(url);
	}
</script>

<div class="history-container p-6 space-y-6 animate-in fade-in duration-300">
	<div class="header-section">
		<h1 class="text-3xl font-bold mb-2 bg-gradient-to-r from-primary to-chart-2 bg-clip-text text-transparent">📚 Encounter History</h1>
		<p class="text-muted-foreground mb-6">View and manage your past combat encounters</p>
		
		<div class="action-buttons flex flex-wrap gap-3 mb-6">
			<button 
				onclick={exportHistory}
				disabled={history.length === 0}
				class="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-all hover:scale-105 shadow-md disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
			>
				<span>💾</span>
				<span>Export History</span>
			</button>
			<button 
				onclick={clearHistory}
				disabled={history.length === 0}
				class="flex items-center gap-2 px-4 py-2 bg-destructive text-destructive-foreground rounded-lg hover:bg-destructive/90 transition-all hover:scale-105 shadow-md disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
			>
				<span>🗑️</span>
				<span>Clear History</span>
			</button>
		</div>
	</div>

	{#if history.length > 0}
		<div class="history-grid grid grid-cols-1 gap-4">
			{#each history as encounter}
				<div class="encounter-card group bg-card rounded-xl shadow-lg border border-border p-6 hover:shadow-2xl transition-all duration-300 hover:scale-[1.02]">
					<div class="flex items-start justify-between mb-4">
						<div class="flex-1">
							<div class="flex items-center gap-3 mb-2">
								<span class="text-2xl">⚔️</span>
								<h3 class="text-xl font-bold">Encounter #{encounter.id.slice(0, 8)}</h3>
							</div>
							<p class="text-sm text-muted-foreground">{formatDate(encounter.timestamp)}</p>
						</div>
						<button
							onclick={() => deleteEncounter(encounter.id)}
							class="px-3 py-2 text-destructive hover:bg-destructive/10 rounded-lg transition-colors"
							title="Delete encounter"
						>
							<span>🗑️</span>
						</button>
					</div>
					
					<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
						<div class="stat-item">
							<div class="text-xs text-muted-foreground mb-1">Duration</div>
							<div class="text-lg font-bold">{formatDuration(encounter.duration)}</div>
						</div>
						<div class="stat-item">
							<div class="text-xs text-muted-foreground mb-1">Total DPS</div>
							<div class="text-lg font-bold text-chart-1">{formatNumber(encounter.totalDps)}</div>
						</div>
						<div class="stat-item">
							<div class="text-xs text-muted-foreground mb-1">Total Damage</div>
							<div class="text-lg font-bold text-chart-2">{formatNumber(encounter.totalDmg)}</div>
						</div>
						<div class="stat-item">
							<div class="text-xs text-muted-foreground mb-1">Players</div>
							<div class="text-lg font-bold">{encounter.playerCount}</div>
						</div>
					</div>
					
					{#if encounter.topPlayer}
						<div class="mt-4 pt-4 border-t border-border">
							<div class="text-xs text-muted-foreground mb-1">Top Player</div>
							<div class="text-sm font-semibold text-primary">{encounter.topPlayer}</div>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{:else}
		<div class="empty-state text-center py-20 bg-card rounded-xl border border-border shadow-lg">
			<div class="text-7xl mb-4">📚</div>
			<h2 class="text-2xl font-bold mb-2">No History Yet</h2>
			<p class="text-muted-foreground mb-4">Your encounter history will appear here after completing combats</p>
			<div class="text-sm text-muted-foreground bg-accent/30 inline-block px-4 py-2 rounded-lg">
				💡 Tip: History is automatically saved locally
			</div>
		</div>
	{/if}
</div>

<style>
	.history-container {
		max-width: 1400px;
		margin: 0 auto;
	}
	
	.encounter-card {
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}
</style>

