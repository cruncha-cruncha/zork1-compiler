function TOUCH_ALL(OBJ) {
  let F = null
  (F = FIRST_4(_2OBJ))
  (() => {
    while (true) {
      if (!_2F) {
        RETURN()
      } else if (true) {
        FSET(_2F, _3TOUCHBIT)
        if (FIRST_4(_2F)) {
          TOUCH_ALL(_2F)
        }

      }

      (F = NEXT_4(_2F))
    }
  })()

}
