# Map

Underground should have three main areas: maze, treasure / puzzle, and monster. Monster can also show up in the treasure area, but is less likely to after initial two encounters. Have to go through the maze or monster area to get to the treasure. Maze rooms move around confusingly but predictably (based on the moon? player movement? idk).

- maze: 2 tunnels, 1 cavern, 3 hollows, 1 gallery, 2 grottos;
- treasure: 1 underground-lake, 3 underground-lake-shores, 2 passages, 1 cavern, 2 halls, 1 hollow, 1 grave, 1 mine;
- monster: 1 den, 5 caves, 2 passages, 1 cleft;

# Objects

On player:
boots, cloak, axe, flint, bowl, cup, kettle, cutlery, knife,

In / near cabin:
table, chair-1, chair-2, bed-frame fireplace,
door, lock, key, window,
bucket, ladder, book, book-pages, nails, floor-boards,

Animals:  
owl, crow, fish, frog, beetle, rabbit, snake, bat, bear,  
spider, cave-spider, spider-web,  
monster,

Forageable:  
sticks, branches, brush, bracken, reeds, bullrushes, dried-grass,
logs, trees, sap, rocks, river-stones, bones, coffin,  
sand, moss, feathers, water,

Foreageable and edible:  
nuts, berries, seeds, herbs, mushrooms, roots, leaves, bark, ferns,

Foreageable and offensive:
thorns, animal-trap

Craftable:
soup, tea, fire, straps, rough-board, boiled-sap, charcoal, torch,

Findable below ground:
gold-lump, cursed-skull, magic-ring, obsidian shard, sword,
pick-axe, chisel, rope, wire, hammer,

If you break the window:
glass shard

Hidey holes:
tree-hollow (and gem, cloth-scraps, button)

# Objectives

enter cabin, light fire, cook meal, find gem, kill monster
find treasure, find tools, write a note
build boat and descend waterfall to finish game

# Mechnics

Monster stalks the undergound, will try to kill you on sight.
Gem allows you to see in the dark (so does torch, but it only lasts a couple rounds).
Gem can be found in the tree-hollow.

Talking to the crow gives you the location of the tree-hollow.
Crow can be found by:

- taking the broken glass outside (is shiny), then waiting a bit
- killing the owl / fish / frog / rabbit / snake / bear, then waiting a bit
- successfully singing an animal story to the bear
- low random chance

Talking to most animals will let you whisper and sing the story of their life.

- owl: the heartbreak of it's first love
- fish: why it dropped out of school
- frog: the best meal it ever ate
- rabbit: how lucky it's lucky pebble is
- snake: dreams of flying, swimming, and walking
- bat: terrifying migration in the sunshine
- crow: growing up with it's brothers and sisters

If you come across the bear, it will ask you if you know a random animal story.

- If you don't, it will take away a random animal story (you're so frightened you forget it). If you don't know any animal stories at all, the bear will attack (but leave once you start fighting back).
- If you do know the random story, the bear will tell you the location of the crow's tree-hollow
- Bear will still show up but leave you alone if you have the gem

Can gain health by eating soup or drinking tea. You're max health is capped unless...

Whispering stories permanently grants you buffs:

- owl: increases max attack damage
- fish: increases max health by a lot
- frog: increases max health
- rabbit: decreases enemy attack damage
- snake: decreases max enemy health (makes all enemies a little less healthy)
- bat: lets you see in the dark, or, if you have the gem, gives you initiative on all attacks
- crow: weapons and tools can't be broken (aka HEALTH doesn't decrease)

Combat
Sharpening a weapon or tool increases it's damage up to it's MAX-DAMAGE. Hitting anything or using the weapon or tool will decrease it's health. If health reaches zero, the tool/weapon is broken, cannot be used, and must be repaired (with nails, wire, and fire).

Actual damage is DAMAGE +/- (MAX-DAMAGE / 3); it's always a high roll or a low roll, never a medium roll. You can do more damage than MAX-DAMAGE; this value is more like max base damage. Minimum dama is 1 (hit is guaranteed). Damage affects your HEALTH.

I don't want the monster to one-encounter-KO, so the first encounter it should be surprised, and run away after one hit. Second encounter it should attack but let you run away. Third+ encounters it should be hard to run away.

# Config

Would like a bash script to checkout main, cargo run, checkout gh-page, merge main, then push.
