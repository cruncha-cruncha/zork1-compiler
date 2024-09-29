# Demo Game

The demo game isn't fully tested, but these are all my plans for it. Spoilers.

## Map

The map is divided into above- and below- ground. Above-ground has forests, a lake, fields, a cliff, and cave entrances. The lake will always have water. Deep forest will always have a tree. Other items spawn somewhat randomly. Below-ground has three main areas: maze, treasure, and den.

## Commands

The main commands are:

- `INVENTORY`
- `LOOK AROUND`
- `GO direction`
- `EXAMINE object`
- `TAKE object`
- `DROP object`
- `EMPTY object`
- `OPEN object`, only used for doors
- `ADD object TO object`
- `HIT object WITH object`
- `WORK object WITH object`
- `SPARK object AT object`
- `TALK TO object`
- `EAT object`
- `SLEEP`

There's also `TIME` and `WEATHER`. If you want a peek under the hood, try `DEBUG`. If you're impatient, try `CHEAT`. You can only interact with objects you're carrying or objects in the room.

## Objects

Above-ground

- On player: cloak, axe, flint, cup, kettle, knife,
- In / near cabin: table, chairs, bed-frame, door, key, window, bucket, book, nails,
- Animals: owl, crow, fish, frog, beetle, rabbit, snake, bear,
- Forageable: stick, detritus, bullrush, log, sap, river-stone, water,
- Foreageable and edible: berries, herbs, mushroom, nuts, root, fern,
- Others: boat-frame, tree-hollow (and gem)

Below-ground

- Animals: cave-spider, child-monster, parent-monster, bat,
- Forageable: bones,
- Others: magic-ring, gold-lump, stone-door, coffin (and cursed-skull), obsidian shard, sword, pick-axe, rocks

Craftable

- book-page, soup, tea, fire, strap, rough-board, boiled-sap, charcoal, torch, note, master-key

## Monster Behaviour

Cave-spiders can show up anywhere underground, but only if there's light in the room (from a lit torch, fire, or the gem). The child monster never leaves it's den, doesn't care about light, and runs away to if the parent monster shows up. The parent monster roams all over the underground, but:

- has to be light in the room for it's first encounter, it won't be lethal, and the monster will run away immediately
- if after first encounter, underground, and light in room, monster might show up but will always run away immediately
- if after first encounter, underground, and not light in room, the monster might show up and will always attack

Both monsters will re-generate health when you're not around. The parent monster's max-health increases after every encounter.

## Mechnics

Can get into the cabin by:

- breaking the window (will take damage)
- breaking down the door
- finding the door key

Book in the cabin talks about a crow, a gem, an underground monster, and a boat-frame.

Several rooms underground have holes which you'll fall into if you're not carrying light or the room isn't lit. The gem emits light, and can be found in the tree-hollow. Talking to the crow gives you the location of the tree-hollow. Crow can be found by:

- breaking the cabin window (shiny glass)
- killing the owl / fish / frog / rabbit / snake / bear, then waiting a bit
- low random chance

Talking to most animals, they will tell you a story from their life.

- owl: the heartbreak of it's first love
- fish: why it dropped out of school
- frog: the best meal it ever ate
- rabbit: how lucky it's lucky pebble is
- snake: dreams of flying, swimming, and walking
- bat: terrifying migration in the sunshine
- crow: growing up with it's brothers and sisters

To talk to the fish, you have to put something edible in the water first, and even then you have to be persistent. If you come across the bear, it will ask you if you know a random animal story.

- If you don't, but tell the truth, the bear will leave you alone
- If you don't, but lie, the bear will attack
- If you do, the bear will tell you the location of the crow's tree-hollow

If come across the bear and have the gem, the bear won't attack..

Invoking an animal's story permanently grants you buffs:

- owl: increases max attack damage of most weapons
- fish: improve chance to high-roll attack from 50% to 75%
- frog: increases max player health (beyond usual limit)
- rabbit: decreases enemy attack damage (aka the damage they can deal to you per turn)
- snake: decreases max enemy health (makes both monsters a little less healthy)
- bat: tells you the secret to open the stone door
- crow: weapons and tools always stay sharp (aka never dull on hit)

Most food gives you health, soup gives you a lot of health, and tea also increases you max-health. The food number e = (total-things-eaten - (day x 2)). If, in the morning: e > 6, then you lose (e + day) health.

