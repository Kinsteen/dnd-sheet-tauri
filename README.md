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
	- [ ] Health
		- [ ] current
		- [ ] max
		- [ ] temporary
		- [ ] total
	- [ ] Spells
		- [ ] Spell slots
		- [ ] Current prepared spells
		- [ ] Their effect, damage...
- [ ] Users can edit (and create) their character with ease
	- [ ] Levels
	- [ ] Ability scores
	- [ ] Race
	- [ ] Background
- [ ] Users can switch between characters
- [ ] Users can import and export their characters to someone else
	- [ ] Protobuf
	- [ ] Json if possible, so it's easier to read?
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
			- [ ] Other window that manages homebrew (per homebrew item (class, race, ..)? per campaign?)
				- [ ] Install from file
				- [ ] Disable homebrew (will unload current sheet if it has homebrew from it)
				- [ ] Enable homebrew
				- [ ] uninstall (delete) homebrew
				- [ ] When loading a sheet, will check for used homebrew. If homebrew is detected but not enabled, will prompt to enable
	- [ ] User data should be saved properly when modifying their sheet
- [ ] (way later, maybe never) gRPC communication
	- [ ] Players can share their page live to their DM
	- [ ] Dice rolls between a party?
