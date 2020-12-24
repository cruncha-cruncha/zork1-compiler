function WINNING_q(V) {
  let VS = null
  let PS = null
  (VS = GETP(dot_V, comma_Pq_STRENGTH))
  (PS = (dot_VS - FIGHT_STRENGTH()))
  if (G_q(dot_PS, 3)) {
    PROB(90)
  } else if (G_q(dot_PS, 0)) {
    PROB(75)
  } else if (isZero(dot_PS)) {
    PROB(50)
  } else if (G_q(dot_VS, 1)) {
    PROB(25)
  } else if (true) {
    PROB(10)
  }

}

