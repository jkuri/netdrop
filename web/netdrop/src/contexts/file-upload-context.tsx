import type React from "react";
import { createContext, useCallback, useContext, useState } from "react";
import { config } from "../config";

export type FileStatus = "pending" | "uploading" | "completed" | "error";

export interface UploadFile {
  id: string;
  name: string;
  size: number;
  type: string;
  status: FileStatus;
  progress: number;
  uploadedBytes?: number;
  uploadSpeed?: number; // bytes per second
  timeRemaining?: number; // seconds
  startTime?: number;
  uploadedAt?: Date;
  url?: string;
  error?: string;
}

interface FileUploadContextType {
  files: UploadFile[];
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
  const [_files, setFiles] = useState<UploadFile[]>([]);
  const [isUploading, setIsUploading] = useState(false);

  const uploadFileWithProgress = useCallback(
    (uploadFile: UploadFile, originalFile: File): Promise<void> => {
      return new Promise((resolve, reject) => {
        const xhr = new XMLHttpRequest();
        const formData = new FormData();
        formData.append("file", originalFile);

        const startTime = Date.now();

        // Update status to uploading
        setFiles((prev) =>
          prev.map((f) =>
            f.id === uploadFile.id
              ? {
                  ...f,
                  status: "uploading" as const,
                  progress: 0,
                  startTime,
                  uploadedBytes: 0,
                  uploadSpeed: 0,
                  timeRemaining: 0,
                }
              : f,
          ),
        );

        xhr.upload.addEventListener("progress", (event) => {
          if (event.lengthComputable) {
            const currentTime = Date.now();

            // Calculate overall average speed
            const totalTime = (currentTime - startTime) / 1000;
            const averageSpeed = totalTime > 0 ? event.loaded / totalTime : 0;

            // Calculate time remaining
            const remainingBytes = event.total - event.loaded;
            const timeRemaining =
              averageSpeed > 0 ? remainingBytes / averageSpeed : 0;

            const progress = Math.round((event.loaded / event.total) * 100);

            setFiles((prev) =>
              prev.map((f) => {
                // Only update if file is still uploading (prevent race conditions)
                if (f.id === uploadFile.id && f.status === "uploading") {
                  return {
                    ...f,
                    progress,
                    uploadedBytes: event.loaded,
                    uploadSpeed: averageSpeed,
                    timeRemaining: Math.max(0, timeRemaining),
                  };
                }
                return f;
              }),
            );
          }
        });

        xhr.addEventListener("load", () => {
          if (xhr.status >= 200 && xhr.status < 300) {
            try {
              const result = JSON.parse(xhr.responseText);
              console.log("Upload response:", result);

              // Construct download URL from file_hash
              let downloadUrl: string | undefined;
              if (result.success && result.file_hash) {
                downloadUrl = `${config.BASE_URL}/download/${result.file_hash}`;
                console.log("Generated download URL:", downloadUrl);
              } else {
                console.warn(
                  "Upload response missing file_hash or success=false:",
                  result,
                );
              }

              setFiles((prev) =>
                prev.map((f) =>
                  f.id === uploadFile.id
                    ? {
                        ...f,
                        status: "completed" as const,
                        progress: 100,
                        uploadedAt: new Date(),
                        url: downloadUrl,
                        // Clear upload-related fields
                        uploadSpeed: undefined,
                        timeRemaining: undefined,
                        uploadedBytes: f.size, // Set to full file size
                      }
                    : f,
                ),
              );
              resolve();
            } catch (_error) {
              reject(new Error("Failed to parse response"));
            }
          } else {
            reject(new Error(`Upload failed: ${xhr.statusText}`));
          }
        });

        xhr.addEventListener("error", () => {
          reject(new Error("Network error occurred"));
        });

        xhr.addEventListener("abort", () => {
          reject(new Error("Upload was aborted"));
        });

        xhr.open("POST", `${config.BASE_URL}/api/v1/upload`);
        xhr.send(formData);
      });
    },
    [config.BASE_URL],
  );

  const uploadFiles = useCallback(async (filesToUpload: File[]) => {
    setIsUploading(true);

    // Add files to state with pending status
    const newFiles: UploadFile[] = filesToUpload.map((file) => ({
      id: crypto.randomUUID(),
      name: file.name,
      size: file.size,
      type: file.type,
      status: "pending" as const,
      progress: 0,
    }));

    setFiles((prev) => [...prev, ...newFiles]);

    // Upload files one by one to show individual progress
    for (const uploadFile of newFiles) {
      const originalFile = filesToUpload.find(
        (f) => f.name === uploadFile.name && f.size === uploadFile.size,
      );
      if (!originalFile) continue;

      try {
        await uploadFileWithProgress(uploadFile, originalFile);
      } catch (error) {
        console.error("Upload error:", error);
        // Update file as error
        setFiles((prev) =>
          prev.map((f) =>
            f.id === uploadFile.id
              ? {
                  ...f,
                  status: "error" as const,
                  error:
                    error instanceof Error ? error.message : "Upload failed",
                }
              : f,
          ),
        );
      }
    }

    setIsUploading(false);
  }, []);

  const removeFile = useCallback((fileId: string) => {
    setFiles((prev) => prev.filter((file) => file.id !== fileId));
  }, []);

  const clearFiles = useCallback(() => {
    setFiles([]);
  }, []);

  const value: FileUploadContextType = {
    files: _files,
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
