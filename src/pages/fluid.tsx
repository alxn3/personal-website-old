import { useEffect, useRef } from 'react';

const FluidSimulation = () => {
  const canvasRef = useRef();

  useEffect(() => {
    import('wasm/pkg').then((module) => {
      const canvas: HTMLCanvasElement = canvasRef.current;
      const gl = canvas.getContext('webgl', { antialias: true });

      gl.enable(gl.BLEND);
      gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

      const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames
      let lastDrawTime = -1; // milliseconds

      let requestId;

      module.HELLO()
      const render = () => {
        requestId = requestAnimationFrame(render);
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPS_THROTTLE) {
          lastDrawTime = currTime;

          if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
            canvas.height = window.innerHeight;
            // canvas.style.height = window.innerHeight;

            canvas.width = window.innerWidth;
            // canvas.style.width = window.innerWidth;

            gl.viewport(0, 0, window.innerWidth, window.innerHeight);
        }
        }
      };
      render();

      return () => {
        cancelAnimationFrame(requestId);
      };
    });
  }, []);
  return (
    <div>
      <canvas ref={canvasRef} />
    </div>
  );
};

export default FluidSimulation;
