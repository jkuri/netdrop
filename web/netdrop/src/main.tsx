import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import { App } from "./App.tsx";
import { FileUploadProvider } from "./contexts/file-upload-context";

createRoot(document.getElementById("root") as HTMLDivElement).render(
  <StrictMode>
    <FileUploadProvider>
      <App />
    </FileUploadProvider>
  </StrictMode>,
);
