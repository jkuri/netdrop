import {
  CheckCircle,
  Download,
  FileText,
  Loader2,
  Share2,
  Upload,
  X,
  XCircle,
} from "lucide-react";
import { useCallback } from "react";
import { useDropzone } from "react-dropzone";
import { Button } from "@/components/ui/button";
import { useFileUpload } from "@/contexts/file-upload-context";
import { cn } from "@/lib/utils";

export function Dropzone() {
  const { uploadFiles, files, removeFile, isUploading } = useFileUpload();

  const onDrop = useCallback(
    (acceptedFiles: File[]) => {
      uploadFiles(acceptedFiles);
    },
    [uploadFiles],
  );

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    multiple: true,
  });

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
  };

  const _formatSpeed = (bytesPerSecond: number) => {
    if (bytesPerSecond === 0) return "0 B/s";
    const k = 1024;
    const sizes = ["B/s", "KB/s", "MB/s", "GB/s"];
    const i = Math.floor(Math.log(bytesPerSecond) / Math.log(k));
    return `${parseFloat((bytesPerSecond / k ** i).toFixed(1))} ${sizes[i]}`;
  };

  const _formatTime = (seconds: number) => {
    if (seconds === 0 || !Number.isFinite(seconds)) return "calculating...";
    if (seconds < 60) return `${Math.round(seconds)}s`;
    if (seconds < 3600)
      return `${Math.round(seconds / 60)}m ${Math.round(seconds % 60)}s`;
    return `${Math.round(seconds / 3600)}h ${Math.round((seconds % 3600) / 60)}m`;
  };

  const _handleShare = useCallback(async (url: string) => {
    const { toast } = await import("sonner");

    try {
      await navigator.clipboard.writeText(url);
      // Show success toast
      toast.success("Download URL copied to clipboard!", {
        description: "You can now share this link with others",
        duration: 3000,
        action: {
          label: "Dismiss",
          onClick: () => {},
        },
      });
      console.log("Download URL copied to clipboard:", url);
    } catch (error) {
      console.error("Failed to copy to clipboard:", error);
      // Fallback: select the text for manual copying
      const textArea = document.createElement("textarea");
      textArea.value = url;
      textArea.style.position = "fixed";
      textArea.style.left = "-999999px";
      textArea.style.top = "-999999px";
      document.body.appendChild(textArea);
      textArea.focus();
      textArea.select();
      try {
        document.execCommand("copy");
        toast.success("Download URL copied to clipboard!", {
          description: "Used fallback method for older browser",
          duration: 3000,
          action: {
            label: "Dismiss",
            onClick: () => {},
          },
        });
        console.log("Download URL copied to clipboard (fallback):", url);
      } catch (fallbackError) {
        console.error("Fallback copy failed:", fallbackError);
        toast.error("Failed to copy URL", {
          description:
            "Please copy the URL manually from the browser address bar",
          duration: 5000,
          action: {
            label: "Dismiss",
            onClick: () => {},
          },
        });
      }
      document.body.removeChild(textArea);
    }
  }, []);

  const getStatusIcon = (file: { status: string; progress: number }) => {
    switch (file.status) {
      case "uploading":
        if (file.progress === 100) {
          // Finishing state - different color but still spinning
          return <Loader2 className="h-5 w-5 animate-spin text-yellow-400" />;
        }
        return <Loader2 className="h-5 w-5 animate-spin text-blue-500" />;
      case "completed":
        return <CheckCircle className="h-5 w-5 text-green-500" />;
      case "error":
        return <XCircle className="h-5 w-5 text-red-500" />;
      default:
        return <FileText className="h-5 w-5 text-gray-400" />;
    }
  };

  return (
    <div className="mx-auto w-full max-w-3xl">
      <div
        className={cn(
          "relative cursor-pointer rounded-lg border-2 border-dashed bg-white/60 p-12 transition-all duration-200",
          isDragActive
            ? "border-blue-400 bg-blue-50"
            : "border-gray-300 hover:border-gray-400 hover:bg-white",
          isUploading && "pointer-events-none opacity-50",
        )}
      >
        <div className="absolute inset-0 z-10" {...getRootProps()} />
        <input {...getInputProps()} />
        <div className="space-y-4">
          <div className="mx-auto flex h-16 w-16 items-center justify-center rounded-full border">
            <Upload className="h-8 w-8 text-gray-800" />
          </div>
          <div>
            <p className="text-center font-medium text-gray-900 text-lg">
              {isDragActive
                ? "Drop files here"
                : "Drag files here or browse to upload"}
            </p>
            <p className="mt-1 text-center text-gray-500 text-sm">
              Select multiple files to upload
            </p>
          </div>
          {files.length > 0 && (
            <div className="relative z-20 space-y-3">
              {files.map((file) => (
                <div
                  key={file.id}
                  className="rounded-lg border bg-white p-4 shadow-sm"
                >
                  <div className="flex items-center justify-between">
                    <div className="flex min-w-0 flex-1 items-center space-x-3">
                      {getStatusIcon(file)}
                      <div className="min-w-0 flex-1">
                        <p className="truncate font-medium text-gray-900 text-sm">
                          {file.name}
                        </p>
                        <div className="flex items-center space-x-2 text-gray-500 text-xs">
                          <span>{formatFileSize(file.size)}</span>
                          {file.status === "uploading" && (
                            <>
                              <span>•</span>
                              {file.progress === 100 ? (
                                <span>Finishing...</span>
                              ) : (
                                <>
                                  <span>{file.progress}% Uploading...</span>
                                  {file.uploadSpeed && file.uploadSpeed > 0 && (
                                    <>
                                      <span>•</span>
                                      <span>
                                        {_formatSpeed(file.uploadSpeed)}
                                      </span>
                                    </>
                                  )}
                                  {file.timeRemaining &&
                                    file.timeRemaining > 0 && (
                                      <>
                                        <span>•</span>
                                        <span>
                                          ETA {_formatTime(file.timeRemaining)}
                                        </span>
                                      </>
                                    )}
                                </>
                              )}
                            </>
                          )}
                          {file.status === "completed" && (
                            <>
                              <span>•</span>
                              <span className="text-green-600">Done</span>
                            </>
                          )}
                          {file.status === "error" && file.error && (
                            <>
                              <span>•</span>
                              <span className="text-red-600">{file.error}</span>
                            </>
                          )}
                        </div>
                      </div>
                    </div>
                    <div className="flex items-center space-x-2">
                      {file.status === "completed" && file.url && (
                        <>
                          <Button
                            variant="outline"
                            size="sm"
                            onClick={() => _handleShare(file.url!)}
                          >
                            <Share2 className="h-4 w-4" />
                          </Button>
                          <Button variant="outline" size="sm" asChild>
                            <a
                              href={file.url}
                              target="_blank"
                              rel="noopener noreferrer"
                              className="inline-flex items-center space-x-1"
                            >
                              <Download className="h-4 w-4" />
                            </a>
                          </Button>
                        </>
                      )}
                      <button
                        type="button"
                        onClick={() => removeFile(file.id)}
                        className="rounded-full p-1 transition-colors hover:bg-gray-200"
                        disabled={file.status === "uploading"}
                      >
                        <X className="h-4 w-4 text-gray-400" />
                      </button>
                    </div>
                  </div>

                  {/* Progress Bar */}
                  {file.status === "uploading" && (
                    <div className="mt-3">
                      <div className="h-2 w-full rounded-full bg-gray-200">
                        <div
                          className={cn(
                            "h-2 rounded-full transition-all duration-300",
                            file.progress === 100
                              ? "animate-pulse bg-yellow-400" // Finishing state
                              : "bg-blue-600", // Normal uploading
                          )}
                          style={{ width: `${file.progress}%` }}
                        />
                      </div>
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
