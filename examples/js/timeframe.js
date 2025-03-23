import * as qp from "qpace";

// Constructor
{
  let tf = qp.Timeframe.days(1);
  tf = qp.Timeframe.hours(4);
  tf = qp.Timeframe.minutes(15);
  tf = qp.Timeframe.unknown(30);
}

// From string
{
  let tf = qp.Timeframe.fromString("1D");
  tf = qp.Timeframe.fromString("4H");
  tf = qp.Timeframe.fromString("15m");
  tf = qp.Timeframe.fromString("?");
}

// To string
{
  let text = qp.Timeframe.days(1).toString();
  text = qp.Timeframe.hours(4).toString();
  text = qp.Timeframe.minutes(15).toString();
  text = qp.Timeframe.unknown().toString();
}
