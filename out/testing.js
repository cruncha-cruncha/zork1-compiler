let GO_NEXT = (TBL) => {
  LET(VAL, null);
  return (() => {
    if (SET(VAL, LKP(comma_HERE, dot_TBL))) {
      return (() => {
        if (NOT(GOTO(dot_VAL))) {
          return 2;
        } else if (T) {
          return 1;
        } else {
          return 0;
        }
      })();
    } else {
      return 0;
    }
  })();
};
