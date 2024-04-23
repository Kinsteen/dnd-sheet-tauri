# D&D Sheet (with Tauri)

This desktop app tries to be a replacement of D&D Beyond.

## Features, done and planned

- [ ] Display information on current character
	- [ ] Name
	- [ ] Classes
	- [ ] Levels
	- [x] Abilities
		- [x] calculated, with the correct modifier
	- [x]  Skills, with correct calculated modifier
	- [x] Saving throws
	- [X] Health
		- [X] current
		- [X] max
		- [ ] temporary
		- [ ] total
		- [X] Can modify directly from the sheet
	- [ ] Spells
		- [ ] Spell slots
		- [ ] Current prepared spells
		- [ ] Their effect, damage...
	- [X] Class counters
		- [X] Name
		- [X] Used
		- [X] Max uses
		- [ ] When resets (short rest/long rest/custom)
		- [X] Can modify directly from the sheet
- [ ] Users can edit (and create) their character with ease
	- [ ] Levels
	- [ ] Ability scores
	- [ ] Race
	- [ ] Background
- [ ] Users can switch between characters
- [ ] Users can import and export their characters to someone else
	- [ ] Protobuf
	- [ ] Json if possible, so it's easier to read?
- [ ] Page to see what happens on short/long rest (health, counters, ...)
- [ ] Users can create homebrew content with Rust code
- [ ] Users can create homebrew content with UI? (Not sure if it's the goal)
- [ ] Save and load information dynamically
	- [x] Using protobuf for data storage
	- [x] Static data (stuff from SRD) is read directly from binary
		- [x] Classes (with subclasses when applicable)
		- [x] Races,
		- [ ] Backgrounds,
		- [x] Skills,
		- [ ] Spells
		- [ ] Should also be able to be read from disk with user content (or paid non-SRD content)
			- [ ] Users can drag and drop homebrew files directly in the app to install new content
			- [ ] Read from protobuf data or JSON (with serde). Check file extension
			- [ ] Other window that manages homebrew (per homebrew item (class, race, ..)? per campaign?)
				- [ ] Install from file
				- [ ] Disable homebrew (will unload current sheet if it has homebrew from it)
				- [ ] Enable homebrew
				- [ ] uninstall (delete) homebrew
				- [ ] When loading a sheet, will check for used homebrew. If homebrew is detected but not enabled, will prompt to enable
	- [ ] User data should be saved properly when modifying their sheet
- [ ] Support UI translations
	- [ ] Use string keys, at least English embedded in, probably French as well
	- [ ] Way to embed translations in Homebrew
	- [ ] Way to add user translations for new languages, or untranslated keys
- [ ] (way later, maybe never) gRPC communication
	- [ ] Players can share their page live to their DM
	- [ ] Dice rolls between a party?