Sleep is also important. The sleep number z = (total-sleeps - day), and the nap (sleeps during this day) number is k. If, at the start of night: z < -2, then the player loses 40 health, immediately falls asleep, and dreams nothing; z < -1, then the player loses 20 health, immediately falls asleep, and dreams nothing; z < -1 then the player loses 10 health; z < 0, then the player gets a warning; z > 1, then the player gets a warning; z > 3, then the player loses 20 health. If, at the start of night: k > 5, the player loses 10 health but regains 1 sleep; k > 2, the player regains 1 sleep. Player can only sleep (or nap) if they have their cloak.

The player may dream at night, but never during naps.

The player can only hold seven things lol.

## Combat

Weapons have DAMAGE and MAX-DAMAGE. Sharpening (using a wet river-stone) a weapon increases it's damage up to it's MAX-DAMAGE. Using a weapon might decrease it's damage (to a minimum of '0').

Actual damage is usually DAMAGE x (1 + ((health - (max-health / 2)) / max-health)) +/- (MAX-DAMAGE / 4), but may change based on the weapon and it's target.

- ((health - (max-health / 2)) / max-health) is going to be a value from -0.5 to 0.5, and it means that the healthier you are, the better you wield a weapon.
- DAMAGE x (1 + above) scales the base damage from 0.5 to 1.5
- (MAX-DAMAGE / 4) means that a deadly weapon is always dangerous: if a weapon has the potential to harm, then it always will, regardless of whether or not the blade is sharp
- all together, it means that the healthier you are, the more damage you can do, but a weapon (and similarly a tool) with DAMAGE less than half of it's MAX-DAMAGE has the potential to hurt you

Negative damage hurts you. You can think of damage as always a high roll or a low roll, never a medium roll. Chance to high roll is 50%. Damage affects you or your opponents HEALTH. You can do more damage than MAX-DAMAGE; this value is more like max base damage. A full moon changes the actual damage calculation to DAMAGE +/- MAX-DAMAGE.

Obsidian shard is extremely sharp but not strong (has a lot of damage but loses it quickly). It cannot be sharpened.

There is a sword in the below-ground. If you have the magic-ring, you can pick up and wield the sword no problem. If not magic-ring or cursed-skull, picking up the sword will result in it immediately disintegrating. Examining the sword reveals something about this danger. Cursed-skull immediately dulls all your weapons if you pick it up (even if you've talked to the crow). If the player breaks it, they die.

The treasure area of below-ground has a lake, a cavern, and a crypt. The crypt has the sword, and a coffin with the cursed-skull in it. The crypt can be accessed by opening a stone door. The stone door can be:

- broken down
- opened with the master-key
- will open for you if you've talked to the bat

There's a secret entrance to the crypt behind the waterfall that flows over the cliff, but it's only accessible if you've crafted the right tool (work on pick-axe with a strap).

## Crafting

The gold-lump can be found if you hit enough rocks. It does nothing. The pick-axe is just lying in a room below-ground. It never goes dull.

Fire can be put out by putting water on it. Fire lasts (logs x 8) commands long, or (logs x 4) commands long if outside, or cabin door or window is broken. Fire leaves behind some charcoal. Starting a fire consumes all charcoal.

Eating raw food gives you +10 health. Eating bone, mushrooms, or roots raw only give you +5, and might instead result in a mild sickness (-5 health). Eating soup or tea gives you +40 health. Drinking tea also increases your max health (up to some limit).

Soup: put anything edible + water in the kettle or bucket, cook over the fire
Tea: put anything edible + water in the kettle or bucket, cook over the fire, pour into cup
Fire: spark flint at [sticks, detritus, bullrushes, sap, book, book-page], then add logs
Strap: work detritus with bone
Rough-board: work log with axe (makes 4), or hit [chair, table, door, bed-frame]
Boiled-sap: put sap in kettle, cook over fire
Charcoal: after a fire goes out, but before a new fire has started.
Book-page: hit or empty book
Note: work book-page with charcoal
Torch: put both [sap, boiled-sap] and [detritus, bullrushes] onto a stick, strike flint to light
Master-key: work bone with obsidian shard
Boat: work boat-frame with at least 2 straps, at least 10 rough-boards, boiled-sap, and nails

If there's a fire outside and it hasn't rained in two days, the forest burns down and you die.
