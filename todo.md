# TO-DO

- [x] World
- [x] Player input
- [x] Letterbox display
- [x] Physics component
- [x] Character data
- [x] World to screen utilities
- [x] State machine
- [x] Display current State
- [x] Pause and frame advance
- [x] State transition utilities
- [x] Read state from data file
- [x] Animation system
- [ ] Fix sprite position
- [x] Allow attack to attack transition without an Idle frame in between
- [x] Incorporate every standing and crouching attack
- [x] Find a way to modify movement based on action data
- [x] Chain attack modifier
- [x] Buffer system
- [x] Fix dash after forward walk
- [x] Fix attack input buffer for chain attacks
- [ ] Collision system
- [ ] Change font

# Dash should fail
- [x] *4* > *5* > *6* (walk back and forth repeatedly)
- [x] *4...* (walk back) ->  *5* > *4*
- [x] *1...* (crouching) ->  *4* > *5* > *4*;
- [x] *5...* (standing) -> *6* > *5* > *2* > *5* > *6*


# Dash should work
- [x] *4...* (walk back) ->  *5* > *4* > *5* > *4*
- [x] *1...* (crouching) ->  *5* > *4* > *5* > *4*
