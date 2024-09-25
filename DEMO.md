# Demo Game

The demo game is unfinished, but these are all my plans for it:

## Map

Underground has three main areas: maze, treasure, and monster. Monster and cave-spiders can show up anywhere underground if the player isn't carrying light.

## Objects

On player:
cloak, axe, flint, cup, kettle, knife,

In / near cabin:
table, chairs, bed-frame,
door, key, window,
bucket, book, nails,

Others:
boat-frame, tree-hollow (and gem), glass shard

Animals:  
owl, crow, fish, frog, beetle, rabbit, snake, bat, bear,  
spider, cave-spider, spider-web,  
monster,

Forageable:  
sticks, detritus, bullrushes,
logs, sap, river-stones, bones, water,

Foreageable and edible:  
berries, herbs, mushrooms, nuts, roots, ferns,

Craftable:
book-page, soup, tea, fire, straps, rough-board, boiled-sap, charcoal, torch, note, master-key

Below ground:
gold-lump, stone-door, coffin (and cursed-skull, magic-ring), obsidian shard, sword, pick-axe, wire, rocks

## Objectives

enter cabin, light fire, cook meal, find gem, kill monster, write a note,
build a boat, descend waterfall to finish game

## Mechnics

Can get into the cabin by:

- breaking the window (will take damage)
- breaking down the door
- finding the door key

Book in the cabin talks about a crow, a gem, an underground monster, and a boat-frame.

Monster stalks the undergound, will try to kill you on sight. What does it eat? Gem allows you to see in the dark (so does torch, but it only lasts a couple rounds). Gem can be found in the tree-hollow.

Talking to the crow gives you the location of the tree-hollow.
Crow can be found by:

- taking the broken glass outside (will take damage), then waiting a bit (it's shiny)
- killing the owl / fish / frog / rabbit / snake / bear, then waiting a bit
- low random chance

Tree-hollow can also be found by drinking mushroom tea or soup. Drinking mushroom tea / soup also has the side-effect that tools and weapons can never be broken. Talking to most animals, they will tell you a story from their life.

- owl: the heartbreak of it's first love
- fish: why it dropped out of school
- frog: the best meal it ever ate
- rabbit: how lucky it's lucky pebble is
- snake: dreams of flying, swimming, and walking
- bat: terrifying migration in the sunshine
- crow: growing up with it's brothers and sisters

If you come across the bear, it will ask you if you know a random animal story.

- If you don't, it will take away a random animal buff (you're so frightened you forget it). If you haven't heard any animal stories, the bear will attack (but leave once you start fighting back).
- If you do know the random story, the bear will tell you the location of the crow's tree-hollow

If come across the bear and have the gem, the bear won't attack. It will ask what your plans are and if you've built a boat yet, maybe providing pointers on how to go about it.

To talk to / kill the fish, you have to put something edible in the water first, and even then the fish only sticks around for two turns.

Can gain health by eating soup or drinking tea. Hearing an animal's story permanently grants you buffs:

- owl: increases max attack damage of some weapons and tools
- fish: improve chance to high-roll attack from 50% to 75%
- frog: increases max player health (beyond global limit)
- rabbit: decreases enemy attack damage (aka the damage of the weapons they use)
- snake: decreases max enemy health (makes all enemies a little less healthy)
- bat: tells you the secret to open the stone door
- crow: weapons and tools always stay sharp (aka max damage)

Food is important. The food number e = (total-things-eaten - (day x 2)). If, in the morning: e < -5, then you lose 20 health; e < -3, then you lose 10 health; e < 0 then you lose 2 health. If, in the morning: e > 6, then you lose (e + day) health.

Sleep is also important. The sleep number z = (total-sleeps - day), and the nap (sleeps during this day) number is k. If, at the start of night: z < -2, then the player loses 40 health, immediately falls asleep, and dreams nothing; z < -1, then the player loses 20 health, immediately falls asleep, and dreams nothing; z < -1 then the player loses 10 health; z < 0, then the player gets a warning; z > 1, then the player gets a warning; z > 3, then the player loses 20 health. If, at the start of night: k > 5, the player loses 10 health but regains 1 sleep; k > 2, the player regains 1 sleep.

The player may dream at night, but never during naps.

The player can only hold seven things lol.

## Combat

