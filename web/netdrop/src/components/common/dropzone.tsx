import { FileText, Upload, X } from "lucide-react";
import { useCallback } from "react";
import { useDropzone } from "react-dropzone";
import { useFileUpload } from "../../contexts/file-upload-context";
import { cn } from "../../lib/utils";

export function Dropzone() {
  const { uploadFiles, uploadedFiles, removeFile, isUploading } =
    useFileUpload();

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

  return (
    <div className="mx-auto w-full max-w-2xl space-y-6">
      {/* Dropzone Area */}
      <div
        {...getRootProps()}
        className={cn(
          "relative cursor-pointer rounded-lg border-2 border-dashed p-12 text-center transition-all duration-200",
          isDragActive
            ? "border-blue-400 bg-blue-50"
            : "border-gray-300 hover:border-gray-400 hover:bg-gray-50",
          isUploading && "pointer-events-none opacity-50",
        )}
      >
        <input {...getInputProps()} />
        <div className="space-y-4">
          <div className="mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-gray-100">
            <Upload className="h-8 w-8 text-gray-400" />
          </div>
          <div>
            <p className="font-medium text-gray-900 text-lg">
              {isDragActive ? "Drop files here" : "Drop files to upload"}
            </p>
            <p className="mt-1 text-gray-500 text-sm">
              or click to select files
            </p>
          </div>
        </div>
      </div>

      {/* Upload Progress */}
      {isUploading && (
        <div className="text-center">
          <div className="inline-flex items-center space-x-2 text-blue-600">
            <div className="h-4 w-4 animate-spin rounded-full border-blue-600 border-b-2"></div>
            <span className="text-sm">Uploading files...</span>
          </div>
        </div>
      )}

      {/* Uploaded Files List */}
      {uploadedFiles.length > 0 && (
        <div className="space-y-3">
          <h3 className="font-medium text-gray-900 text-lg">Uploaded Files</h3>
          <div className="space-y-2">
            {uploadedFiles.map((file) => (
              <div
                key={file.id}
                className="flex items-center justify-between rounded-lg bg-gray-50 p-3"
              >
                <div className="flex items-center space-x-3">
                  <FileText className="h-5 w-5 text-gray-400" />
                  <div>
                    <p className="font-medium text-gray-900 text-sm">
                      {file.name}
                    </p>
                    <p className="text-gray-500 text-xs">
                      {(file.size / 1024 / 1024).toFixed(2)} MB
                    </p>
                  </div>
                </div>
                <button
                  type="button"
                  onClick={() => removeFile(file.id)}
                  className="rounded-full p-1 transition-colors hover:bg-gray-200"
                >
                  <X className="h-4 w-4 text-gray-400" />
                </button>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
