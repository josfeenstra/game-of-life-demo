// import { Universe } from "wasm-game-of-life";

async function bootstrap() {
  const wasm = await import("../pkg/index.js");

  const pre = document.getElementById("game-of-life-canvas");
  const universe = wasm.Universe.new(64,64);

  const renderLoop = () => {
      pre.textContent = universe.render();
      universe.update();
      requestAnimationFrame(renderLoop);
    };
    requestAnimationFrame(renderLoop);
}
bootstrap();



