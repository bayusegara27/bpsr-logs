<script lang="ts">
	interface ChangelogEntry {
		version: string;
		date: string;
		changes: {
			type: 'feature' | 'fix' | 'improvement' | 'breaking';
			description: string;
		}[];
	}

	const changelog: ChangelogEntry[] = [
		{
			version: '0.22.1',
			date: '2024-12-19',
			changes: [
				{ type: 'feature', description: 'Added static file server on port 1420 for production web browser access' },
				{ type: 'feature', description: 'Enhanced UI/UX for Statistics, Charts, History, and Changelog pages' },
				{ type: 'feature', description: 'Implemented Encounter History tracking with local storage' },
				{ type: 'improvement', description: 'Improved styling with better animations and transitions' },
				{ type: 'improvement', description: 'Added web-specific settings for browser mode' },
				{ type: 'fix', description: 'Fixed tunnel-based browser access for production builds' }
			]
		},
		{
			version: '0.22.0',
			date: '2024-12-15',
			changes: [
				{ type: 'feature', description: 'Live Statistics Dashboard with real-time updates' },
				{ type: 'feature', description: 'DPS Charts with visual analytics' },
				{ type: 'feature', description: 'Export and share functionality for statistics' },
				{ type: 'improvement', description: 'Enhanced performance for data processing' }
			]
		},
		{
			version: '0.21.0',
			date: '2024-12-10',
			changes: [
				{ type: 'feature', description: 'HTTP API server for web browser access' },
				{ type: 'feature', description: 'Full browser compatibility with automatic environment detection' },
				{ type: 'improvement', description: 'Updated documentation for web access setup' }
			]
		}
	];

	function getTypeIcon(type: string): string {
		switch (type) {
			case 'feature': return '✨';
			case 'fix': return '🐛';
			case 'improvement': return '⚡';
			case 'breaking': return '💥';
			default: return '📝';
		}
	}

	function getTypeColor(type: string): string {
		switch (type) {
			case 'feature': return 'text-chart-2';
			case 'fix': return 'text-destructive';
			case 'improvement': return 'text-chart-1';
			case 'breaking': return 'text-orange-500';
			default: return 'text-muted-foreground';
		}
	}

	function getTypeBg(type: string): string {
		switch (type) {
			case 'feature': return 'bg-chart-2/10 border-chart-2/30';
			case 'fix': return 'bg-destructive/10 border-destructive/30';
			case 'improvement': return 'bg-chart-1/10 border-chart-1/30';
			case 'breaking': return 'bg-orange-500/10 border-orange-500/30';
			default: return 'bg-muted/10 border-muted/30';
		}
	}
</script>

<div class="changelog-container p-6 space-y-6 animate-in fade-in duration-300">
	<div class="header-section">
		<h1 class="text-3xl font-bold mb-2 bg-gradient-to-r from-primary to-chart-2 bg-clip-text text-transparent">📝 Changelog</h1>
		<p class="text-muted-foreground mb-6">Version history and release notes</p>
	</div>

	<div class="changelog-timeline space-y-6">
		{#each changelog as entry, index (entry.version)}
			<div class="version-entry group bg-card rounded-xl shadow-lg border border-border p-6 hover:shadow-2xl transition-all duration-300 animate-in slide-in-from-bottom-{(index + 1) * 2} duration-{500 + index * 100}">
				<div class="flex items-start justify-between mb-4">
					<div>
						<div class="flex items-center gap-3 mb-1">
							<span class="text-2xl">🚀</span>
							<h2 class="text-2xl font-bold">Version {entry.version}</h2>
							{#if index === 0}
								<span class="px-3 py-1 bg-primary text-primary-foreground rounded-full text-xs font-semibold">LATEST</span>
							{/if}
						</div>
						<p class="text-sm text-muted-foreground">{entry.date}</p>
					</div>
				</div>
				
				<div class="changes-list space-y-2 mt-4">
					{#each entry.changes as change, idx (`${entry.version}-${idx}`)}
						<div class="change-item flex items-start gap-3 p-3 rounded-lg hover:bg-accent/30 transition-colors">
							<span class="text-xl">{getTypeIcon(change.type)}</span>
							<div class="flex-1">
								<div class="flex items-center gap-2 mb-1">
									<span class="px-2 py-0.5 text-xs font-semibold rounded-md border {getTypeBg(change.type)} {getTypeColor(change.type)}">
										{change.type.toUpperCase()}
									</span>
								</div>
								<p class="text-sm">{change.description}</p>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>

	<div class="info-section bg-accent/30 border border-accent rounded-xl p-5 backdrop-blur-sm">
		<h3 class="text-lg font-semibold mb-2 flex items-center gap-2">
			<span>ℹ️</span>
			<span>About Updates</span>
		</h3>
		<p class="text-sm text-muted-foreground leading-relaxed">
			BPSR Logs automatically checks for updates on launch. New versions include bug fixes, 
			features, and performance improvements. Check this page regularly to stay informed about the latest changes.
		</p>
	</div>
</div>

<style>
	.changelog-container {
		max-width: 1000px;
		margin: 0 auto;
	}
	
	.version-entry {
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}
	
	.version-entry:hover {
		transform: translateX(4px);
	}
</style>

