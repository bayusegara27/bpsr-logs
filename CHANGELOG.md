## v0.22.3 - 2025-12-19

### Changes

- fix: Add package-lock.json and remove it from .gitignore to fix Build & Release workflow (3a75d43)
- Initial plan (bef8c35)

### Installation

Download the appropriate installer for your platform from the assets below.

- **Windows**: `bpsr-logs_0.22.3_x64_en-US.msi`

### Notes

- Auto-update is enabled - the app will update automatically on next launch
- Join our [Discord](https://discord.gg/Tcc54ST5BU) for support


## v0.22.2 - 2025-12-19

### Changes

- Add automated release workflow with auto-versioning and changelog generation (0555850)
- Add port configuration UI and enhanced logging for better troubleshooting (c610ac6)
- Add automatic port fallback for HTTP API and static file servers (86a718e)
- Fix TypeScript errors and add keys to each blocks for lint compliance (6b2c4a8)
- Enhance UI/UX for stats, charts, history, changelog, and add web settings (f8a26d6)
- Add SPA fallback support for client-side routing (5ba706a)
- Update documentation to reflect static file server in production (9b4f6af)
- Add static file server on port 1420 for web browser access (6d8c904)
- Initial plan (1a93eeb)
- Fix Rust compilation errors - wrap Mutex types with Arc for shared state (836388d)
- Initial plan (011711f)
- Fix code review issues: remove duplicate imports and fix API base URL (c8a7654)
- Update frontend to use Universal API layer (f72030b)
- Implement full HTTP API for complete browser functionality (ab52b6f)
- Refine port detection patterns for better accuracy (3ebe22c)
- Improve backward compatibility and portability (5918427)
- Improve helper scripts and documentation based on code review (2e6444e)
- Fix HMR configuration comment for clarity (bd967a5)
- Add helper scripts for easy tunnel setup (bfe6261)
- Clarify web access limitations and use cases (0f1a62b)
- Add comprehensive web access documentation (d054f49)
- Enable web browser access via tunnel - initial configuration (48f413e)
- Initial plan (22569b6)
- Fix: Disable createUpdaterArtifacts to prevent signing error (8895161)
- Initial plan (444c831)
- Update README with new features documentation (4a6ec03)
- Address code review feedback: improve error handling and UX (8168644)
- Fix Svelte 5 compatibility issues - use $state and $derived (a67eefd)
- Add statistics and DPS charts pages with shareable data export (9a99c45)
- Initial plan (d1be06e)
- hotfix: change player account_id and uid when changing clients / characters / accounts fix: npm-check-updates in package.json (06f8c2a)
- fix: make bptimer toggle live switch without app restart fix: html tag warnings fix: blank image for test mode fix: lint and format errors refactor: slight improvement to windivert handle refactor: config files for consistency chore: run formatters chore: update dependencies (ff3e9dc)
- feat: add log cleanup (keeps last 4 sessions) refactor: improve log name and filter refactor: remove log rotation (no longer needed due to filter and 100mb should be plenty) chore: update dependencies and version (19a4467)
- feat: added account_id and uid to bptimer (future use for multi-region support and showing player instead of api report) feat: added local player state and player cache (should help with unknown entries - persistent till app restart) refactor: changed http client used for bptimer to blocking via thread and force rust tls (better timeout) refactor: lowered log count and size and changed too verbose logs to debug! fix: changed skill_uid_to_dps_stats to skill_uid_to_heal_stats if heal_skill chore: ran cargo fmt and cargo update (53e945b)
- fix: removed restart note effect (had issues due to race condition of settings load) and just added a persistent note (919ab31)
- feat: move bptimer logic into bptimer.rs feat: add ENV support locally (dotenvy) and via GH actions for bptimer feat: show indicator when switching bptimer toggle needing an app restart (var only loaded once on app start) fix: allow reporting mobs at 100% hp on first view (will make sure mob attributes are set before reporting) (cf69d30)
- boss only done? thanks @Inngrimsch1, closes #13 (a88fed9)
- v0.21.0 - added magical creature locations for https://bptimer.com/ (f896ded)
- Add monster location for bptimer (bc5e0b5)
- v0.20.0 - fixed some translations - rate limit API call to bptimer - integrations page to opt-out of bptimer - add button to open logs, etc. in settings - add shortcuts to switch between dps and heal tab (fcf6a06)
- add shortcuts for heal/dps tab (398565c)
- add integrations page to opt-out (2f227a4)
- reformat cleanup (5f261c6)
- add buttons to open various folders in the settings (a06ee9a)
- rate limit API calls by hp diff (6f3278d)
- translation fixes (cd4aca3)
- fix tauri dev (67ceccc)
- v0.19.1 - bugfix: window keeps resetting (dc99c6f)
- bugfix: window keeps resetting on load (b67a1e5)
- fix shield knight spec added skill names beside skill id (09889e6)
- v0.19.0 - better translation + skill icons (thanks @warflash from questlog!) - added crowdsourcing world boss (@woheedev)     - https://bptimer.com/     - Join the [Discord](https://discord.gg/Tcc54ST5BU): https://discord.gg/Tcc54ST5BU to report issues     - Still a bit scuffed btw this is super beta - added autostart (fdcf2bc)
- removed a bunch of warnings (66e637a)
- Added App Autostart cleaned up settings too (5f77e7b)
- crowdsource data (05983ae)
- edit readme (c999740)
- get ready for crowdsourcing bosses, magical creatures, etc. (0eedd07)
- cleanup packet logs (d23fce3)
- git add .idea (42ad6a0)
- add better translations + skill icons (ee8b28a)
- v0.18.0 - missing dmg packets no longer missing - UI freezes workaround - windivert fixes (52818af)
- bugfix: UI freeze workaround by refreshing user screen every 5m (3c253d2)
- maybe windivert will behave this time... (ef5f123)
- bugfix: missing damage packets (31aead3)
- v0.17.0 - bufix: heal page was pointing at dps - bufix: settings button now focus the window - implement log rotation and limits - setting to toggle shortened ability score (b0aedbb)
- bugfix: heal page (2fabc5c)
- extracted testing (94958a8)
- bugfix: settings button focus (a0a09fd)
- Add setting to toggle shortened ability score (3c6c3b6)
- implement log rotation (1771634)
- fix tauri + rustrover lagging (9002670)
- v0.16.1 - windivert critical fix (c7ed3b6)
- windivert fixes (afe1116)
- v0.16.0 - Fix settings breaking after every update - Add settings in misc. for UI testing - Added more fine grained options for relative dps vs heal and player vs skills - Added settings to reset the encounter after a certain amount of time has passed - Updated translations - Maybe fixed WinDivert? (2941a3c)
- maybe fixed WinDivert being stuck? (67c1c72)
- Updated translations (f55c504)
- Added settings to reset the encounter after a certain amount of time has passed (b7fb6f9)
- Added more options for relative dps vs heal and player vs skills (d32b63c)
- add UI testing in settings (3feea7f)
- formatting (e6077c3)
- Fix settings breaking every update - all settings will be reset to default (cb3ce2f)
- v0.15.0 - TEMP FIX: both reset buttons refresh the UI too - removed more logging (4b9b4b7)
- TEMP FIX: reset now refreshes the page (68fc2b3)
- remove more logs (511ec67)
- small bugfix for names having leading character (0e1f45f)
- v0.14.0 - Fixed local player data - Added button in settings to dump it to clipboard - Added colors to class icons - Fixed log issue (oopsies) (006db87)
- small bugfix with settings button - it was not opening when its minimized (d74287a)
- add button in settings to dump local player data (efd27ef)
- Fixed your own character not being recognized - fixed SyncContainerData - see: https://github.com/winjwinj/StarResonanceDamageCounter/commits/master/algo/BlueProtobuf_pb.js - TODO: grab it from https://github.com/PotRooms/StarResonanceData/tree/main/proto instead (0f757d3)
- remove more log lines (3e98cff)
- add colors to class icons (86c65c5)
- v0.13.0 - added relative to top dps player (d49ac6b)
- undo insane logging (b09f504)
- readme change to add windivert ignore (b358ca5)
- Added setting to change relative to top player (d8464bf)
- v0.12.0 - TEMP FIX: add hard reset button (this should "fix" the freezing click this instead of opening/closing the meter) - Add transparency effects to Windows 11 - Add shortcuts to reset encounter (and hard reset) (70b3bbf)
- add shortcuts to reset encounter and hard reset (21a8b2f)
- small log cleanup (4032950)
- Add transparency effects (f2d6f17)
- temp fix: add HARD reset button - AI wrote half of this TBH rust async sux (4934169)
- v0.11.0 - hopefully no flag by Windows Defender now because people are mass reporting for no reason - fixed decimal points - gear score now shows ?? instead of 0 if hidden or unknown due to opening meter late - fixed names not showing up if unknown (shows up as "Unknown Name" now) (4b9e207)
- Unknown names show up as "Unknown Name" now (602668e)
- gear score 0 -> ?? instead (8d200a5)
- abbreviated number: 3dp -> 0dp for non k/m/t numbers (efe448b)
- Unify bar rendering logic across all views (a0465e7)
- Make damage bars relative to top player (167024b)
- v0.10.0 - revert class translations since website was wrong apparently (4f5e6b1)
- typo (54952a7)
- Revert "translations for classes changed" (2644bd9)
- Add meter images (1b39378)
- v0.9.0 - Class Style Translations changed - Fix blur for Windows 11 users (who uses this???) (b58967d)
- Windows 11 Blur effect fixes (de3635c)
- translations for classes changed - spec -> style - Moonstrike -> Moonblade - Icicle -> Frostlance - Frostbeam -> Ray - Vanguard -> Overdrive - Skyward -> Aerial - Smite -> Thornlash - Lifebind -> Healing - Earthfort -> Stonewall - Wildpack -> Beastmaster - Recovery -> Bulwark - Shield -> RadiantGuard (f1d1baa)
- v0.8.0 - Added streamer mode (59ec0d9)
- features - Added streamer mode - Screenshot button automatically hides others' name (b1a5d87)
- v0.7.0 Settings page added (05d2340)
- Settings Page - settings uses tauri-store (unofficial plugin) https://github.com/ferreira-tb/tauri-store - tables reworked to use tanstack - table header visibility toggle-able in the settings - shortcuts (f9c90aa)
- v0.6.0 - Add reset + pause/unpause encounter (c7f4991)
- add reset + pause/unpause encounter (800985f)
- v0.5.1 - bugfix with ui not showing on first load - better translations (e97ec74)
- small changes - bugfix with ui not showing on first load - better translations (a1bfb2b)
- v0.5.0 - vastly improved translations - added healing tab - add screenshot to clipboard button - other minor UI tweaks (e.g. tooltips) - subclasses (class specs) - bug fixes (93b0fd6)
- few things - add screenshot to clipboard button - fix reroute if skill breakdown's player uid is missing in backend (e.g. during line change) (143f5bb)
- add healing (3f72dbe)
- bunch of stuff - UI tweaks/refactoring (e.g. hits/s -> HPM, AbbreviatedNumber, tooltips) - Player Skill Window now errors if not found and redirects to main page - Added subclasses (called Class Spec in-game) (ee34a3e)
- Add scripts for cleaning, combining, translating, and finalizing skill names (442198a)
- Update README.md (74edce2)
- v0.4.0 - Add auto updater (a51f706)
- v0.4.0 - Add auto updater (f4865b5)
- Add updater (2900b0b)
- v0.3.0 - UI working - Encounter timer + total dmg + total dps - DPS tab - Skill breakdown (left click to inspect player + right click to exit out) - Heal tab WIP - Lots of feature missing though (e.g. tooltips, screenshot button, settings page) (6b3c937)
- UI working with DPS + skill breakdown still no settings, etc. (b0e8598)
- reset ui (c80a940)
- v0.2.0 - skill breakdowns - crit, lucky, hits (5b787ab)
- added crit, lucky, and hits (681d9d7)
- add configuration for svelte (526dca3)
- skill breakdown kinda works (3ecf82c)
- v0.1.0 barebones meter - dps, damage, total damage, % damage - character info (lvl, ability score, class) (ee77e7a)
- it works but code has degraded greatly (64419a6)
- barebones working meter with specta-typescript (a0801d3)
- barebones working meter with ts-rs (9eb3431)
- tcp reassembler + logging done (22648a6)
- opcodes::Pkt done (d13043c)
- tcp reassembler + logging done (ac34f82)
- tauri init (8e99aff)
- Initial commit (9280a97)

### Installation

Download the appropriate installer for your platform from the assets below.

- **Windows**: `bpsr-logs_0.22.2_x64_en-US.msi`

### Notes

- Auto-update is enabled - the app will update automatically on next launch
- Join our [Discord](https://discord.gg/Tcc54ST5BU) for support


# Changelog

All notable changes to BPSR Logs will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial changelog setup
- Automated release workflow

---

## Previous Releases

For releases prior to automated changelog, please see the [Releases page](https://github.com/winjwinj/bpsr-logs/releases).
