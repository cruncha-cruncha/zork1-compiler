<ROUTINE GO-NEXT (TBL "AUX" VAL)
	 <COND (<SET VAL <LKP ,HERE .TBL>>
		<COND (<NOT <GOTO .VAL>> 2)
			  (<FIRST? ROUGH> <RFALSE>)
		      (T 1)>)>>