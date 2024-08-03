import React from "react";
// import { styled } from "@mui/material/styles";
import Button from "@mui/material/Button";
import CloudUploadIcon from "@mui/icons-material/CloudUpload";
import { open } from "@tauri-apps/api/dialog";

interface FileUploadButtonProps {
  onFileSelect: (file: null | string) => void;
}

const FileUploadButton: React.FC<FileUploadButtonProps> = ({
  onFileSelect,
}) => {
  const handleOpen = async () => {
    const selected = await open({
      directory: false,
      multiple: false,
    });
    const first = selected;
    console.log(first);
    onFileSelect(first as string);
  };

  return (
    <Button
      component="label"
      role={undefined}
      variant="contained"
      tabIndex={-1}
      startIcon={<CloudUploadIcon />}
      onClick={handleOpen}
    >
      Choose file
      {/* <VisuallyHiddenInput type="file" onChange={handleFileChange} /> */}
    </Button>
  );
};

export default FileUploadButton;
