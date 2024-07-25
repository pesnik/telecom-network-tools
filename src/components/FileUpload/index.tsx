import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import CardHeader from "@mui/material/CardHeader";
import { useFileContext } from "contexts/FileContext";
import React from "react";
import FileUploadButton from "./FileUploadButton";
import SelectedActualFile from "./FileSelected";

const FileUploader: React.FC = () => {
  const { selectedFile, setSelectedFile } = useFileContext();

  const handleFileSelect = (file: null | string) => {
    setSelectedFile(file);
  };

  return (
    <Card
      sx={{
        minWidth: 275,
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        textAlign: "center",
      }}
    >
      <CardHeader
        title="Transmission Path Dependency Checker"
        sx={{ alignSelf: "center" }}
      />
      <CardContent>
        {selectedFile ? (
          <SelectedActualFile />
        ) : (
          <FileUploadButton onFileSelect={handleFileSelect} />
        )}
      </CardContent>
      <CardActions></CardActions>
    </Card>
  );
};

export default FileUploader;
