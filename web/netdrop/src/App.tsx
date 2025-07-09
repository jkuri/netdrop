import { Toaster } from "sonner";
import { Background } from "./components/common/background";
import { Dropzone } from "./components/common/dropzone";

export function App() {
  return (
    <div className="relative min-h-screen w-full bg-white">
      <Background />

      <div className="relative z-10 flex min-h-screen flex-col items-center justify-center p-8">
        <Dropzone />
      </div>

      <Toaster position="top-right" richColors />
    </div>
  );
}
