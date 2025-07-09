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
    canvas.style.width = `${rect.width}px`;
    canvas.style.height = `${rect.height}px`;

    ctx.scale(dpr, dpr);

    return {
      ctx,
      width: rect.width,
      height: rect.height,
    };
  }, []);

  useEffect(() => {
    const canvasData = setupCanvas();
    if (!canvasData) return;

    const { ctx, width, height } = canvasData;

    let step = 0;
    let lastTime = 0;
    const targetFPS = 60;
    const frameInterval = 1000 / targetFPS;

    // Animation parameters
    const getParams = () => {
      return {
        centerX: width / 2,
        centerY: height / 2,
        spacing: Math.max(width, height, 1000) / 13,
        maxRadius: Math.max(width, height) * 1.2,
      };
    };

    let params = getParams();

    const drawCircle = (radius: number) => {
      if (radius <= 0 || radius > params.maxRadius) return;

      ctx.beginPath();

      const normalizedRadius = radius / params.maxRadius;
      const baseOpacity = 0.12;
      const opacity = baseOpacity * (1 - normalizedRadius * 0.7);
      const grayValue = Math.round(190 + 20 * (1 - normalizedRadius));

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

    let resizeTimeout: NodeJS.Timeout;
    const handleResize = () => {
      clearTimeout(resizeTimeout);
      resizeTimeout = setTimeout(() => {
        const newCanvasData = setupCanvas();
        if (newCanvasData) {
          params = getParams();
        }
      }, 150);
    };

    window.addEventListener("resize", handleResize);

    animationRef.current = requestAnimationFrame(animate);

    return () => {
      window.removeEventListener("resize", handleResize);

      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
      clearTimeout(resizeTimeout);
    };
  }, [setupCanvas]);

  return (
    <canvas ref={canvasRef} className="absolute inset-0 z-0 h-full w-full" />
  );
}
