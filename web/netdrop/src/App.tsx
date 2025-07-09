import { Background } from "./components/common/background";

export function App() {
  return (
    <div className="relative min-h-screen w-full bg-white">
      <Background />

      <div className="relative z-10 flex min-h-screen flex-col items-center justify-center p-8">
        <div className="text-center">
          <h1 className="font-medium text-2xl text-gray-800">
            NetDrop initial impl
          </h1>
        </div>
      </div>
    </div>
  );
}
