import qpace as qp

# Constructor
if True:
    tf = qp.Timeframe.Days(1)
    tf = qp.Timeframe.Hours(4)
    tf = qp.Timeframe.Minutes(15)
    tf = qp.Timeframe.Unknown()

# From string
if True:
    tf = qp.Timeframe.from_str("1D")
    tf = qp.Timeframe.from_str("4H")
    tf = qp.Timeframe.from_str("15m")
    tf = qp.Timeframe.from_str("?")

# To string
if True:
    text = str(qp.Timeframe.Days(1))
    text = str(qp.Timeframe.Hours(4))
    text = str(qp.Timeframe.Minutes(15))
    text = str(qp.Timeframe.Unknown())
