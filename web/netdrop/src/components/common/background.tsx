import { useCallback, useEffect, useRef } from "react";

export function Background() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationRef = useRef<number | undefined>(undefined);

  const setupCanvas = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return null;

    const ctx = canvas.getContext("2d");
    if (!ctx) return null;

    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.getBoundingClientRect();

    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;

    ctx.scale(dpr, dpr);

    return {
      ctx,
      width: rect.width,
      height: rect.height,
    };
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    let canvasData = setupCanvas();
    if (!canvasData) return;

    let { ctx, width, height } = canvasData;

    let step = 0;
    let lastTime = 0;
    const targetFPS = 60;
    const frameInterval = 1000 / targetFPS;

    const getParams = (width: number, height: number) => {
      return {
        centerX: width / 2,
        centerY: height / 2,
        spacing: Math.max(width, height, 1000) / 13,
        maxRadius: Math.max(width, height) * 1.2,
      };
    };

    let params = getParams(width, height);

    const drawCircle = (radius: number) => {
      if (radius <= 0 || radius > params.maxRadius) return;

      ctx.beginPath();

      const normalizedRadius = radius / params.maxRadius;
      const baseOpacity = 0.12;
      const opacity = baseOpacity * (1 - normalizedRadius * 0.7);
      const grayValue = Math.round(190 + 25 * (1 - normalizedRadius));

      ctx.strokeStyle = `rgba(${grayValue}, ${grayValue}, ${grayValue}, ${opacity})`;
      ctx.lineWidth = 1.8;
      ctx.arc(params.centerX, params.centerY, radius, 0, 2 * Math.PI);
      ctx.stroke();
    };

    const render = () => {
      ctx.clearRect(0, 0, width, height);

      for (let i = 0; i < 8; i++) {
        const radius = params.spacing * i + (step % params.spacing);
        drawCircle(radius);
      }

      step += 0.7;
    };

    const animate = (currentTime: number) => {
      if (currentTime - lastTime >= frameInterval) {
        render();
        lastTime = currentTime;
      }

      animationRef.current = requestAnimationFrame(animate);
    };

    const observer = new ResizeObserver(() => {
      canvasData = setupCanvas();
      if (canvasData) {
        params = getParams(canvasData.width, canvasData.height);
        ({ width, height, ctx } = canvasData);
      }
    });

    observer.observe(canvas);

    animationRef.current = requestAnimationFrame(animate);

    return () => {
      observer.disconnect();
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [setupCanvas]);

  return (
    <canvas ref={canvasRef} className="absolute inset-0 z-0 h-full w-full" />
  );
}
