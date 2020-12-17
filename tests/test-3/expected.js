function FIND_WEAPON(O) {
  let W = null
  (W = FIRST_q(dot_O))
  if (!dot_W) {
    RFALSE()
  }

  (() => {
    while (true) {
      if ((dot_W == comma_STILETTO || dot_W == comma_AXE || dot_W == comma_SWORD || dot_W == comma_KNIFE || dot_W == comma_RUSTY_KNIFE)) {
        RETURN(dot_W)
      } else if (!(W = NEXT_q(dot_W))) {
        RFALSE()
      }

    }
  })()

}

