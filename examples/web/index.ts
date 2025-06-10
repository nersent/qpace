import * as qp from "qpace/web";

window.onload = async () => {
  await qp.init();
  const root = document.getElementById("root")!;
  root.textContent = `${qp.VERSION}`;
  console.log(qp.Timeframe.Days(1).toString());
};
