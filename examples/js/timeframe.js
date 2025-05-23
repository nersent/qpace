import * as qp from "qpace";

// Constructor
{
  const tf = qp.Timeframe.days(1);
  const tf = qp.Timeframe.hours(4);
  const tf = qp.Timeframe.minutes(15);
  const tf = qp.Timeframe.unknown(30);
}

// From string
{
  const tf = qp.Timeframe.fromString("1D");
  const tf = qp.Timeframe.fromString("4H");
  const tf = qp.Timeframe.fromString("15m");
  const tf = qp.Timeframe.fromString("?");
}

// To string
{
  const text = qp.Timeframe.days(1).toString();
  const text = qp.Timeframe.hours(4).toString();
  const text = qp.Timeframe.minutes(15).toString();
  const text = qp.Timeframe.unknown().toString();
}
