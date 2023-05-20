<ROOM MOUNTAINS
      (IN ROOMS)
      (LDESC "The forest thins out, revealing impassable mountains.")
      (DESC "Forest")
      (UP "The mountains are impassable.")
      (NORTH TO FOREST-2)
      (EAST "The mountains are impassable.")
      (SOUTH TO FOREST-2)
      (WEST TO FOREST-2)
      (FLAGS RLANDBIT ONBIT SACREDBIT)
      (GLOBAL TREE WHITE-HOUSE)>

;"0 -> no next, 1 -> success, 2 -> failed move"

<COND (<NOT <0? .WD>>
		<TELL " which will be cured after ">
		<PRINTN
		 <+ <* ,CURE-WAIT <- .WD 1>>
		    <GET <INT I-CURE> ,C-TICK>>>
		<TELL " moves.">)>

<TELL ", especially in light of your condition.">