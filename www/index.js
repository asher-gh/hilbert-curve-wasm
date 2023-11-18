import * as d3 from "d3";
import { HilbertCurve } from "hilbert-curve-wasm";
import { memory } from "hilbert-curve-wasm/hilbert_curve_wasm_bg.wasm";

function drawHilbertCurve(order) {
  const n = Math.pow(2, order);
  let hc = HilbertCurve.new(order);
  const flatPoints = new Uint32Array(
    memory.buffer,
    hc.points(),
    hc.points_len(),
  );

  if (flatPoints.length % 2 !== 0) {
    console.error("The length of flatPoints array is not even.");
  } else {
    // numberof points = 2^(2*order), eg. for order=10, there are ~1M points
    const points = [];

    for (let i = 0; i < flatPoints.length; i += 2) {
      points.push([flatPoints[i], flatPoints[i + 1]]);
    }
    // Get viewport dimensions
    const width =
      window.innerWidth ||
      document.documentElement.clientWidth ||
      document.body.clientWidth;
    const height =
      window.innerHeight ||
      document.documentElement.clientHeight ||
      document.body.clientHeight;

    let size = width <= height ? width : height;
    size = size * 0.9;

    const svg = d3
      .select("#hilbertContainer")
      .append("svg")
      .attr("width", size)
      .attr("height", size)
      .call(
        d3
          .zoom()
          .scaleExtent([1, 40])
          .wheelDelta((event) => -event.deltaY * 0.001)
          .on("zoom", (event) => {
            container.attr("transform", event.transform);
          }),
      );

    const container = svg.append("g");

    const line = d3
      .line()
      .x((d) => d[0] * (size / n))
      .y((d) => d[1] * (size / n));

    container
      .append("path")
      .datum(points)
      .attr("d", line)
      .attr("stroke", "black")
      .attr("stroke-width", 0.7)
      .attr("fill", "none");
  }
}

drawHilbertCurve(10); // Change the order here

// ----------------------- disable browser's zoom ----------------------

window.addEventListener(
  "wheel",
  function (event) {
    if (event.ctrlKey === true) {
      event.preventDefault();
    }
  },
  { passive: false },
);

window.addEventListener("keydown", function (event) {
  if (
    event.ctrlKey === true &&
    (event.key === "+" || event.key === "-" || event.key === "0")
  ) {
    event.preventDefault();
  }
});
