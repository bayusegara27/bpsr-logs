/**
 * Universal API layer for BPSR Logs
 * Automatically detects if running in Tauri or browser context
 * and uses the appropriate communication method
 */

import type { HeaderInfo, PlayersWindow, SkillsWindow, Result } from './bindings';

// Check if we're running in Tauri environment
function isTauriEnvironment(): boolean {
	if (typeof window === 'undefined') {
		return false;
	}
	return '__TAURI_INTERNALS__' in window;
}

// API implementation that works in both Tauri and browser
class UniversalAPI {
	private readonly isTauri: boolean;

	constructor() {
		this.isTauri = isTauriEnvironment();
		if (!this.isTauri) {
			console.log('[BPSR API] Running in browser mode - using HTTP API');
		} else {
			console.log('[BPSR API] Running in Tauri mode - using native invoke');
		}
	}

	private async httpFetch<T>(endpoint: string, options?: RequestInit): Promise<T> {
		const baseUrl = window.location.origin;
		const url = `${baseUrl}/api/${endpoint}`;
		
		try {
			const response = await fetch(url, {
				...options,
				headers: {
					'Content-Type': 'application/json',
					...options?.headers
				}
			});

			if (!response.ok) {
				throw new Error(`HTTP ${response.status}: ${response.statusText}`);
			}

			return await response.json();
		} catch (error) {
			console.error(`[BPSR API] HTTP request failed for ${endpoint}:`, error);
			throw error;
		}
	}

