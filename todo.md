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
- [x] Fix sprite position
- [x] Allow attack to attack transition without an Idle frame in between
- [x] Incorporate every standing and crouching attack
- [x] Find a way to modify movement based on action data
- [x] Chain attack modifier
- [x] Buffer system
- [x] Fix dash after forward walk
- [x] Fix attack input buffer for chain attacks
- [x] Implement jump states
- [x] Fix dash lockout API
- [x] Fix screen size
- [x] Fix character size
- [x] Add Player 2 (flipped)
- [x] Implement flipping logic
- [x] Make stuff work correctly when flipped
- [x] Collision system
- [x] Reaction system
- [x] Implement blocking
- [x] Implement all hit reaction states
- [x] Make an editor for character data
- [x] Change font
- [x] Make editor work while paused
- [x] Be able to see changes instantly while paused
- [x] Fix Push logic
- [x] Fix ordering of systems that might be adding a frame of input delay
- [x] Fix flip logic and buffer system when flipping
- [x] walls
- [ ] Fix push behavior on a cornered character
- [x] Apply knock-back to attacker when defender is cornered
- [ ] Fix wall knock-back implementation
- [x] Pull character on cross-up hit
- [ ] Fix cross-up hit pull logic, amount and feel
- [ ] Fix not stun when hit on the first frame of the Turn animation
- [ ] Background
- [ ] Camera
- [ ] Proximity normals
- [x] Special moves
- [ ] Special cancels
- [x] Implement cross-cut DPs
- [ ] Implement all knockdown states
- [ ] Separate checking for a button press and a direction press with flipped flag
- [ ] Learn how to do an input and buffer systems with bitwise operations

## Dash should fail

- [x] *4* > *5* > *6* (walk back and forth repeatedly)
- [x] *4...* (walking) ->  *5* > *4*
- [x] *1...* (crouching) ->  *4* > *5* > *4*;
- [x] *5...* (standing) -> *6* > *5* > *2* > *5* > *6*
- [x] *5...* (standing) -> *6* > *5* > *3* > *6*

## Dash should work

- [x] *4...* (walk back) ->  *5* > *4* > *5* > *4*
- [x] *1...* (crouching) ->  *5* > *4* > *5* > *4*
- [x] *5...* (standing) -> *3* > *5* > *6*
- [x] *5...* (standing) -> *1* > *5* > *4*
- [x] *5...* (standing) -> *4* > *6* > *5* > *6*
- [x] *5...* (standing) -> *6* > *3* > *5* > *6*


# Motions
I will most likely need to read them backwards but also have priority between motions

- [ ] *6*... -> *2* > *3* > *6* => DP
- [ ] *6*... -> *4* > *1* > *2* > *3* > *6* => QCF

