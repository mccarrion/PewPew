# PewPew
A Rust game about appreciating laser beams.

### Roadmap for this Game
This is the current roadmap for this repo. Bullet points represent general order
that development will move along, but may change slightly as the project moves
forward.

* Draw a square <- This repo is here
* Connect the square to a keyboard
* Draw multiple squares
* Make player square blue and all other squares red
* Make red squares populate randomly inside 2D space
* Connect AI to red squares to follow blue square
* Make blue square shoot LASER BEAMS and connect direction of LASER BEAMS
to a mouse
* Have laser beams destroy red squares on contact with them 
* Have red squares destroy player square on contact with player square

At this point most of the core game will be complete. The next steps will have
to deal with overlaying 16-bit pixel art, score tracking, and level tracking.

### Current State of the Game
The only thing this repo does is try to draw a square before the compiler
panics...

### Game Engine
This game will use the ggez engine for development.

### Goal of this Repo
The end goal would be to create a 16 bit arena zombie shooter.
