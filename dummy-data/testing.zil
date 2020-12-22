<ROUTINE EAST-HOUSE (RARG)
	 <COND (<EQUAL? .RARG ,M-LOOK>
		<TELL
"You are behind the white house. A path leads into the forest
to the east. In one corner of the house there is a small window
which is ">
		<COND (<FSET? ,KITCHEN-WINDOW ,OPENBIT>
		       <TELL "open.">)
		      (T <TELL "slightly ajar.">)>
		<CRLF>)>>