Weapons have DAMAGE, tools have HEALTH. Most of the information in this sectino can be similarly applied to tools. Sharpening (using a wet river-stone) a weapon increases it's damage up to it's MAX-DAMAGE. Using a weapon might decrease it's damage. If damage reaches zero, the weapon is broken, cannot be used, and must be repaired (with sticks and wire, cooked over the fire). Not everything can be repaired.

Actual damage is usually DAMAGE x (1 + ((health - (max-health / 2)) / max-health)) +/- (MAX-DAMAGE / 4), but may change based on the weapon and it's target.

- ((health - (max-health / 2)) / max-health) is going to be a value from -0.5 to 0.5, and it means that the healthier you are, the better you wield a weapon.
- DAMAGE x (1 + above) scales the base damage from 0.5 to 1.5
- (MAX-DAMAGE / 4) means that a deadly weapon is always dangerous: if a weapon has the potential to harm, then it always will, regardless of whether or not the blade is sharp
- all together, it means that the healthier you are, the more damage you can do, but a weapon (and similarly a tool) with DAMAGE less than half of it's MAX-DAMAGE has the potential to hurt you

Negative damage hurts you. You can think of damage as always a high roll or a low roll, never a medium roll. Chance to high roll is 50%. Damage affects you or your opponents HEALTH. You can do more damage than MAX-DAMAGE; this value is more like max base damage. A full moon changes the actual damage calculation to DAMAGE +/- MAX-DAMAGE.

I don't want the monster to one-encounter-KO, so the first encounter it should be surprised, and run away after one hit. After that, it won't run away, and will pursue you unless you're carrying light or you leave the underground.

Obsidian shard is extremely sharp but not strong (has a lot of damage but loses it quickly). Once it breaks, it cannot be repaired.

The gold-lump can be found if you hit enough rocks. It does nothing, but you can't carry it and the boat at the same time. Hitting rocks with anything other than the pick-axe will damage them severely. The pick-axe is just lying in a room in the underground.

The obsidian shard is glinting in the underground lake, but only if you're wearing the magic-ring or have drank mushroom tea / soup. You simply empty the water, then take it. Can empty even if you don't see it glimmering.

There is a sword in the underground. If you have the magic-ring, you can pick up and wield the sword no problem. If not, picking up the sword will result in it immediately disintegrating (0 DAMAGE), unless you've drank mushroom tea / soup or talked to the crow, in which case it will go to 1 DAMAGE. Examining the sword reveals something about this danger.

Both cursed-skull and magic-ring can be found in the coffin. Cursed-skull immediately dulls all your weapons if you pick it up (even if you've talked to the crow). If the player breaks it, they die.

The coffin is in the underground hall. The hall can only be accessed by a stone door.
The stone door can be:

- broken down
- opened with the master-key
- will open for you if you've talked to the bat

## Crafting

Player can only sleep (or nap) if they have their cloak. Fire can be put out by peeing or putting [tea, soup, water] on it. Fire lasts (logs x 8) commands long, or (logs x 4) commands long if outside, or cabin door or window is broken.

Eating bone, mushrooms, or roots raw will result in a mild sickness (-5 health). Other raw food gives you +5 health. Eating soup or tea gives you +20 health. Drinking soup with bone in it, or any tea, increases your max health (up to a global limit).

Soup: put anything edible + water in the kettle or bucket, cook over the fire
Tea: put anything edible + water in the kettle or bucket, cook over the fire, pour into cup
Fire: put [sticks, detritus, bullrushes, sap, book, book-page] and logs in fireplace, then spark flint at fireplace.
Strap: work detritus with bone, or bullrush with bone. Must have 4 or more detritus / bullrush in the area.
Rough-board: work log with axe (makes 4), or hit [chair, table, door, bed-frame]
Boiled-sap: put sap in kettle, cook over fire
Charcoal: after a fire goes out, but before a new fire has started.
Book-page: hit book
Note: work book-page with charcoal
Torch: put both [sap, boiled-sap] and [detritus, bullrushes] onto stick, strike flint at stick
Master-key: work bone with obsidian shard
Boat: work boat-frame with at least 2 straps, at least 10 rough-boards, boiled-sap, and nails

If you strike flint at [sticks, detritus, book, book-page] outside and it hasn't rained in two days, the forest burns down and you die. Otherwise, the fire will last one turn.

In order to rappel down the waterfall: work on pick-axe with straps, then keep in inventory.
