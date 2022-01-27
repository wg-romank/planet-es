import * as fl from "florest";

let started = new Date().valueOf();
let r = fl.Render.new("florest-canvas");

let lastCall = 0;
let cum = 0;

const fps = 60;

const renderLoop = (timestamp) => {
  const delta = timestamp - lastCall;
  lastCall = timestamp;
  cum += delta;

  if (cum > 1000 / fps) {
    r.frame(lastCall / 1000);

    cum = 0;
  }

  requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);