import * as d3 from "d3";

function rot(n, x, y, rx, ry) {
  if (ry === 0) {
    if (rx === 1) {
      x[0] = n - 1 - x[0];
      y[0] = n - 1 - y[0];
    }

    // Swap x and y
    let t = x[0];
    x[0] = y[0];
    y[0] = t;
  }
}

function d2xy(n, d, x, y) {
  let rx, ry;
  x[0] = y[0] = 0;
  for (let s = 1; s < n; s *= 2) {
    rx = 1 & (d / 2);
    ry = 1 & (d ^ rx);
    rot(s, x, y, rx, ry);
    x[0] += s * rx;
    y[0] += s * ry;
    d /= 4;
  }
}

function drawHilbertCurve(order) {
  const n = Math.pow(2, order);
  const maxD = n * n;
  const points = [];

  for (let d = 0; d < maxD; d++) {
    let x = [0],
      y = [0];
    d2xy(n, d, x, y);
    points.push([x[0], y[0]]);
  }

  const svg = d3
    .select("#hilbertContainer")
    .append("svg")
    .attr("width", 600)
    .attr("height", 600)
    .call(
      d3
        .zoom()
        .scaleExtent([1, 40])
        .wheelDelta((event) => -event.deltaY * 0.001)
        .on("zoom", (event) => {
          container.attr("transform", event.transform);
        }),
    )
    .append("g");

  const container = svg.append("g");

  const line = d3
    .line()
    .x((d) => d[0] * (600 / n))
    .y((d) => d[1] * (600 / n));

  container
    .append("path")
    .datum(points)
    .attr("d", line)
    .attr("stroke", "black")
    .attr("stroke-width", 0.5)
    .attr("fill", "none");
}

drawHilbertCurve(11); // Change the order here

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
