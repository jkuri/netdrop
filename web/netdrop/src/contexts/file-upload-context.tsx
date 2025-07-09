import type React from "react";
import { createContext, useCallback, useContext, useState } from "react";
import { config } from "../config";

export interface UploadedFile {
  id: string;
  name: string;
  size: number;
  type: string;
  uploadedAt: Date;
  url?: string;
}

interface FileUploadContextType {
  uploadedFiles: UploadedFile[];
  isUploading: boolean;
  uploadFiles: (files: File[]) => Promise<void>;
  removeFile: (fileId: string) => void;
  clearFiles: () => void;
}

const FileUploadContext = createContext<FileUploadContextType | undefined>(
  undefined,
);

export function FileUploadProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [uploadedFiles, setUploadedFiles] = useState<UploadedFile[]>([]);
  const [isUploading, setIsUploading] = useState(false);

  const uploadFiles = useCallback(async (files: File[]) => {
    setIsUploading(true);

    try {
      const uploadPromises = files.map(async (file) => {
        const formData = new FormData();
        formData.append("file", file);

        const response = await fetch(`${config.BASE_URL}/api/v1/upload`, {
          method: "POST",
          body: formData,
        });

        if (!response.ok) {
          throw new Error(`Upload failed: ${response.statusText}`);
        }

        const result = await response.json();

        const uploadedFile: UploadedFile = {
          id: crypto.randomUUID(),
          name: file.name,
          size: file.size,
          type: file.type,
          uploadedAt: new Date(),
          url: result.url || result.downloadUrl,
        };

        return uploadedFile;
      });

      const newUploadedFiles = await Promise.all(uploadPromises);

      setUploadedFiles((prev) => [...prev, ...newUploadedFiles]);
    } catch (error) {
      console.error("Upload error:", error);
      // You might want to add error handling/notification here
    } finally {
      setIsUploading(false);
    }
  }, []);

  const removeFile = useCallback((fileId: string) => {
    setUploadedFiles((prev) => prev.filter((file) => file.id !== fileId));
  }, []);

  const clearFiles = useCallback(() => {
    setUploadedFiles([]);
  }, []);

  const value: FileUploadContextType = {
    uploadedFiles,
    isUploading,
    uploadFiles,
    removeFile,
    clearFiles,
  };

  return (
    <FileUploadContext.Provider value={value}>
      {children}
    </FileUploadContext.Provider>
  );
}

export function useFileUpload() {
  const context = useContext(FileUploadContext);
  if (context === undefined) {
    throw new Error("useFileUpload must be used within a FileUploadProvider");
  }
  return context;
}