	private async tauriInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
		const { invoke } = await import('@tauri-apps/api/core');
		return invoke<T>(command, args);
	}

	// Header info
	async getHeaderInfo(): Promise<Result<HeaderInfo, string>> {
		if (this.isTauri) {
			try {
				const data = await this.tauriInvoke<HeaderInfo>('get_header_info');
				return { status: 'ok', data };
			} catch (error) {
				return { status: 'error', error: String(error) };
			}
		} else {
			try {
				const data = await this.httpFetch<HeaderInfo>('header-info');
				return { status: 'ok', data };
			} catch (error) {
				return { status: 'error', error: String(error) };
			}
		}
	}

	// DPS Player Windows
	async getDpsPlayerWindow(): Promise<PlayersWindow> {
		if (this.isTauri) {
			return this.tauriInvoke<PlayersWindow>('get_dps_player_window');
		} else {
			return this.httpFetch<PlayersWindow>('dps-player-window');
		}
	}

	async getDpsBossOnlyPlayerWindow(): Promise<PlayersWindow> {
		if (this.isTauri) {
			return this.tauriInvoke<PlayersWindow>('get_dps_boss_only_player_window');
		} else {
			return this.httpFetch<PlayersWindow>('dps-boss-only-player-window');
		}
	}

	async getTestPlayerWindow(): Promise<PlayersWindow> {
		if (this.isTauri) {
			return this.tauriInvoke<PlayersWindow>('get_test_player_window');
		} else {
			return this.httpFetch<PlayersWindow>('test-player-window');
		}
	}

	// Heal Player Window
	async getHealPlayerWindow(): Promise<PlayersWindow> {
		if (this.isTauri) {
			return this.tauriInvoke<PlayersWindow>('get_heal_player_window');
		} else {
			return this.httpFetch<PlayersWindow>('heal-player-window');
		}
	}

	// Skill Windows
	async getDpsSkillWindow(playerUidStr: string): Promise<Result<SkillsWindow, string>> {
		if (this.isTauri) {
			try {
				const data = await this.tauriInvoke<SkillsWindow>('get_dps_skill_window', {
					playerUidStr
				});
				return { status: 'ok', data };
			} catch (error) {
				return { status: 'error', error: String(error) };
			}
		} else {
			try {
				const data = await this.httpFetch<SkillsWindow>(`dps-skill-window/${playerUidStr}`);
				return { status: 'ok', data };
			} catch (error) {
				return { status: 'error', error: String(error) };
			}
		}
	}

	async getDpsBossOnlySkillWindow(playerUidStr: string): Promise<Result<SkillsWindow, string>> {
		if (this.isTauri) {
			try {
				const data = await this.tauriInvoke<SkillsWindow>('get_dps_boss_only_skill_window', {
					playerUidStr
				});
				return { status: 'ok', data };
			} catch (error) {
				return { status: 'error', error: String(error) };
			}
		} else {
			try {
				const data = await this.httpFetch<SkillsWindow>(
					`dps-boss-only-skill-window/${playerUidStr}`
				);
				return { status: 'ok', data };
			} catch (error) {
				return { status: 'error', error: String(error) };
			}
		}
	}

	async getHealSkillWindow(playerUidStr: string): Promise<Result<SkillsWindow, string>> {
		if (this.isTauri) {
			try {
				const data = await this.tauriInvoke<SkillsWindow>('get_heal_skill_window', {
					playerUidStr
				});
				return { status: 'ok', data };
			} catch (error) {
				return { status: 'error', error: String(error) };
			}
		} else {
			try {
				const data = await this.httpFetch<SkillsWindow>(`heal-skill-window/${playerUidStr}`);
				return { status: 'ok', data };
			} catch (error) {
				return { status: 'error', error: String(error) };
			}
		}
	}

	async getTestSkillWindow(playerUid: string): Promise<Result<SkillsWindow, string>> {
		if (this.isTauri) {
			try {
				const data = await this.tauriInvoke<SkillsWindow>('get_test_skill_window', { playerUid });
				return { status: 'ok', data };
			} catch (error) {
				return { status: 'error', error: String(error) };
			}
		} else {
			try {
				const data = await this.httpFetch<SkillsWindow>(`test-skill-window/${playerUid}`);
				return { status: 'ok', data };
			} catch (error) {
				return { status: 'error', error: String(error) };
			}
		}
	}

	// Encounter control
	async resetEncounter(): Promise<void> {
		if (this.isTauri) {
			await this.tauriInvoke<void>('reset_encounter');
		} else {
			await this.httpFetch<void>('reset-encounter', { method: 'POST' });
		}
	}

	async togglePauseEncounter(): Promise<void> {
		if (this.isTauri) {
			await this.tauriInvoke<void>('toggle_pause_encounter');
		} else {
			await this.httpFetch<void>('toggle-pause-encounter', { method: 'POST' });
		}
	}

	async hardReset(): Promise<void> {
		if (this.isTauri) {
			await this.tauriInvoke<void>('hard_reset');
		} else {
			await this.httpFetch<void>('hard-reset', { method: 'POST' });
		}
	}

	// Settings (Tauri-only features - gracefully degrade in browser)
	async enableBlur(): Promise<void> {
		if (this.isTauri) {
			await this.tauriInvoke<void>('enable_blur');
		} else {
			console.warn('[BPSR API] Blur effects are not available in browser mode');
		}
	}

	async disableBlur(): Promise<void> {
		if (this.isTauri) {
			await this.tauriInvoke<void>('disable_blur');
		} else {
			console.warn('[BPSR API] Blur effects are not available in browser mode');
		}
	}

	async copySyncContainerData(): Promise<void> {
		if (this.isTauri) {
			await this.tauriInvoke<void>('copy_sync_container_data');
		} else {
			console.warn('[BPSR API] Clipboard operations are not available in browser mode');
		}
	}

	async setBptimerEnabled(enabled: boolean): Promise<void> {
		if (this.isTauri) {
			await this.tauriInvoke<void>('set_bptimer_enabled', { enabled });
		} else {
			await this.httpFetch<void>('set-bptimer-enabled', {
				method: 'POST',
				body: JSON.stringify({ enabled })
			});
		}
	}

	// Utility
	isRunningInTauri(): boolean {
		return this.isTauri;
	}
}

// Export singleton instance
export const api = new UniversalAPI();

// Export for type checking
export type { UniversalAPI };
